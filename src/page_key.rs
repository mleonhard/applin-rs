use std::ops::Deref;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PageKey(String);
impl PageKey {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl Deref for PageKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
