use crate::action::Action;
use crate::h_alignment::HAlignment;
use serde::{Deserialize, Serialize};
use serde_json::Value;

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == Default::default()
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
#[serde(tag = "typ")]
pub enum Widget {
    #[serde(rename = "back-button")]
    BackButton { actions: Vec<Action> },
    #[serde(rename = "button")]
    Button {
        text: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[serde(default)]
        actions: Vec<Action>,
    },
    #[serde(rename = "column")]
    Column {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[serde(default)]
        widgets: Vec<Widget>,
        #[serde(rename = "h-alignment")]
        #[serde(default)]
        h_alignment: HAlignment,
        #[serde(skip_serializing_if = "is_default")]
        #[serde(default)]
        spacing: u16,
    },
    #[serde(rename = "detail-cell")]
    DetailCell {
        text: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[serde(default)]
        actions: Vec<Action>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo: Option<String>,
    },
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "text")]
    Text { text: String },
}
impl Widget {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_value(&self) -> Value {
        serde_json::to_value(&self).unwrap()
    }
}
impl From<Widget> for Value {
    fn from(src: Widget) -> Self {
        src.to_value()
    }
}
impl Default for Widget {
    fn default() -> Self {
        Self::Empty
    }
}
