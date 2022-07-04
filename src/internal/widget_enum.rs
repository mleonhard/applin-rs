use crate::internal::Action;
use crate::is_default;
use crate::widget::HAlignment;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    },
    #[serde(rename = "checkbox")]
    CheckBox {
        id: String,
        #[serde(rename = "initial-bool")]
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        initial_bool: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actions: Vec<Action>,
    },
    #[serde(rename = "column")]
    Column {
        #[serde(default, rename = "h-alignment")]
        h_alignment: HAlignment,
        #[serde(default, skip_serializing_if = "is_default")]
        spacing: u16,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        widgets: Vec<Widget>,
    },
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "error-details")]
    ErrorDetails,
    #[serde(rename = "form")]
    Form {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        widgets: Vec<Widget>,
    },
    #[serde(rename = "form-button")]
    FormButton {
        text: String,
        #[serde(rename = "is-destructive")]
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_destructive: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actions: Vec<Action>,
    },
    #[serde(rename = "form-detail")]
    FormDetail {
        text: String,
        #[serde(rename = "sub-text", skip_serializing_if = "Option::is_none")]
        sub_text: Option<String>,
        #[serde(rename = "photo-url", skip_serializing_if = "Option::is_none")]
        photo_url: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actions: Vec<Action>,
    },
    #[serde(rename = "form-section")]
    FormSection {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        widgets: Vec<Widget>,
    },
    #[serde(rename = "modal-button")]
    ModalButton {
        text: String,
        #[serde(rename = "is-cancel")]
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_cancel: bool,
        #[serde(rename = "is-default")]
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_default: bool,
        #[serde(rename = "is-destructive")]
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_destructive: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actions: Vec<Action>,
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
