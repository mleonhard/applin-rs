use crate::internal::Widget;
use crate::is_default;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
#[serde(tag = "typ")]
pub enum Page {
    #[serde(rename = "alert-modal")]
    Alert {
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        title: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[serde(default)]
        widgets: Vec<Widget>,
    },
    #[serde(rename = "drawer-modal")]
    Drawer {
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        title: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[serde(default)]
        widgets: Vec<Widget>,
    },
    #[serde(rename = "nav-page")]
    Nav {
        #[serde(skip_serializing_if = "Option::is_none")]
        end: Option<Widget>,
        #[serde(rename = "poll-seconds")]
        #[serde(default, skip_serializing_if = "is_default")]
        poll_seconds: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        start: Option<Widget>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        stream: bool,
        title: String,
        widget: Widget,
    },
    #[serde(rename = "plain-page")]
    Plain {
        #[serde(rename = "poll-seconds")]
        #[serde(default, skip_serializing_if = "is_default")]
        poll_seconds: u32,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        stream: bool,
        title: String,
        widget: Widget,
    },
}
impl Page {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
impl From<Page> for Value {
    fn from(src: Page) -> Self {
        src.to_value()
    }
}
impl Default for Page {
    fn default() -> Self {
        Self::Nav {
            end: None,
            poll_seconds: 0,
            start: None,
            stream: false,
            title: "Default Page".to_string(),
            widget: Widget::EmptyVariant,
        }
    }
}
