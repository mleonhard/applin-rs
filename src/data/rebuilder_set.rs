use crate::data::{Context, Rebuilder};
use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;
use servlin::reexport::safina_executor::Executor;
use servlin::reexport::safina_timer;
use std::collections::HashSet;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct RebuilderSet<T> {
    pub cleanup_task_started: AtomicBool,
    pub set: Arc<RwLock<HashSet<Rebuilder<T>>>>,
}
impl<T: 'static + Send + Sync> RebuilderSet<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            cleanup_task_started: AtomicBool::new(false),
            set: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    #[must_use]
    pub fn with_cleanup_task(self, executor: &Arc<Executor>) -> Self {
        self.start_cleanup_task(executor);
        self
    }

    /// Calling this a second time does nothing.
    pub fn start_cleanup_task(&self, executor: &Arc<Executor>) {
        if self.cleanup_task_started.swap(true, Ordering::AcqRel) {
            // Already started.
        } else {
            let weak_set = Arc::downgrade(&self.set);
            executor.spawn(async move {
                loop {
                    safina_timer::sleep_for(Duration::from_secs(61)).await;
                    if let Some(set) = weak_set.upgrade() {
                        set.write()
                            .unwrap_or_else(PoisonError::into_inner)
                            .retain(Rebuilder::session_fresh);
                    } else {
                        return;
                    }
                }
            });
        }
    }

    fn read(&self) -> RwLockReadGuard<HashSet<Rebuilder<T>>> {
        self.set.read().unwrap_or_else(PoisonError::into_inner)
    }

    fn write(&self) -> RwLockWriteGuard<HashSet<Rebuilder<T>>> {
        self.set.write().unwrap_or_else(PoisonError::into_inner)
    }

    /// Remove rebuilders whose sessions no longer exist.
    pub fn clean(&self) {
        self.write().retain(Rebuilder::session_fresh);
    }

    pub fn clean_if_cleanup_task_not_started(&self) {
        if !self.cleanup_task_started.load(Ordering::Acquire) {
            self.clean();
        }
    }

    pub fn insert(&self, rebuilder: Rebuilder<T>) {
        self.clean_if_cleanup_task_not_started();
        if !self.read().contains(&rebuilder) {
            self.write().insert(rebuilder);
        }
    }

    pub fn remove(&self, rebuilder: &Rebuilder<T>) -> bool {
        self.clean_if_cleanup_task_not_started();
        self.write().remove(rebuilder)
    }

    pub fn rebuild_all(&self, ctx: Context) {
        //dbg!(&session_id);
        self.clean_if_cleanup_task_not_started();
        for rebuilder in self.read().iter() {
            rebuilder.rebuild(ctx);
        }
    }
}
impl<T: 'static + Send + Sync> Default for RebuilderSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
