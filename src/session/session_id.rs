use crate::data::random_u64;
use core::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use servlin::internal::escape_and_elide;
use std::str::FromStr;

#[derive(Clone, Copy, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SessionId(u64);
impl SessionId {
    #[must_use]
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn new_random() -> Self {
        Self(random_u64())
    }

    #[must_use]
    pub fn inner(&self) -> u64 {
        self.0
    }
}
impl TryFrom<&str> for SessionId {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let value: u64 = s
            .parse()
            .map_err(|_e| format!("invalid SessionId: {}", escape_and_elide(s.as_bytes(), 20)))?;
        Ok(Self(value))
    }
}
impl FromStr for SessionId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}
impl Debug for SessionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "SessionId({})", self.0)
    }
}
