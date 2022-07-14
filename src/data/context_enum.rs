use crate::session::{Session, SessionId};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Weak};

pub enum Context<T> {
    // TODO: Remove Keys and Value variants.
    Keys(Weak<Session<T>>),
    Value(Weak<Session<T>>, String),
    Rpc(SessionId),
    Empty,
}
impl<T: 'static + Send + Sync> Context<T> {
    pub fn rebuild(&self, ctx: &Context<T>) {
        match self {
            Context::Keys(weak_session) => {
                if let Some(session) = weak_session.upgrade() {
                    session.schedule_rebuild_key_set(ctx);
                }
            }
            Context::Value(weak_session, key) => {
                if let Some(session) = weak_session.upgrade() {
                    session.schedule_rebuild_value(key, ctx);
                }
            }
            Context::Rpc(..) | Context::Empty => {}
        }
    }

    /// # Errors
    /// Returns an error when the session is not found.
    /// This happens when the connection is closed and the session was cleaned up.
    pub fn session(&self) -> Result<Arc<Session<T>>, &'static str> {
        let weak_session = match self {
            Context::Keys(weak_session) | Context::Value(weak_session, ..) => weak_session,
            Context::Rpc(..) => return Err("Context::Rpc has no session"),
            Context::Empty => return Err("Context::Empty has no session"),
        };
        weak_session.upgrade().ok_or("session not found")
    }
}
impl<T> Context<T> {
    #[must_use]
    pub fn session_exists(&self) -> bool {
        match self {
            Context::Keys(weak_session) | Context::Value(weak_session, ..) => {
                weak_session.strong_count() > 0
            }
            Context::Rpc(..) | Context::Empty => false,
        }
    }

    fn order_num(&self) -> u8 {
        match self {
            Context::Keys(..) => 0,
            Context::Value(..) => 1,
            Context::Rpc(..) => 2,
            Context::Empty => 3,
        }
    }
}
// Deriving `Clone` requires `T: Clone`.
// We don't want that restriction, so we implement `Clone` here.
impl<T> Clone for Context<T> {
    fn clone(&self) -> Self {
        match self {
            Context::Keys(weak_session) => Context::Keys(weak_session.clone()),
            Context::Value(weak_session, key) => Context::Value(weak_session.clone(), key.clone()),
            Context::Rpc(session_id) => Context::Rpc(*session_id),
            Context::Empty => Context::Empty,
        }
    }
}
impl<T> PartialEq for Context<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Context::Keys(weak), Context::Keys(other_weak)) => Weak::ptr_eq(weak, other_weak),
            (Context::Value(weak, key), Context::Value(other_weak, other_key)) => {
                Weak::ptr_eq(weak, other_weak) && key == other_key
            }
            (Context::Rpc(session_id1), Context::Rpc(session_id2)) => session_id1 == session_id2,
            (Context::Empty, Context::Empty) => true,
            _ => false,
        }
    }
}
impl<T> Eq for Context<T> {}
impl<T> Ord for Context<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Context::Keys(weak), Context::Keys(other_weak)) => {
                Weak::as_ptr(weak).cmp(&Weak::as_ptr(other_weak))
            }
            (Context::Value(weak, key), Context::Value(other_weak, other_key)) => {
                match Weak::as_ptr(weak).cmp(&Weak::as_ptr(other_weak)) {
                    Ordering::Equal => key.cmp(other_key),
                    other => other,
                }
            }
            (Context::Rpc(session_id1), Context::Rpc(session_id2)) => session_id1.cmp(session_id2),
            (Context::Empty, Context::Empty) => Ordering::Equal,
            (a, b) => a.order_num().cmp(&b.order_num()),
        }
    }
}
impl<T> PartialOrd for Context<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Hash for Context<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self {
            Context::Keys(weak) => Weak::as_ptr(weak).hash(hasher),
            Context::Value(weak, key) => {
                Weak::as_ptr(weak).hash(hasher);
                key.hash(hasher);
            }
            Context::Rpc(session_id) => session_id.hash(hasher),
            Context::Empty => {}
        }
    }
}
