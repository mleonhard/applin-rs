use crate::context::Context;
use core::fmt::{Debug, Formatter};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[allow(clippy::module_name_repetitions)]
pub type KeySetFn<T> = dyn 'static + Send + Sync + Fn(&Context<T>) -> Result<Value, Box<dyn Error>>;

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
    pub fn with_fn<F>(mut self, key: impl ToString, value_fn: F) -> Self
    where
        F: 'static + Send + Sync + Fn(&Context<T>) -> Result<Value, Box<dyn std::error::Error>>,
    {
        self.key_to_value_fn
            .insert(key.to_string(), Box::new(value_fn));
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_static(mut self, key: impl ToString, value: Value) -> Self {
        self.key_to_value_fn.insert(
            key.to_string(),
            Box::new(move |_rebuilder| Ok(value.clone())),
        );
        self
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn add_fn<F>(&mut self, key: impl ToString, value_fn: F)
    where
        F: 'static + Send + Sync + Fn(&Context<T>) -> Result<Value, Box<dyn std::error::Error>>,
    {
        self.key_to_value_fn
            .insert(key.to_string(), Box::new(value_fn));
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn add_static(&mut self, key: impl ToString, value: Value) {
        self.key_to_value_fn.insert(
            key.to_string(),
            Box::new(move |_rebuilder| Ok(value.clone())),
        );
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
