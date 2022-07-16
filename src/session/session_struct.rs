use crate::data::{Context, Rebuilder};
use crate::error::server_error;
use crate::session::{KeySet, SessionCookie, SessionId};
use core::fmt::{Debug, Formatter};
use serde_json::{json, Value};
use servlin::reexport::safina_executor::Executor;
use servlin::{Event, EventSender, Response};
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PendingUpdate {
    KeySet,
    Key(String),
}

#[allow(clippy::module_name_repetitions)]
pub struct SessionStateGuard<'x, T>(MutexGuard<'x, T>);
impl<'x, T> Deref for SessionStateGuard<'x, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
impl<'x, T> DerefMut for SessionStateGuard<'x, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct InnerSession<T> {
    pub key_set: KeySet<T>,
    pub rpc_updates: HashSet<PendingUpdate>,
    pub sender: EventSender,
}

pub struct Session<T> {
    pub executor: Arc<Executor>,
    pub cookie: SessionCookie,
    #[allow(clippy::type_complexity)]
    pub key_set_fn: Box<
        dyn 'static
            + Send
            + Sync
            + Fn(Rebuilder<T>) -> Result<KeySet<T>, Box<dyn std::error::Error>>,
    >,
    pub scheduled_updates: Mutex<HashSet<PendingUpdate>>,
    pub state: Mutex<T>,
    pub inner: Mutex<InnerSession<T>>,
}
impl<T: 'static + Send + Sync> Session<T> {
    pub fn new<F>(executor: &Arc<Executor>, key_set_fn: F, state: T) -> Arc<Self>
    where
        F: 'static
            + Send
            + Sync
            + Fn(Rebuilder<T>) -> Result<KeySet<T>, Box<dyn std::error::Error>>,
    {
        Arc::new(Self {
            executor: executor.clone(),
            key_set_fn: Box::new(key_set_fn),
            cookie: SessionCookie::new_random(),
            scheduled_updates: Mutex::new(HashSet::new()),
            state: Mutex::new(state),
            inner: Mutex::new(InnerSession {
                key_set: KeySet::new(),
                rpc_updates: HashSet::new(),
                sender: EventSender::unconnected(),
            }),
        })
    }

    pub fn id(&self) -> SessionId {
        self.cookie.id()
    }

    pub fn rpc_context(&self) -> Context {
        Context::Rpc(self.id())
    }

    pub fn lock_inner(&self) -> MutexGuard<InnerSession<T>> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn lock_scheduled_updates(&self) -> MutexGuard<HashSet<PendingUpdate>> {
        self.scheduled_updates
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    #[must_use]
    pub fn state(&self) -> SessionStateGuard<'_, T> {
        SessionStateGuard(self.state.lock().unwrap_or_else(PoisonError::into_inner))
    }

    /// # Errors
    /// Returns an error when it cannot start the stream.
    pub fn stream(self: &Arc<Self>) -> Result<Response, Response> {
        self.lock_scheduled_updates().clear();
        let mut inner_guard = self.lock_inner();
        inner_guard.key_set = KeySet::new();
        let (sender, response) = Response::event_stream();
        inner_guard.sender = sender;
        drop(inner_guard);
        // TODO: Send the client an opaque version ID
        //       and skip rebuilding all if it matches.
        self.rebuild_key_set(&Context::Empty);
        Ok(response
            .with_set_cookie(self.cookie.to_cookie())
            .with_no_store())
    }

    /// # Errors
    /// Returns an error when we fail to build the new key set or fail to build the value for a key.
    pub fn build_key_set(
        self: &Arc<Self>,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        let rebuilder = Rebuilder::Keys(Arc::downgrade(self));
        let mut inner_guard = self.lock_inner();
        let result = (*self.key_set_fn)(rebuilder);
        let mut new_key_set = result?;
        let mut diff = HashMap::new();
        // Removed keys.
        for key in inner_guard.key_set.key_to_value_fn.keys() {
            if !new_key_set.key_to_value_fn.contains_key(key) {
                diff.insert(key.to_string(), Value::Null);
            }
        }
        // Added keys.
        for (key, value_fn) in &new_key_set.key_to_value_fn {
            if !inner_guard.key_set.key_to_value_fn.contains_key(key) {
                let rebuilder = Rebuilder::Value(Arc::downgrade(self), key.to_string());
                let value = (*value_fn)(rebuilder)?;
                diff.insert(key.to_string(), value);
            }
        }
        std::mem::swap(&mut inner_guard.key_set, &mut new_key_set);
        Ok(diff)
    }

    /// # Errors
    /// Returns an error when we fail to build the new key set or fail to build the value for a key.
    #[allow(clippy::missing_panics_doc)]
    pub fn build_key_set_and_send(self: &Arc<Self>) -> Result<(), Box<dyn std::error::Error>> {
        let diff = self.build_key_set()?;
        let update = json!({ "pages": diff });
        let json_string = serde_json::to_string(&update).unwrap();
        //dbg!(&json_string);
        self.lock_inner().sender.send(Event::Message(json_string));
        Ok(())
    }

    /// # Errors
    /// Returns an error when we build the value for the key.
    pub fn build_value(self: &Arc<Self>, key: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let rebuilder = Rebuilder::Value(Arc::downgrade(self), key.to_string());
        let inner_guard = self.lock_inner();
        let value_fn = inner_guard
            .key_set
            .key_to_value_fn
            .get(key)
            .ok_or_else(|| format!("key {:?} not found", key))?;
        (*value_fn)(rebuilder)
    }

    /// # Errors
    /// Returns an error when we fail to build the value for the key.
    pub fn build_value_and_send(
        self: &Arc<Self>,
        key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let value = self.build_value(key)?;
        let json_obj = json!({"pages": { key: value }});
        let json_string = json_obj.to_string();
        //dbg!(&json_string);
        let mut inner = self.lock_inner();
        if inner.sender.is_connected() {
            inner.sender.send(Event::Message(json_string));
        } else {
            inner
                .rpc_updates
                .insert(PendingUpdate::Key(key.to_string()));
        }
        Ok(())
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn rebuild_key_set(self: &Arc<Self>, ctx: &Context) {
        if &Context::Rpc(self.id()) == ctx {
            self.lock_inner().rpc_updates.insert(PendingUpdate::KeySet);
            return;
        }
        let self_clone = self.clone();
        self.executor.schedule_blocking(move || {
            self_clone.build_key_set_and_send().unwrap();
        });
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn rebuild_value(self: &Arc<Self>, key: impl AsRef<str>, ctx: &Context) {
        let key = key.as_ref().to_string();
        if &Context::Rpc(self.id()) == ctx {
            self.lock_inner()
                .rpc_updates
                .insert(PendingUpdate::Key(key));
            return;
        }
        let self_clone = self.clone();
        self.executor.schedule_blocking(move || {
            self_clone.build_value_and_send(&key).unwrap();
        });
    }

    /// # Errors
    /// Returns an error when we fail to build the new key set or fail to build the value for a key.
    #[allow(clippy::missing_panics_doc)]
    pub fn rpc_response(self: &Arc<Self>) -> Result<Response, Response> {
        let mut pending_updates = HashSet::new();
        std::mem::swap(&mut self.lock_inner().rpc_updates, &mut pending_updates);
        //dbg!(&pending_updates);
        let mut diff = if pending_updates.remove(&PendingUpdate::KeySet) {
            self.build_key_set()
                .map_err(|e| server_error(format!("error building keys: {}", e)))?
        } else {
            HashMap::new()
        };
        for pending_update in pending_updates {
            let key = match pending_update {
                PendingUpdate::KeySet => unreachable!(),
                PendingUpdate::Key(key) => key,
            };
            if diff.contains_key(&key) {
                // Skip deleted keys.
                continue;
            }
            let value = self
                .build_value(&key)
                .map_err(|e| server_error(format!("error building key {:?}: {}", key, e)))?;
            diff.insert(key, value);
        }
        //dbg!(&diff);
        Ok(Response::json(200, json!({ "pages": diff }))
            .unwrap()
            .with_no_store())
    }

    /// # Errors
    /// Returns an error when it fails building keys.
    pub fn poll(self: &Arc<Self>) -> Result<Response, Response> {
        // TODO: Send the client an opaque version ID
        //       and skip rebuilding all if it matches.
        self.rebuild_key_set(&Context::Rpc(self.id()));
        let response = self.rpc_response()?;
        Ok(response.with_set_cookie(self.cookie.to_cookie()))
    }
}
impl<T> PartialEq for Session<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cookie == other.cookie
    }
}
impl<T> Eq for Session<T> {}
impl<T: 'static + Send + Sync> Debug for Session<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut rpc_updates: Vec<PendingUpdate> =
            self.lock_inner().rpc_updates.iter().cloned().collect();
        rpc_updates.sort();
        let mut scheduled_updates: Vec<PendingUpdate> =
            self.lock_scheduled_updates().iter().cloned().collect();
        scheduled_updates.sort();
        let mut keys: Vec<String> = self
            .lock_inner()
            .key_set
            .key_to_value_fn
            .keys()
            .cloned()
            .collect();
        keys.sort();
        write!(
            f,
            "Session{{rpc_updates={:?}, scheduled_updates={:?}, keys={:?}}}",
            rpc_updates, scheduled_updates, keys
        )
    }
}
