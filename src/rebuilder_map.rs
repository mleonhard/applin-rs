use crate::rebuilder::Rebuilder;
use crate::session::Session;
use beatrice::reexport::safina_executor::Executor;
use beatrice::reexport::safina_timer;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;

pub struct RebuilderMap<K: 'static + Clone + Debug + Eq + Hash + Send + Sync> {
    rebuilders: Arc<Mutex<HashMap<K, Rebuilder>>>,
}
impl<K: 'static + Clone + Debug + Eq + Hash + Send + Sync> RebuilderMap<K> {
    pub async fn cleanup_task(weak_rebuilders: Weak<Mutex<HashMap<K, Rebuilder>>>) {
        loop {
            safina_timer::sleep_for(Duration::from_secs(67)).await;
            if let Some(arc_rebuilders) = weak_rebuilders.upgrade() {
                arc_rebuilders
                    .lock()
                    .unwrap()
                    .retain(|_key, rebuilder| rebuilder.session_exists());
            } else {
                return;
            }
        }
    }

    pub fn new(executor: &Arc<Executor>) -> Self {
        let rebuilders = Arc::new(Mutex::new(HashMap::new()));
        executor.spawn(Self::cleanup_task(Arc::downgrade(&rebuilders)));
        Self { rebuilders }
    }

    pub fn insert(&self, key: K, rebuilder: Rebuilder) {
        self.rebuilders.lock().unwrap().insert(key, rebuilder);
    }

    pub fn remove(&self, key: &K) -> Option<Rebuilder> {
        self.rebuilders.lock().unwrap().remove(key)
    }

    pub fn rebuild(&self, key: &K, rpc_session: Option<&Arc<Session>>) {
        dbg!(key, &rpc_session);
        if let Some(rebuilder) = self.rebuilders.lock().unwrap().get(key) {
            rebuilder.schedule_rebuild(rpc_session);
        }
    }

    pub fn rebuild_all(&self, rpc_session: Option<&Arc<Session>>) {
        dbg!(&rpc_session);
        for rebuilder in self.rebuilders.lock().unwrap().values() {
            rebuilder.schedule_rebuild(rpc_session);
        }
    }
}
