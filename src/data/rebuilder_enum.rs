use crate::data::Context;
use crate::session::Session;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Weak};

pub enum Rebuilder<T> {
    Keys(Weak<Session<T>>),
    Value(Weak<Session<T>>, String),
}
impl<T> Rebuilder<T> {
    #[must_use]
    pub fn weak_session(&self) -> &Weak<Session<T>> {
        match self {
            Rebuilder::Keys(weak_session) | Rebuilder::Value(weak_session, ..) => weak_session,
        }
    }

    /// # Errors
    /// Returns an error when the session is not found.
    /// This happens when the connection is closed and the session was cleaned up.
    pub fn session(&self) -> Result<Arc<Session<T>>, &'static str> {
        self.weak_session().upgrade().ok_or("session not found")
    }

    #[must_use]
    pub fn session_exists(&self) -> bool {
        self.weak_session().strong_count() > 0
    }

    fn order_num(&self) -> u8 {
        match self {
            Rebuilder::Keys(..) => 0,
            Rebuilder::Value(..) => 1,
        }
    }
}
impl<T: 'static + Send + Sync> Rebuilder<T> {
    pub fn rebuild(&self, rebuilder: &Context) {
        match self {
            Rebuilder::Keys(weak_session) => {
                if let Some(session) = weak_session.upgrade() {
                    session.rebuild_key_set(rebuilder);
                }
            }
            Rebuilder::Value(weak_session, key) => {
                if let Some(session) = weak_session.upgrade() {
                    session.rebuild_value(key, rebuilder);
                }
            }
        }
    }
}
// Deriving `Clone` requires `T: Clone`.
// We don't want that restriction, so we implement `Clone` here.
impl<T> Clone for Rebuilder<T> {
    fn clone(&self) -> Self {
        match self {
            Rebuilder::Keys(weak_session) => Rebuilder::Keys(weak_session.clone()),
            Rebuilder::Value(weak_session, key) => {
                Rebuilder::Value(weak_session.clone(), key.clone())
            }
        }
    }
}
impl<T> PartialEq for Rebuilder<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Rebuilder::Keys(weak), Rebuilder::Keys(other_weak)) => Weak::ptr_eq(weak, other_weak),
            (Rebuilder::Value(weak, key), Rebuilder::Value(other_weak, other_key)) => {
                Weak::ptr_eq(weak, other_weak) && key == other_key
            }
            _ => false,
        }
    }
}
impl<T> Eq for Rebuilder<T> {}
impl<T> Ord for Rebuilder<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Rebuilder::Keys(weak), Rebuilder::Keys(other_weak)) => {
                Weak::as_ptr(weak).cmp(&Weak::as_ptr(other_weak))
            }
            (Rebuilder::Value(weak, key), Rebuilder::Value(other_weak, other_key)) => {
                match Weak::as_ptr(weak).cmp(&Weak::as_ptr(other_weak)) {
                    Ordering::Equal => key.cmp(other_key),
                    other => other,
                }
            }
            (a, b) => a.order_num().cmp(&b.order_num()),
        }
    }
}
impl<T> PartialOrd for Rebuilder<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Hash for Rebuilder<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self {
            Rebuilder::Keys(weak) => Weak::as_ptr(weak).hash(hasher),
            Rebuilder::Value(weak, key) => {
                Weak::as_ptr(weak).hash(hasher);
                key.hash(hasher);
            }
        }
    }
}
