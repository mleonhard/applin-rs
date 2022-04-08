use crate::session::Session;
use crate::session_id::SessionId;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Weak};

pub enum Context<T> {
    Keys(Weak<Session<T>>),
    Value(Weak<Session<T>>, String),
}
impl<T: 'static + Send + Sync> Context<T> {
    pub fn rebuild(&self, session_id: Option<SessionId>) {
        // TODO: Schedule only one worker at a time per session.
        match self {
            Context::Keys(weak_session) => {
                if let Some(session) = weak_session.upgrade() {
                    session.schedule_rebuild_key_set(session_id)
                }
            }
            Context::Value(weak_session, key) => {
                if let Some(session) = weak_session.upgrade() {
                    session.schedule_rebuild_value(key, session_id)
                }
            }
        }
    }

    pub fn session(&self) -> Result<Arc<Session<T>>, &'static str> {
        let weak_session = match self {
            Context::Keys(weak_session) | Context::Value(weak_session, ..) => weak_session,
        };
        weak_session.upgrade().ok_or("session not found")
    }

    pub fn session_exists(&self) -> bool {
        let weak_session = match self {
            Context::Keys(weak_session) | Context::Value(weak_session, ..) => weak_session,
        };
        weak_session.strong_count() > 0
    }
}
impl<T> Clone for Context<T> {
    fn clone(&self) -> Self {
        match self {
            Context::Keys(weak_session) => Context::Keys(weak_session.clone()),
            Context::Value(weak_session, key) => Context::Value(weak_session.clone(), key.clone()),
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
            (Context::Keys(..), Context::Value(..)) => Ordering::Less,
            (Context::Value(..), Context::Keys(..)) => Ordering::Greater,
        }
    }
}
impl<T> PartialOrd for Context<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Hash for Context<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Context::Keys(weak) => Weak::as_ptr(weak).hash(state),
            Context::Value(weak, key) => {
                Weak::as_ptr(weak).hash(state);
                key.hash(state);
            }
        }
    }
}
