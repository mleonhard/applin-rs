use crate::data::random_positive_nonzero_i64;
use core::fmt::Debug;
use std::fmt::Display;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ServerInstanceId(pub i64);
impl ServerInstanceId {
    #[must_use]
    pub fn new_random() -> Self {
        Self(random_positive_nonzero_i64())
    }
}
impl Debug for ServerInstanceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ServerInstanceId({}", self.id)
    }
}
impl Display for ServerInstanceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.id)
    }
}
