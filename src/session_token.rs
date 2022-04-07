use beatrice::internal::escape_and_elide;
use core::fmt::{Debug, Formatter};
use nanorand::{ChaCha, Rng};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Copy, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SessionToken(pub u64);
impl SessionToken {
    pub fn new(rng: &mut ChaCha<20>) -> Self {
        Self(rng.generate_range(0_u64..u64::MAX))
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}
impl TryFrom<&str> for SessionToken {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let value: u64 = s.parse().map_err(|_e| {
            format!(
                "invalid SessionToken: {}",
                escape_and_elide(s.as_bytes(), 20)
            )
        })?;
        Ok(Self(value))
    }
}
impl FromStr for SessionToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}
impl Debug for SessionToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "SessionId(...{})", self.0 % 1000)
    }
}
