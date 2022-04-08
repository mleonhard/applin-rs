use crate::context::Context;
use crate::context_set::ContextSet;
use beatrice::reexport::safina_executor::Executor;
use core::fmt::{Debug, Formatter};
use core::ops::{Deref, DerefMut};
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[allow(clippy::module_name_repetitions)]
pub struct RosterWriteGuard<'x, V, T: 'static + Send + Sync>(
    Option<RwLockWriteGuard<'x, V>>,
    &'x ContextSet<T>,
    &'x Context<T>,
);
impl<'x, V, T: 'static + Send + Sync> Deref for RosterWriteGuard<'x, V, T> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &*self.0.as_ref().unwrap()
    }
}
impl<'x, V, T: 'static + Send + Sync> DerefMut for RosterWriteGuard<'x, V, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0.as_mut().unwrap()
    }
}
impl<'x, V, T: 'static + Send + Sync> Drop for RosterWriteGuard<'x, V, T> {
    fn drop(&mut self) {
        self.0.take();
        self.1.rebuild_all(self.2);
    }
}

pub struct Roster<V, T: 'static + Send + Sync> {
    pub context_set: ContextSet<T>,
    value: RwLock<V>,
}
impl<V, T: 'static + Send + Sync> Roster<V, T> {
    pub fn new(value: V) -> Self {
        Self {
            context_set: ContextSet::new(),
            value: RwLock::new(value),
        }
    }

    #[must_use]
    pub fn with_cleanup_task(self, executor: &Arc<Executor>) -> Self {
        self.context_set.start_cleanup_task(executor);
        self
    }

    /// Calling this a second time does nothing.
    pub fn start_cleanup_task(&self, executor: &Arc<Executor>) {
        self.context_set.start_cleanup_task(executor);
    }

    pub fn subscribe(&self, ctx: &Context<T>) {
        self.context_set.insert(ctx);
    }

    pub fn unsubscribe(&self, ctx: &Context<T>) {
        self.context_set.remove(ctx);
    }

    fn value_read_lock(&self) -> RwLockReadGuard<V> {
        self.value.read().unwrap_or_else(PoisonError::into_inner)
    }

    fn value_write_lock(&self) -> RwLockWriteGuard<V> {
        self.value.write().unwrap_or_else(PoisonError::into_inner)
    }

    /// Read the value and subscribe to changes.
    pub fn read(&self, ctx: &Context<T>) -> RwLockReadGuard<'_, V> {
        self.context_set.insert(ctx);
        self.value_read_lock()
    }

    /// Read the value.
    pub fn peek(&self) -> RwLockReadGuard<'_, V> {
        self.value_read_lock()
    }

    /// Get a write lock on the value.
    /// When the returned guard drops, it rebuilds all subscribed contexts.
    pub fn write<'x>(&'x self, ctx: &'x Context<T>) -> RosterWriteGuard<'x, V, T> {
        RosterWriteGuard(Some(self.value_write_lock()), &self.context_set, ctx)
    }

    /// Write the value without updating clients.
    pub fn stealth_write(&self) -> RwLockWriteGuard<'_, V> {
        self.value_write_lock()
    }
}
impl<V, T: 'static + Send + Sync> Debug for Roster<V, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "Roster<{},{}>",
            core::any::type_name::<V>(),
            core::any::type_name::<T>(),
        )
    }
}
