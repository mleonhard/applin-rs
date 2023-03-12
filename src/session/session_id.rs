use crate::data::random_positive_nonzero_i64;
use core::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use servlin::internal::escape_and_elide;
use std::str::FromStr;

#[derive(Clone, Copy, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SessionId(i64);
impl SessionId {
    #[must_use]
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn new_random() -> Self {
        Self(random_positive_nonzero_i64())
    }

    #[must_use]
    pub fn inner(&self) -> i64 {
        self.0
    }
}
impl TryFrom<&str> for SessionId {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let err = || format!("invalid SessionId: {}", escape_and_elide(s.as_bytes(), 20));
        let value: i64 = s.parse().map_err(|_e| err())?;
        if value < 0 {
            return Err(err());
        }
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
