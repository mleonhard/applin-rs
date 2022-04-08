use crate::context::Context;
use core::fmt::{Debug, Formatter};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

/// A set of key strings and functions to generate their values.
pub struct KeySet<T> {
    pub key_to_value_fn: HashMap<
        String,
        Box<dyn 'static + Send + Sync + Fn(&Context<T>) -> Result<Value, Box<dyn Error>>>,
    >,
}
impl<T> KeySet<T> {
    pub fn new() -> Self {
        Self {
            key_to_value_fn: HashMap::new(),
        }
    }

    pub fn with_fn<F>(mut self, key: impl ToString, value_fn: F) -> Self
    where
        F: 'static + Send + Sync + Fn(&Context<T>) -> Result<Value, Box<dyn std::error::Error>>,
    {
        self.key_to_value_fn
            .insert(key.to_string(), Box::new(value_fn));
        self
    }

    pub fn with_static(mut self, key: impl ToString, value: Value) -> Self {
        self.key_to_value_fn.insert(
            key.to_string(),
            Box::new(move |_rebuilder| Ok(value.clone())),
        );
        self
    }

    pub fn add_fn<F>(&mut self, key: impl ToString, value_fn: F)
    where
        F: 'static + Send + Sync + Fn(&Context<T>) -> Result<Value, Box<dyn std::error::Error>>,
    {
        self.key_to_value_fn
            .insert(key.to_string(), Box::new(value_fn));
    }

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
