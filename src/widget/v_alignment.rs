use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
pub enum VAlignment {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "bottom")]
    Bottom,
}
impl VAlignment {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_value(&self) -> Value {
        serde_json::to_value(&self).unwrap()
    }
}
impl From<VAlignment> for Value {
    fn from(src: VAlignment) -> Self {
        src.to_value()
    }
}
impl Default for VAlignment {
    fn default() -> Self {
        Self::Top
    }
}
