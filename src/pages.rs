use crate::widgets::Widget;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Page(pub Value);

#[derive(Clone, Deserialize, Eq, Serialize, PartialEq)]
pub struct NavPage {
    #[serde(skip_deserializing)]
    typ: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<Value>,
    title: String,
    widget: Value,
}
impl NavPage {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(title: impl ToString, widget: impl Into<Widget>) -> Self {
        Self {
            typ: "nav-page",
            end: None,
            start: None,
            title: title.to_string(),
            widget: widget.into().0,
        }
    }

    #[must_use]
    pub fn without_back(mut self) -> Self {
        self.start = Some(Value::Null);
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_end(mut self, widget: Widget) -> Self {
        self.end = Some(widget.0);
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_start(mut self, widget: Widget) -> Self {
        self.start = Some(widget.0);
        self
    }
}
impl From<NavPage> for Page {
    fn from(nav_page: NavPage) -> Self {
        let value: Value = nav_page.into();
        Page(value)
    }
}
impl From<NavPage> for Value {
    fn from(nav_page: NavPage) -> Self {
        serde_json::to_value(nav_page).unwrap()
    }
}

#[derive(Clone, Deserialize, Eq, Serialize, PartialEq)]
pub struct PlainPage {
    #[serde(skip_deserializing)]
    typ: &'static str,
    title: String,
    widget: Value,
}
impl PlainPage {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(title: impl ToString, widget: impl Into<Widget>) -> Self {
        Self {
            typ: "plain-page",
            title: title.to_string(),
            widget: widget.into().0,
        }
    }
}
impl From<PlainPage> for Page {
    fn from(plain_page: PlainPage) -> Self {
        let value: Value = plain_page.into();
        Page(value)
    }
}
impl From<PlainPage> for Value {
    fn from(plain_page: PlainPage) -> Self {
        serde_json::to_value(plain_page).unwrap()
    }
}
