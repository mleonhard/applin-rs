use crate::internal::Action;
use crate::is_default;
use crate::widget::HAlignment;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
#[serde(tag = "typ")]
pub enum Widget {
    #[serde(rename = "back-button")]
    BackButtonVariant { actions: Vec<Action> },
    #[serde(rename = "button")]
    ButtonVariant {
        text: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actions: Vec<Action>,
    },
    #[serde(rename = "checkbox")]
    CheckBoxVariant {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actions: Vec<Action>,
        id: String,
        #[serde(rename = "initial-bool")]
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        initial_bool: bool,
    },
    #[serde(rename = "column")]
    ColumnVariant {
        #[serde(default, rename = "h-alignment")]
        h_alignment: HAlignment,
        #[serde(default, skip_serializing_if = "is_default")]
        spacing: u16,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        widgets: Vec<Widget>,
    },
    #[serde(rename = "empty")]
    EmptyVariant,
    #[serde(rename = "error-details")]
    ErrorDetailsVariant,
    #[serde(rename = "form")]
    FormVariant {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        widgets: Vec<Widget>,
    },
    #[serde(rename = "form-button")]
    FormButtonVariant {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actions: Vec<Action>,
        #[serde(rename = "is-destructive")]
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_destructive: bool,
        text: String,
    },
    #[serde(rename = "form-checkbox")]
    FormCheckboxVariant {
        id: String,
        #[serde(rename = "initial-bool")]
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        initial_bool: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rpc: Option<String>,
        text: String,
    },
    #[serde(rename = "form-detail")]
    FormDetailVariant {
        text: String,
        #[serde(rename = "sub-text", default, skip_serializing_if = "Option::is_none")]
        sub_text: Option<String>,
        #[serde(rename = "photo-url", default, skip_serializing_if = "Option::is_none")]
        photo_url: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actions: Vec<Action>,
    },
    #[serde(rename = "form-error")]
    FormErrorVariant { text: String },
    #[serde(rename = "form-section")]
    FormSectionVariant {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        widgets: Vec<Widget>,
    },
    #[serde(rename = "modal-button")]
    ModalButtonVariant {
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
    ScrollVariant { widget: Box<Widget> },
    #[serde(rename = "text")]
    TextVariant { text: String },
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
        Self::EmptyVariant
    }
}
