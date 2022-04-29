use crate::widget_list::WidgetList;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct Widget(pub Value);

#[derive(Clone, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
pub struct Button {
    #[serde(skip_deserializing)]
    typ: &'static str,
    // TODO: Make an Action enum.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    actions: Vec<String>,
    text: String,
}
impl Button {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(text: impl ToString) -> Self {
        Self {
            typ: "button",
            actions: Vec::new(),
            text: text.to_string(),
        }
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_action(mut self, action: impl ToString) -> Self {
        self.actions.push(action.to_string());
        self
    }

    #[must_use]
    pub fn with_actions(mut self, actions: impl IntoIterator<Item = impl ToString>) -> Self {
        self.actions
            .extend(actions.into_iter().map(|action| action.to_string()));
        self
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_widget(self) -> Widget {
        Widget(serde_json::to_value(self).unwrap())
    }
}
impl From<Button> for Widget {
    fn from(inner: Button) -> Self {
        Widget(serde_json::to_value(inner).unwrap())
    }
}

#[derive(Clone, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
pub enum HAlignment {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "end")]
    End,
}

#[derive(Clone, Deserialize, Eq, Serialize, PartialEq)]
pub struct Column {
    #[serde(skip_deserializing)]
    typ: &'static str,
    widgets: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alignment: Option<HAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spacing: Option<u16>,
}
impl Column {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(widgets: impl Into<WidgetList>) -> Self {
        Self {
            typ: "column",
            widgets: widgets.into().0,
            alignment: None,
            spacing: None,
        }
    }

    #[must_use]
    pub fn with_alignment(mut self, alignment: HAlignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    #[must_use]
    pub fn with_spacing(mut self, spacing: u16) -> Self {
        self.spacing = Some(spacing);
        self
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_widget(self) -> Widget {
        Widget(serde_json::to_value(self).unwrap())
    }
}
impl From<Column> for Widget {
    fn from(inner: Column) -> Self {
        Widget(serde_json::to_value(inner).unwrap())
    }
}

#[derive(Clone, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
pub struct DetailCell {
    #[serde(skip_deserializing)]
    typ: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    actions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo: Option<String>,
    text: String,
}
impl DetailCell {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(text: impl ToString) -> Self {
        Self {
            typ: "detail-cell",
            actions: Vec::new(),
            photo: None,
            text: text.to_string(),
        }
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_action(mut self, action: impl ToString) -> Self {
        self.actions.push(action.to_string());
        self
    }

    #[must_use]
    pub fn with_actions(mut self, actions: impl IntoIterator<Item = impl ToString>) -> Self {
        self.actions
            .extend(actions.into_iter().map(|action| action.to_string()));
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_photo(mut self, path: impl ToString) -> Self {
        self.photo = Some(path.to_string());
        self
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_widget(self) -> Widget {
        Widget(serde_json::to_value(self).unwrap())
    }
}
impl From<DetailCell> for Widget {
    fn from(inner: DetailCell) -> Self {
        Widget(serde_json::to_value(inner).unwrap())
    }
}

#[derive(Clone, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
pub struct Empty {}
impl Empty {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget(json!({"typ": "empty"}))
    }
}
impl From<Empty> for Widget {
    fn from(inner: Empty) -> Self {
        Widget(serde_json::to_value(inner).unwrap())
    }
}

#[must_use]
pub fn section_heading(text: impl AsRef<str>) -> Value {
    json!({"typ": "section-heading", "text": text.as_ref().to_string()})
}

#[derive(Clone, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
pub struct Text {
    #[serde(skip_deserializing)]
    typ: &'static str,
    text: String,
}
impl Text {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(text: impl ToString) -> Self {
        Self {
            typ: "text",
            text: text.to_string(),
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_widget(self) -> Widget {
        Widget(serde_json::to_value(self).unwrap())
    }
}
impl From<Text> for Widget {
    fn from(inner: Text) -> Self {
        Widget(serde_json::to_value(inner).unwrap())
    }
}
