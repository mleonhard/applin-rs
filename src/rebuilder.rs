use crate::session::Session;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Weak};

#[derive(Clone)]
pub enum Rebuilder {
    Keys(Weak<Session>),
    Value(Weak<Session>, String),
}
impl Rebuilder {
    pub fn schedule_rebuild(&self, rpc_session: Option<&Arc<Session>>) {
        // TODO: Schedule only one worker at a time per session.
        match self {
            Rebuilder::Keys(weak_session) => {
                if let Some(arc_session) = weak_session.upgrade() {
                    arc_session.schedule_rebuild_keys(rpc_session)
                }
            }
            Rebuilder::Value(weak_session, key) => {
                if let Some(arc_session) = weak_session.upgrade() {
                    arc_session.schedule_rebuild_value(key, rpc_session)
                }
            }
        }
    }

    pub fn session(&self) -> Option<Arc<Session>> {
        let weak_session = match self {
            Rebuilder::Keys(weak_session) | Rebuilder::Value(weak_session, ..) => weak_session,
        };
        weak_session.upgrade()
    }

    pub fn session_exists(&self) -> bool {
        let weak_session = match self {
            Rebuilder::Keys(weak_session) | Rebuilder::Value(weak_session, ..) => weak_session,
        };
        weak_session.strong_count() > 0
    }
}
impl PartialEq for Rebuilder {
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
impl Eq for Rebuilder {}
impl PartialOrd for Rebuilder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Rebuilder {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Rebuilder::Keys(weak), Rebuilder::Keys(other_arc)) => {
                Weak::as_ptr(weak).cmp(&Weak::as_ptr(other_arc))
            }
            (Rebuilder::Value(weak, key), Rebuilder::Value(other_arc, other_key)) => {
                match Weak::as_ptr(weak).cmp(&Weak::as_ptr(other_arc)) {
                    Ordering::Equal => key.cmp(other_key),
                    other => other,
                }
            }
            (Rebuilder::Keys(..), Rebuilder::Value(..)) => Ordering::Less,
            (Rebuilder::Value(..), Rebuilder::Keys(..)) => Ordering::Greater,
        }
    }
}
impl Hash for Rebuilder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Rebuilder::Keys(weak) => Weak::as_ptr(weak).hash(state),
            Rebuilder::Value(weak, key) => {
                Weak::as_ptr(weak).hash(state);
                key.hash(state);
            }
        }
    }
}
