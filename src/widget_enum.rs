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
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actions: Vec<Action>,
        #[serde(default, rename = "is-cancel")]
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        is_cancel: bool,
        #[serde(default, rename = "is-default")]
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        is_default: bool,
        #[serde(default, rename = "is-destructive")]
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        is_destructive: bool,
    },
    #[serde(rename = "checkbox")]
    CheckBox {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actions: Vec<Action>,
        id: String,
        #[serde(default, rename = "initial-bool")]
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        initial_bool: bool,
    },
    #[serde(rename = "column")]
    Column {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        widgets: Vec<Widget>,
        #[serde(default, rename = "h-alignment")]
        h_alignment: HAlignment,
        #[serde(default, skip_serializing_if = "is_default")]
        spacing: u16,
    },
    #[serde(rename = "detail-cell")]
    DetailCell {
        text: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actions: Vec<Action>,
        #[serde(rename = "photo-url", skip_serializing_if = "Option::is_none")]
        photo_url: Option<String>,
    },
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "horizontal-scroll")]
    HorizontalScroll { widget: Box<Widget> },
    #[serde(rename = "list")]
    List {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        widgets: Vec<Widget>,
    },
    #[serde(rename = "scroll")]
    Scroll { widget: Box<Widget> },
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
