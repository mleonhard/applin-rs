use crate::rebuilder::Rebuilder;
use crate::session::Session;
use beatrice::reexport::safina_executor::Executor;
use beatrice::reexport::safina_timer;
use std::collections::HashSet;
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;

pub struct RebuilderSet {
    rebuilders: Arc<Mutex<HashSet<Rebuilder>>>,
}
impl RebuilderSet {
    pub async fn cleanup_task(weak_rebuilders: Weak<Mutex<HashSet<Rebuilder>>>) {
        loop {
            safina_timer::sleep_for(Duration::from_secs(61)).await;
            if let Some(arc_rebuilders) = weak_rebuilders.upgrade() {
                arc_rebuilders
                    .lock()
                    .unwrap()
                    .retain(Rebuilder::session_exists);
            } else {
                return;
            }
        }
    }

    pub fn new(executor: &Arc<Executor>) -> Self {
        let rebuilders = Arc::new(Mutex::new(HashSet::new()));
        executor.spawn(Self::cleanup_task(Arc::downgrade(&rebuilders)));
        Self { rebuilders }
    }

    pub fn insert(&self, rebuilder: Rebuilder) {
        self.rebuilders.lock().unwrap().insert(rebuilder);
    }

    pub fn remove(&self, rebuilder: &Rebuilder) {
        self.rebuilders.lock().unwrap().remove(rebuilder);
    }

    pub fn rebuild_all(&self, rpc_session: Option<&Arc<Session>>) {
        dbg!(&rpc_session);
        for rebuilder in self.rebuilders.lock().unwrap().iter() {
            rebuilder.schedule_rebuild(rpc_session);
        }
    }
}
