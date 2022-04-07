use crate::app::{App, AppFn};
use crate::error::server_error;
use crate::rebuilder::Rebuilder;
use crate::session_cookie::SessionCookie;
use beatrice::reexport::safina_executor;
use beatrice::{Event, EventSender, Response};
use core::fmt::{Debug, Formatter};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

pub fn session_not_found() -> Response {
    Response::text(400, "SESSION_NOT_FOUND")
}

// TODO: Clean shutdown.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PendingUpdate {
    Keys,
    Value(String),
}

pub struct Session {
    pub app_fn: Box<AppFn>,
    pub app: Mutex<App>,
    pub cookie: SessionCookie,
    pub rpc_updates: Mutex<HashSet<PendingUpdate>>,
    pub scheduled_updates: Mutex<HashSet<PendingUpdate>>,
    pub sender: Mutex<EventSender>,
}
impl Session {
    pub fn new<F>(app_fn: F) -> (Arc<Self>, Response)
    where
        F: 'static + Send + Sync + Fn(Rebuilder) -> Result<App, Box<dyn std::error::Error>>,
    {
        let cookie = SessionCookie::new_random();
        let (sender, response) = Response::event_stream();
        let response = response.with_set_cookie(cookie.to_cookie());
        let session = Arc::new(Self {
            app_fn: Box::new(app_fn),
            app: Mutex::new(App::new()),
            cookie,
            rpc_updates: Mutex::new(HashSet::new()),
            scheduled_updates: Mutex::new(HashSet::new()),
            sender: Mutex::new(sender),
        });
        (session, response)
    }

    pub fn lock_app(&self) -> MutexGuard<App> {
        self.app.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn lock_sender(&self) -> MutexGuard<EventSender> {
        self.sender.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn build_keys(self: &Arc<Self>) -> Result<HashMap<String, Value>, Box<dyn Error>> {
        let mut app_guard = self.lock_app();
        let mut new_app = (*self.app_fn)(Rebuilder::Keys(Arc::downgrade(self)))?;
        let mut diff = HashMap::new();
        // Removed keys.
        for key in app_guard.key_to_value_fn.keys() {
            if !new_app.key_to_value_fn.contains_key(key) {
                diff.insert(key.to_string(), Value::Null);
            }
        }
        // Added keys.
        for (key, value_fn) in new_app.key_to_value_fn.iter() {
            if !app_guard.key_to_value_fn.contains_key(key) {
                let rebuilder = Rebuilder::Value(Arc::downgrade(self), key.to_string());
                let value = (*value_fn)(rebuilder)?;
                diff.insert(key.to_string(), value);
            }
        }
        std::mem::swap(&mut *app_guard, &mut new_app);
        Ok(diff)
    }

    pub fn build_and_send_keys(self: &Arc<Self>) -> Result<(), Box<dyn Error>> {
        let diff = self.build_keys()?;
        let json_string = serde_json::to_string(&diff).unwrap();
        self.lock_sender().send(Event::Message(json_string));
        Ok(())
    }

    pub fn build_value(self: &Arc<Self>, key: &str) -> Result<Value, Box<dyn Error>> {
        let app_guard = self.lock_app();
        let value_fn = app_guard
            .key_to_value_fn
            .get(key)
            .ok_or_else(|| format!("key {:?} not found", key))?;
        let rebuilder = Rebuilder::Value(Arc::downgrade(self), key.to_string());
        (*value_fn)(rebuilder)
    }

    pub fn build_and_send_value(self: &Arc<Self>, key: &str) -> Result<(), Box<dyn Error>> {
        let value = self.build_value(key)?;
        let json_obj = json!({ key: value });
        let json_string = json_obj.to_string();
        self.lock_sender().send(Event::Message(json_string));
        Ok(())
    }

    pub fn schedule_rebuild_keys(self: &Arc<Self>, rpc_session: Option<&Arc<Session>>) {
        if let Some(rpc_session) = rpc_session {
            if Arc::ptr_eq(self, rpc_session) {
                self.rpc_updates.lock().unwrap().insert(PendingUpdate::Keys);
                return;
            }
        }
        let self_clone = self.clone();
        safina_executor::schedule_blocking(move || {
            self_clone.build_and_send_keys().unwrap();
            // TODO: Disconnect on error or panic.  Also below.
            self_clone.lock_sender().disconnect();
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
        let self_clone = self.clone();
        safina_executor::schedule_blocking(move || {
            self_clone.build_and_send_value(&key).unwrap();
        });
    }

    pub fn response(self: &Arc<Self>) -> Result<Response, Response> {
        let mut pending_updates = HashSet::new();
        std::mem::swap(&mut *self.rpc_updates.lock().unwrap(), &mut pending_updates);
        dbg!(&pending_updates);
        let mut diff = if pending_updates.remove(&PendingUpdate::Keys) {
            self.build_keys()
                .map_err(|e| server_error(format!("error building keys: {}", e)))?
        } else {
            HashMap::new()
        };
        for pending_update in pending_updates {
            let key = match pending_update {
                PendingUpdate::Keys => unreachable!(),
                PendingUpdate::Value(key) => key,
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
        dbg!(&diff);
        Ok(Response::json(200, diff).unwrap())
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
        let mut keys: Vec<String> = self.lock_app().key_to_value_fn.keys().cloned().collect();
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
