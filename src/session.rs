use crate::error::server_error;
use crate::rebuilder::Rebuilder;
use crate::session_cookie::SessionCookie;
use beatrice::reexport::safina_executor;
use beatrice::{Event, EventSender, Response};
use core::fmt::{Debug, Formatter};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError, Weak};

pub fn session_not_found() -> Response {
    Response::text(400, "SESSION_NOT_FOUND")
}

// TODO: Clean shutdown.

pub struct Inner {
    weak_session: Weak<Session>,
    keys_fn: Box<dyn Send + Sync + Fn(&Rebuilder) -> Result<HashSet<String>, Box<dyn Error>>>,
    value_fn: Box<dyn Send + Sync + Fn(&str, &Rebuilder) -> Result<Value, Box<dyn Error>>>,
    keys_rebuilder: Rebuilder,
    value_rebuilders: HashMap<String, Rebuilder>,
    sender: EventSender,
}
impl Inner {
    // TODO: Pass &Arc<Session> and remove `weak_session`.
    pub fn build_keys(&mut self) -> Result<Vec<(String, Value)>, Box<dyn Error>> {
        let mut updates = Vec::new();
        let mut keys = (*self.keys_fn)(&self.keys_rebuilder)?;
        // Remove keys.
        self.value_rebuilders.retain(|key, _rebuilder| {
            if keys.contains(key) {
                true
            } else {
                updates.push((key.to_string(), Value::Null));
                false
            }
        });
        // Add keys.
        keys.retain(|key| !self.value_rebuilders.contains_key(key));
        for key in keys {
            let rebuilder = Rebuilder::Value(self.weak_session.clone(), key.clone());
            self.value_rebuilders.insert(key.clone(), rebuilder.clone());
            let value = (*self.value_fn)(&key, &rebuilder)?;
            updates.push((key, value));
        }
        Ok(updates)
    }

    pub fn build_and_send_keys(&mut self) -> Result<(), Box<dyn Error>> {
        let updates = self.build_keys()?;
        let json_updates = Value::Array(
            updates
                .into_iter()
                .map(|(key, value)| json!({ key: value }))
                .collect(),
        );
        self.sender.send(Event::Message(json_updates.to_string()));
        Ok(())
    }

    pub fn build_value(&mut self, key: &str) -> Result<Value, Box<dyn Error>> {
        let rebuilder = self
            .value_rebuilders
            .get(key)
            .ok_or_else(|| format!("no rebuilder found for key {:?}", key))?;
        let value = (self.value_fn)(key, rebuilder)?;
        Ok(json!({ key: value }))
    }

    pub fn build_and_send_value(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        let diff = self.build_value(key)?;
        self.sender
            .send(Event::Message(serde_json::to_string(&diff).unwrap()));
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PendingUpdate {
    Keys,
    Value(String),
}

pub struct Session {
    pub cookie: SessionCookie,
    pub inner: Mutex<Option<Inner>>,
    pub rpc_updates: Mutex<HashSet<PendingUpdate>>,
    pub scheduled_updates: Mutex<HashSet<PendingUpdate>>,
}
impl Session {
    pub fn new<PathsFn, PagesFn>(
        cookie: SessionCookie,
        keys_fn: PathsFn,
        value_fn: PagesFn,
    ) -> (Arc<Self>, Response)
    where
        PathsFn: 'static
            + Send
            + Sync
            + Fn(&Rebuilder) -> Result<HashSet<String>, Box<dyn std::error::Error>>,
        PagesFn: 'static
            + Send
            + Sync
            + Fn(&str, &Rebuilder) -> Result<Value, Box<dyn std::error::Error>>,
    {
        let (sender, response) = Response::event_stream();
        let response = response.with_set_cookie(cookie.to_cookie());
        let session = Arc::new(Self {
            cookie,
            inner: Mutex::new(None),
            rpc_updates: Mutex::new(HashSet::new()),
            scheduled_updates: Mutex::new(HashSet::new()),
        });
        let inner = Inner {
            weak_session: Arc::downgrade(&session),
            keys_fn: Box::new(keys_fn),
            value_fn: Box::new(value_fn),
            keys_rebuilder: Rebuilder::Keys(Arc::downgrade(&session)),
            value_rebuilders: HashMap::new(),
            sender,
        };
        *(session.inner.lock().unwrap()) = Some(inner);
        session.schedule_rebuild_keys(None);
        (session, response)
    }

    pub fn lock_inner(&self) -> MutexGuard<Option<Inner>> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn schedule_rebuild_keys(self: &Arc<Self>, rpc_session: Option<&Arc<Session>>) {
        if let Some(rpc_session) = rpc_session {
            if Arc::ptr_eq(self, rpc_session) {
                self.rpc_updates.lock().unwrap().insert(PendingUpdate::Keys);
                return;
            }
        }
        let arc_session = self.clone();
        safina_executor::schedule_blocking(move || {
            arc_session
                .lock_inner()
                .as_mut()
                .unwrap()
                .build_and_send_keys()
                .unwrap();
        });
    }

    pub fn schedule_rebuild_value(
        self: &Arc<Self>,
        key: impl AsRef<str>,
        rpc_session: Option<&Arc<Session>>,
    ) {
        let key = key.as_ref().to_string();
        if let Some(rpc_session) = rpc_session {
            if Arc::ptr_eq(self, rpc_session) {
                self.rpc_updates
                    .lock()
                    .unwrap()
                    .insert(PendingUpdate::Value(key));
                return;
            }
        }
        let arc_session = self.clone();
        safina_executor::schedule_blocking(move || {
            arc_session
                .lock_inner()
                .as_mut()
                .unwrap()
                .build_and_send_value(&key)
                .unwrap();
        });
    }

    pub fn response(&self) -> Result<Response, Response> {
        let mut pending_updates = HashSet::new();
        std::mem::swap(&mut *self.rpc_updates.lock().unwrap(), &mut pending_updates);
        dbg!(&pending_updates);
        let mut inner_guard = self.lock_inner();
        let mut json_updates = Vec::new();
        if pending_updates.remove(&PendingUpdate::Keys) {
            let keys_and_values = inner_guard
                .as_mut()
                .unwrap()
                .build_keys()
                .map_err(|e| server_error(format!("error building keys: {}", e)))?;
            for (key, value) in keys_and_values {
                json_updates.push(json!({ &key: value }));
                pending_updates.remove(&PendingUpdate::Value(key));
            }
        }
        for pending_update in pending_updates {
            match pending_update {
                PendingUpdate::Keys => unreachable!(),
                PendingUpdate::Value(key) => {
                    let value = inner_guard
                        .as_mut()
                        .unwrap()
                        .build_value(&key)
                        .map_err(|e| {
                            server_error(format!("error building key {:?}: {}", key, e))
                        })?;
                    json_updates.push(json!({ key: value }));
                }
            }
        }
        dbg!(&json_updates);
        Ok(Response::json(200, Value::Array(json_updates)).unwrap())
    }
}
impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.cookie == other.cookie
    }
}
impl Eq for Session {}
impl Debug for Session {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut rpc_updates: Vec<PendingUpdate> =
            self.rpc_updates.lock().unwrap().iter().cloned().collect();
        rpc_updates.sort();
        let mut scheduled_updates: Vec<PendingUpdate> = self
            .scheduled_updates
            .lock()
            .unwrap()
            .iter()
            .cloned()
            .collect();
        scheduled_updates.sort();
        let mut keys: Vec<String> = self
            .lock_inner()
            .as_ref()
            .unwrap()
            .value_rebuilders
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

// pub struct Sessions {
//     rng: Mutex<ChaCha<20>>,
//     sessions: RwLock<HashMap<SessionId, Arc<Session>>>,
// }
// impl Sessions {
//     pub fn new() -> Self {
//         Self {
//             rng: Mutex::new(ChaCha::new()),
//             sessions: RwLock::new(HashMap::new()),
//         }
//     }
//
//     pub fn begin<PathsFn, PagesFn>(
//         &self,
//         _req: &Request,
//         paths_fn: PathsFn,
//         pages_fn: PagesFn,
//     ) -> Result<Response, Response>
//     where
//         PathsFn: 'static
//             + Send
//             + Sync
//             + Fn(&Rebuilder) -> Result<HashSet<String>, Box<dyn std::error::Error>>,
//         PagesFn: 'static
//             + Send
//             + Sync
//             + Fn(&str, &Rebuilder) -> Result<Value, Box<dyn std::error::Error>>,
//     {
//         // TODO: Use existing session, if it exists.
//         let (sender, response) = Response::event_stream();
//         let (session_id, session_token) = {
//             let mut rng_guard = self.rng.lock().unwrap();
//             (
//                 SessionId::new(&mut *rng_guard),
//                 SessionToken::new(&mut *rng_guard),
//             )
//         };
//         let session = Session::new(session_token, sender, paths_fn, pages_fn);
//         session.schedule_rebuild_keys(None);
//         self.sessions.write().unwrap().insert(session_id, session);
//         Ok(response.with_set_cookie(Cookie::new(
//             SESSION_COOKIE_NAME,
//             AsciiString::try_from(format!("{}-{}", session_id.0, session_token.0)).unwrap(),
//         )))
//     }
//
//     pub fn get(&self, req: &Request) -> Result<Arc<Session>, Response> {
//         // dbg!("Session::get");
//         let session_cookie = req
//             .cookies
//             .get(SESSION_COOKIE_NAME)
//             .ok_or_else(session_not_found)?;
//         // dbg!(&session_cookie);
//         let parts: Vec<&str> = session_cookie.split('-').collect();
//         if parts.len() != 2 {
//             return Err(client_error(format!(
//                 "malformed {:?} cookie: {}",
//                 SESSION_COOKIE_NAME,
//                 escape_and_elide(session_cookie.as_bytes(), 100)
//             )));
//         }
//         let session_id = SessionId::try_from(parts[0]).map_err(|e| {
//             client_error(format!(
//                 "error processing {:?} cookie: {}",
//                 SESSION_COOKIE_NAME, e
//             ))
//         })?;
//         let session_token = SessionToken::try_from(parts[1]).map_err(|e| {
//             client_error(format!(
//                 "error processing {:?} cookie: {}",
//                 SESSION_COOKIE_NAME, e
//             ))
//         })?;
//         // dbg!(session_id, session_token);
//         let session = self
//             .sessions
//             .read()
//             .unwrap()
//             .get(&session_id)
//             .ok_or_else(session_not_found)?
//             .clone();
//         // dbg!(&session);
//         if session.token == session_token {
//             Ok(session)
//         } else {
//             Err(session_not_found())
//         }
//     }
// }
