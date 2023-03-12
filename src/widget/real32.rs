use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::num::FpCategory;

/// A 32-bit floating point number that cannot be NaN, infinity, or negative zero.
#[derive(Clone, Debug, Serialize)]
pub struct Real32(f32);
impl Real32 {
    /// Silently changes `-0.0` into `0.0`.
    ///
    /// # Panics
    /// Panics when `v` is NaN or infinity.
    #[must_use]
    pub fn new(v: f32) -> Self {
        Self::try_from(v).unwrap()
    }

    #[must_use]
    pub fn get(self) -> f32 {
        self.0
    }
}
impl TryFrom<f32> for Real32 {
    type Error = String;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value.is_finite() {
            if value.classify() == FpCategory::Zero && value.is_sign_negative() {
                Ok(Self(0.0))
            } else {
                Ok(Self(value))
            }
        } else {
            Err(format!("cannot make Real32 from {value:?}"))
        }
    }
}
impl PartialEq for Real32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0) == Ordering::Equal
    }
}
impl Eq for Real32 {}
impl PartialOrd for Real32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.total_cmp(&other.0))
    }
}
impl Ord for Real32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl Hash for Real32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.0.classify() {
            FpCategory::Nan => "NaN".hash(state),
            FpCategory::Infinite => "inf".hash(state),
            FpCategory::Zero | FpCategory::Subnormal | FpCategory::Normal => {
                self.0.to_bits().hash(state);
            }
        }
    }
}
impl<'de> Deserialize<'de> for Real32 {
    fn deserialize<D>(deserializer: D) -> Result<Real32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: f32 = f32::deserialize(deserializer)?;
        Real32::try_from(raw).map_err(serde::de::Error::custom)
    }
}
