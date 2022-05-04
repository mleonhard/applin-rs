use crate::widget_enum::Widget;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
#[serde(tag = "typ")]
pub enum Page {
    #[serde(rename = "nav-page")]
    Nav {
        title: String,
        widget: Widget,
        #[serde(skip_serializing_if = "Option::is_none")]
        start: Option<Widget>,
        #[serde(skip_serializing_if = "Option::is_none")]
        end: Option<Widget>,
    },
    #[serde(rename = "plain-page")]
    Plain { title: String, widget: Widget },
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
            start: None,
            title: "Default Page".to_string(),
            widget: Widget::Empty,
        }
    }
}
