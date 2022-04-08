use crate::context::Context;
use crate::key_set::KeySet;
use crate::session::Session;
use crate::session_cookie::SessionCookie;
use crate::session_id::SessionId;
use beatrice::reexport::safina_executor::Executor;
use beatrice::{Request, Response};
use std::collections::HashMap;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[must_use]
pub fn session_not_found() -> Response {
    Response::text(400, "SESSION_NOT_FOUND")
}

pub struct SessionSet<T> {
    pub executor: Arc<Executor>,
    // TODO: Remove disconnected clients from the set after a delay.
    // TODO: Send keepalives.
    pub set: Arc<RwLock<HashMap<SessionId, Arc<Session<T>>>>>,
}
impl<T: 'static + Send + Sync> SessionSet<T> {
    #[must_use]
    pub fn new(executor: &Arc<Executor>) -> Self {
        Self {
            executor: executor.clone(),
            set: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn read_lock(&self) -> RwLockReadGuard<HashMap<SessionId, Arc<Session<T>>>> {
        self.set.read().unwrap_or_else(PoisonError::into_inner)
    }

    fn write_lock(&self) -> RwLockWriteGuard<HashMap<SessionId, Arc<Session<T>>>> {
        self.set.write().unwrap_or_else(PoisonError::into_inner)
    }

    /// # Errors
    /// Returns an error when the request has the session cookie but we fail to parse it.
    pub fn get_opt(&self, req: &Request) -> Result<Option<Arc<Session<T>>>, Response> {
        if let Some(cookie) = SessionCookie::from_req_option(req)? {
            if let Some(session) = self.read_lock().get(&cookie.id()).cloned() {
                if cookie == session.cookie {
                    return Ok(Some(session));
                }
            }
        }
        Ok(None)
    }

    /// # Errors
    /// Returns an error when:
    /// - the request has no session cookie
    /// - we fail to parse the session cookie
    /// - the session is not found
    pub fn get(&self, req: &Request) -> Result<Arc<Session<T>>, Response> {
        self.get_opt(req)?.ok_or_else(session_not_found)
    }

    /// # Errors
    /// Returns an error when the request has the session cookie but we fail to parse it.
    pub fn resume_opt(&self, req: &Request) -> Result<Option<Response>, Response> {
        if let Some(session) = self.get_opt(req)? {
            Ok(Some(session.resume()))
        } else {
            Ok(None)
        }
    }

    pub fn new_session<F>(&self, key_set_fn: F, value: T) -> (Arc<Session<T>>, Response)
    where
        F: 'static + Send + Sync + Fn(&Context<T>) -> Result<KeySet<T>, Box<dyn std::error::Error>>,
    {
        let (session, response) = Session::new(&self.executor, key_set_fn, value);
        self.write_lock()
            .insert(session.cookie.id(), session.clone());
        (session, response)
    }

    /// # Errors
    /// Returns an error when the request has the session cookie but we fail to parse it.
    pub fn resume_or_new<F>(
        &self,
        req: &Request,
        key_set_fn: F,
        session_state_fn: impl FnOnce() -> T,
    ) -> Result<(Arc<Session<T>>, Response), Response>
    where
        F: 'static + Send + Sync + Fn(&Context<T>) -> Result<KeySet<T>, Box<dyn std::error::Error>>,
    {
        if let Some(session) = self.get_opt(req)? {
            let response = session.resume();
            Ok((session, response))
        } else {
            let session_state = session_state_fn();
            Ok(self.new_session(key_set_fn, session_state))
        }
    }
}
