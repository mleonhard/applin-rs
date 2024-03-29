use crate::data::Rebuilder;
use crate::session::{ApplinSession, PageMap, SessionCookie, SessionId};
use servlin::reexport::safina_executor::Executor;
use servlin::{Request, Response};
use std::collections::HashMap;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};

#[must_use]
pub fn session_not_found() -> Response {
    Response::text(400, "SESSION_NOT_FOUND")
}

pub struct SessionSet<T> {
    pub executor: Weak<Executor>,
    pub set: Arc<RwLock<HashMap<SessionId, Arc<ApplinSession<T>>>>>,
}
impl<T: 'static + Send + Sync> SessionSet<T> {
    #[must_use]
    pub fn new(executor: &Arc<Executor>) -> Self {
        Self {
            executor: Arc::downgrade(executor),
            set: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn read_lock(&self) -> RwLockReadGuard<HashMap<SessionId, Arc<ApplinSession<T>>>> {
        self.set.read().unwrap_or_else(PoisonError::into_inner)
    }

    fn write_lock(&self) -> RwLockWriteGuard<HashMap<SessionId, Arc<ApplinSession<T>>>> {
        self.set.write().unwrap_or_else(PoisonError::into_inner)
    }

    /// # Errors
    /// Returns an error when the request has the session cookie but we fail to parse it.
    pub fn get_opt(&self, req: &Request) -> Result<Option<Arc<ApplinSession<T>>>, Response> {
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
    pub fn get(&self, req: &Request) -> Result<Arc<ApplinSession<T>>, Response> {
        self.get_opt(req)?.ok_or_else(session_not_found)
    }

    /// # Errors
    /// Returns an error when the request has the session cookie but we fail to parse it.
    pub fn try_get(&self, req: &Request) -> Result<Option<Response>, Response> {
        if let Some(session) = self.get_opt(req)? {
            Ok(Some(session.stream()?))
        } else {
            Ok(None)
        }
    }

    pub fn new_session<F>(&self, page_map_fn: F, value: T) -> Arc<ApplinSession<T>>
    where
        F: 'static
            + Send
            + Sync
            + Fn(Rebuilder<T>) -> Result<PageMap<T>, Box<dyn std::error::Error>>,
    {
        let session = ApplinSession::new(self.executor.clone(), page_map_fn, value);
        self.write_lock()
            .insert(session.cookie.id(), session.clone());
        session
    }

    /// # Errors
    /// Returns an error when the request has the session cookie but we fail to parse it.
    pub fn get_or_new<F>(
        &self,
        req: &Request,
        page_map_fn: F,
        new_value_fn: impl FnOnce() -> T,
    ) -> Result<Arc<ApplinSession<T>>, Response>
    where
        F: 'static
            + Send
            + Sync
            + Fn(Rebuilder<T>) -> Result<PageMap<T>, Box<dyn std::error::Error>>,
    {
        if let Some(session) = self.get_opt(req)? {
            Ok(session)
        } else {
            let value = new_value_fn();
            Ok(self.new_session(page_map_fn, value))
        }
    }
}
