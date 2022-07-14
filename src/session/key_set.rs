use crate::data::Context;
use crate::internal::Page;
use crate::session::PageKey;
use core::fmt::{Debug, Formatter};
use serde_json::Value;
use std::collections::HashMap;

#[allow(clippy::module_name_repetitions)]
pub type KeySetFn<T> =
    dyn 'static + Send + Sync + Fn(&Context<T>) -> Result<Value, Box<dyn std::error::Error>>;

/// A set of key strings and functions to generate their values.
pub struct KeySet<T> {
    pub key_to_value_fn: HashMap<String, Box<KeySetFn<T>>>,
}
impl<T> KeySet<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            key_to_value_fn: HashMap::new(),
        }
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_page_fn<F, P: Into<Page>>(mut self, key: impl Into<String>, page_fn: F) -> Self
    where
        F: 'static + Send + Sync + Fn(&Context<T>) -> Result<P, Box<dyn std::error::Error>>,
    {
        self.key_to_value_fn.insert(
            key.into(),
            Box::new(move |ctx| page_fn(ctx).map(Into::into).map(Into::into)),
        );
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_value_fn<F, V: Into<Value>>(mut self, key: impl Into<String>, value_fn: F) -> Self
    where
        F: 'static + Send + Sync + Fn(&Context<T>) -> Result<V, Box<dyn std::error::Error>>,
    {
        self.key_to_value_fn.insert(
            key.into(),
            Box::new(move |ctx| value_fn(ctx).map(Into::into)),
        );
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_static_page(mut self, key: impl Into<String>, page: impl Into<Page>) -> Self {
        let value = page.into();
        self.key_to_value_fn
            .insert(key.into(), Box::new(move |_rebuilder| Ok(value.to_value())));
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_static_value(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        let value = value.into();
        self.key_to_value_fn
            .insert(key.into(), Box::new(move |_rebuilder| Ok(value.clone())));
        self
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn add_page_fn<F, P: Into<Page>>(&mut self, key: impl Into<String>, page_fn: F) -> PageKey
    where
        F: 'static + Send + Sync + Fn(&Context<T>) -> Result<P, Box<dyn std::error::Error>>,
    {
        let key = key.into();
        self.key_to_value_fn.insert(
            key.clone(),
            Box::new(move |ctx| {
                page_fn(ctx)
                    .map(Into::into)
                    .map(|page: Page| page.to_value())
            }),
        );
        PageKey::new(key)
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn add_value_fn<F, V: Into<Value>>(
        &mut self,
        key: impl Into<String>,
        value_fn: F,
    ) -> PageKey
    where
        F: 'static + Send + Sync + Fn(&Context<T>) -> Result<V, Box<dyn std::error::Error>>,
    {
        let key = key.into();
        self.key_to_value_fn.insert(
            key.clone(),
            Box::new(move |ctx| value_fn(ctx).map(Into::into)),
        );
        PageKey::new(key)
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn add_static_page(&mut self, key: impl Into<String>, page: impl Into<Page>) -> PageKey {
        let key = key.into();
        let value = page.into();
        // TODO: Warn if key already exists.
        self.key_to_value_fn.insert(
            key.clone(),
            Box::new(move |_rebuilder| Ok(value.to_value())),
        );
        PageKey::new(key)
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn add_static_value(&mut self, key: impl Into<String>, value: impl Into<Value>) -> PageKey {
        let key = key.into();
        let value = value.into();
        self.key_to_value_fn
            .insert(key.clone(), Box::new(move |_rebuilder| Ok(value.clone())));
        PageKey::new(key)
    }
}
impl<T> Debug for KeySet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut keys: Vec<&String> = self.key_to_value_fn.keys().collect();
        keys.sort();
        write!(f, "Keys<{}>({:?})", core::any::type_name::<T>(), keys)
    }
}
impl<T> Default for KeySet<T> {
    fn default() -> Self {
        Self::new()
    }
}
