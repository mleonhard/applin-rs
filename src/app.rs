use crate::rebuilder::Rebuilder;
use serde_json::Value;
use std::collections::HashMap;

pub type AppFn =
    dyn 'static + Send + Sync + Fn(Rebuilder) -> Result<App, Box<dyn std::error::Error>>;

pub type ValueFn =
    dyn 'static + Send + Sync + Fn(Rebuilder) -> Result<Value, Box<dyn std::error::Error>>;

pub struct App {
    pub key_to_value_fn: HashMap<String, Box<ValueFn>>,
}
impl App {
    pub fn new() -> Self {
        Self {
            key_to_value_fn: HashMap::new(),
        }
    }

    pub fn with_fn<F>(mut self, key: impl ToString, value_fn: F) -> Self
    where
        F: 'static + Send + Sync + Fn(Rebuilder) -> Result<Value, Box<dyn std::error::Error>>,
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
        F: 'static + Send + Sync + Fn(Rebuilder) -> Result<Value, Box<dyn std::error::Error>>,
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
