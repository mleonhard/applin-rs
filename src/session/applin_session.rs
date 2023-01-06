use crate::data::{Context, Rebuilder};
use crate::error::server_error;
use crate::session::{PageMap, SessionCookie, SessionId};
use core::fmt::{Debug, Formatter};
use serde_json::{json, Value};
use servlin::reexport::safina_executor::Executor;
use servlin::{Event, EventSender, Response};
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError, Weak};
use std::time::SystemTime;

fn epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

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
        &self.0
    }
}
impl<'x, T> DerefMut for SessionStateGuard<'x, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct InnerSession<T> {
    pub page_map: PageMap<T>,
    pub rpc_updates: HashSet<PendingUpdate>,
    pub sender: EventSender,
}

pub struct ApplinSession<T> {
    pub executor: Weak<Executor>,
    pub cookie: SessionCookie,
    #[allow(clippy::type_complexity)]
    pub page_map_fn: Box<
        dyn 'static
            + Send
            + Sync
            + Fn(Rebuilder<T>) -> Result<PageMap<T>, Box<dyn std::error::Error>>,
    >,
    pub last_contact_epoch_seconds: AtomicU64,
    pub scheduled_updates: Mutex<HashSet<PendingUpdate>>,
    pub state: Mutex<T>,
    pub inner: Mutex<InnerSession<T>>,
}
impl<T: 'static + Send + Sync> ApplinSession<T> {
    pub fn new<F>(executor: Weak<Executor>, page_map_fn: F, state: T) -> Arc<Self>
    where
        F: 'static
            + Send
            + Sync
            + Fn(Rebuilder<T>) -> Result<PageMap<T>, Box<dyn std::error::Error>>,
    {
        Arc::new(Self {
            executor,
            cookie: SessionCookie::new_random(),
            page_map_fn: Box::new(page_map_fn),
            last_contact_epoch_seconds: AtomicU64::new(epoch_seconds()),
            scheduled_updates: Mutex::new(HashSet::new()),
            state: Mutex::new(state),
            inner: Mutex::new(InnerSession {
                page_map: PageMap::new(),
                rpc_updates: HashSet::new(),
                sender: EventSender::unconnected(),
            }),
        })
    }

    pub fn id(&self) -> SessionId {
        self.cookie.id()
    }

    pub fn is_fresh(&self) -> bool {
        epoch_seconds() - self.last_contact_epoch_seconds.load(Acquire) < 120
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
        self.last_contact_epoch_seconds
            .store(epoch_seconds(), Release);
        let (sender, response) = Response::event_stream();
        {
            let mut inner_guard = self.lock_inner();
            inner_guard.page_map = PageMap::new();
            inner_guard.sender = sender;
        }
        // TODO: Send the client an opaque version ID
        //       and skip rebuilding all if it matches.
        self.rebuild_page_map(&Context::Empty);
        Ok(response
            .with_set_cookie(self.cookie.to_cookie())
            .with_no_store())
    }

    /// # Errors
    /// Returns an error when we fail to build the new key set or fail to build the value for a key.
    pub fn build_page_map(
        self: &Arc<Self>,
    ) -> Result<serde_json::Map<String, Value>, Box<dyn std::error::Error>> {
        let rebuilder = Rebuilder::PageMap(Arc::downgrade(self));
        let mut inner_guard = self.lock_inner();
        let result = (*self.page_map_fn)(rebuilder);
        let mut new_page_map = result?;
        let mut diff = serde_json::Map::new();
        // Removed keys.
        for key in inner_guard.page_map.keys() {
            if !new_page_map.contains_key(key) {
                diff.insert(key.to_string(), Value::Null);
            }
        }
        // Added keys.
        for (key, value_fn) in new_page_map.iter() {
            if !inner_guard.page_map.contains_key(key) {
                let rebuilder = Rebuilder::Page(Arc::downgrade(self), key.to_string());
                let value = (*value_fn)(rebuilder)?;
                diff.insert(key.to_string(), value);
            }
        }
        std::mem::swap(&mut inner_guard.page_map, &mut new_page_map);
        Ok(diff)
    }

    /// # Errors
    /// Returns an error when we fail to build the new key set or fail to build the value for a key.
    #[allow(clippy::missing_panics_doc)]
    pub fn build_page_map_and_send(self: &Arc<Self>) -> Result<(), Box<dyn std::error::Error>> {
        let diff = self.build_page_map()?;
        let update = json!({ "pages": diff });
        let json_string = serde_json::to_string(&update).unwrap();
        //dbg!(&json_string);
        self.lock_inner().sender.send(Event::Message(json_string));
        Ok(())
    }

    /// # Errors
    /// Returns an error when we build the value for the key.
    pub fn build_value(self: &Arc<Self>, key: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let rebuilder = Rebuilder::Page(Arc::downgrade(self), key.to_string());
        let inner_guard = self.lock_inner();
        let value_fn = inner_guard
            .page_map
            .get(key)
            .ok_or_else(|| format!("key {:?} not found", key))?;
        (*value_fn)(rebuilder)
        // TODO: Warn if page has multiple widgets using the same var name.
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
    pub fn rebuild_page_map(self: &Arc<Self>, ctx: &Context) {
        if &Context::Rpc(self.id()) == ctx || !self.lock_inner().sender.is_connected() {
            self.lock_inner().rpc_updates.insert(PendingUpdate::KeySet);
        } else {
            let self_clone = self.clone();
            if let Some(executor) = self.executor.upgrade() {
                executor.schedule_blocking(move || {
                    self_clone.build_page_map_and_send().unwrap();
                });
            }
        }
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
        if let Some(executor) = self.executor.upgrade() {
            executor.schedule_blocking(move || {
                self_clone.build_value_and_send(&key).unwrap();
            });
        }
    }

    /// # Errors
    /// Returns an error when we fail to build the new key set or fail to build the value for a key.
    #[allow(clippy::missing_panics_doc)]
    pub fn rpc_response(self: &Arc<Self>) -> Result<Response, Response> {
        self.rpc_response_with_vars(Value::Null)
    }

    /// # Errors
    /// Returns an error when we fail to build the new key set or fail to build the value for a key.
    #[allow(clippy::missing_panics_doc)]
    pub fn rpc_response_with_vars<V: serde::Serialize>(
        self: &Arc<Self>,
        vars: V,
    ) -> Result<Response, Response> {
        let mut pending_updates = HashSet::new();
        {
            self.last_contact_epoch_seconds
                .store(epoch_seconds(), Release);
            std::mem::swap(&mut self.lock_inner().rpc_updates, &mut pending_updates);
        }
        //dbg!(&pending_updates);
        let mut diff = if pending_updates.remove(&PendingUpdate::KeySet) {
            self.build_page_map()
                .map_err(|e| server_error(format!("error building keys: {}", e)))?
        } else {
            serde_json::Map::new()
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
        let mut obj = serde_json::Map::new();
        if !diff.is_empty() {
            obj.insert("pages".to_string(), diff.into());
        }
        let vars = serde_json::value::to_value(vars).unwrap();
        if vars != Value::Null {
            obj.insert("vars".to_string(), vars);
        }
        Ok(Response::json(200, Value::Object(obj))
            .unwrap()
            .with_no_store())
    }

    /// # Errors
    /// Returns an error when it fails building keys.
    pub fn poll(self: &Arc<Self>) -> Result<Response, Response> {
        // TODO: Send the client an opaque version ID
        //       and skip rebuilding all if it matches.
        self.rebuild_page_map(&Context::Rpc(self.id()));
        let response = self.rpc_response()?;
        Ok(response.with_set_cookie(self.cookie.to_cookie()))
    }
}
impl<T> PartialEq for ApplinSession<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cookie == other.cookie
    }
}
impl<T> Eq for ApplinSession<T> {}
impl<T: 'static + Send + Sync> Debug for ApplinSession<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut rpc_updates: Vec<PendingUpdate> =
            self.lock_inner().rpc_updates.iter().cloned().collect();
        rpc_updates.sort();
        let mut scheduled_updates: Vec<PendingUpdate> =
            self.lock_scheduled_updates().iter().cloned().collect();
        scheduled_updates.sort();
        let mut keys: Vec<String> = self.lock_inner().page_map.keys().cloned().collect();
        keys.sort();
        write!(
            f,
            "Session{{rpc_updates={:?}, scheduled_updates={:?}, keys={:?}}}",
            rpc_updates, scheduled_updates, keys
        )
    }
}
