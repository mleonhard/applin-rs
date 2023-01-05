use crate::data::Rebuilder;
use crate::internal::Page;
use crate::session::PageKey;
use core::fmt::{Debug, Formatter};
use serde_json::Value;
use std::collections::hash_map::{Iter, Keys};
use std::collections::HashMap;

#[allow(clippy::module_name_repetitions)]
pub type PageFn<T> =
    dyn 'static + Send + Sync + Fn(Rebuilder<T>) -> Result<Value, Box<dyn std::error::Error>>;

/// A map of page key string to page-generator function.
pub struct PageMap<T>(pub HashMap<String, Box<PageFn<T>>>);
impl<T> PageMap<T> {
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_page_fn<F, P: Into<Page>>(mut self, key: impl Into<String>, page_fn: F) -> Self
    where
        F: 'static + Send + Sync + Fn(Rebuilder<T>) -> Result<P, Box<dyn std::error::Error>>,
    {
        self.0.insert(
            key.into(),
            Box::new(move |rebuilder| page_fn(rebuilder).map(Into::into).map(Into::into)),
        );
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_static_page(mut self, key: impl Into<String>, page: impl Into<Page>) -> Self {
        let value = page.into();
        self.0
            .insert(key.into(), Box::new(move |_rebuilder| Ok(value.to_value())));
        self
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn add_page_fn<F, P: Into<Page>>(&mut self, key: impl Into<String>, page_fn: F) -> PageKey
    where
        F: 'static + Send + Sync + Fn(Rebuilder<T>) -> Result<P, Box<dyn std::error::Error>>,
    {
        let key = key.into();
        self.0.insert(
            key.clone(),
            Box::new(move |rebuilder| {
                page_fn(rebuilder)
                    .map(Into::into)
                    .map(|page: Page| page.to_value())
            }),
        );
        PageKey::new(key)
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn add_static_page(&mut self, key: impl Into<String>, page: impl Into<Page>) -> PageKey {
        let key = key.into();
        let value = page.into();
        // TODO: Warn if key already exists.
        self.0.insert(
            key.clone(),
            Box::new(move |_rebuilder| Ok(value.to_value())),
        );
        PageKey::new(key)
    }

    #[must_use]
    pub fn keys(&self) -> Keys<'_, String, Box<PageFn<T>>> {
        self.0.keys()
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_, String, Box<PageFn<T>>> {
        self.0.iter()
    }

    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Box<PageFn<T>>> {
        self.0.get(key)
    }
}
impl<T> Debug for PageMap<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut keys: Vec<&String> = self.0.keys().collect();
        keys.sort();
        write!(f, "Keys({:?})", keys)
    }
}
impl<T> Default for PageMap<T> {
    fn default() -> Self {
        Self::new()
    }
}
