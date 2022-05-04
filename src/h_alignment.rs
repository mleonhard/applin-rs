use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
pub enum HAlignment {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "end")]
    End,
}
impl HAlignment {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_value(&self) -> Value {
        serde_json::to_value(&self).unwrap()
    }
}
impl From<HAlignment> for Value {
    fn from(src: HAlignment) -> Self {
        src.to_value()
    }
}
impl Default for HAlignment {
    fn default() -> Self {
        Self::Start
    }
}
