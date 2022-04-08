use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
pub struct Button {
    #[serde(skip_deserializing)]
    typ: &'static str,
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
    pub fn build(self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
impl From<Button> for Value {
    fn from(button: Button) -> Self {
        serde_json::to_value(button).unwrap()
    }
}

#[derive(Clone, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
pub struct DetailCell {
    #[serde(skip_deserializing)]
    typ: &'static str,
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
    pub fn build(self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
impl From<DetailCell> for Value {
    fn from(detail_cell: DetailCell) -> Self {
        serde_json::to_value(detail_cell).unwrap()
    }
}

#[must_use]
pub fn section_heading(text: impl AsRef<str>) -> Value {
    json!({"typ": "section-heading", "text": text.as_ref().to_string()})
}

#[must_use]
pub fn text(text: impl AsRef<str>) -> Value {
    Value::String(text.as_ref().to_string())
}

#[derive(Clone, Deserialize, Eq, Hash, Serialize, Ord, PartialEq, PartialOrd)]
pub struct TitleBar {
    #[serde(skip_deserializing)]
    typ: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    end_actions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_text: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    start_actions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_text: Option<String>,
    text: String,
}
impl TitleBar {
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(text: impl ToString) -> Self {
        Self {
            typ: "title-bar",
            start_actions: Vec::new(),
            start_text: None,
            end_actions: Vec::new(),
            end_text: None,
            text: text.to_string(),
        }
    }

    #[must_use]
    pub fn with_back(mut self) -> Self {
        self.start_text = Some("Back".to_string());
        self.start_actions = vec!["pop".to_string()];
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_end(mut self, text: impl ToString) -> Self {
        self.end_text = Some(text.to_string());
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_start(mut self, text: impl ToString) -> Self {
        self.start_text = Some(text.to_string());
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_end_action(mut self, action: impl ToString) -> Self {
        self.end_actions.push(action.to_string());
        self
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_start_action(mut self, action: impl ToString) -> Self {
        self.start_actions.push(action.to_string());
        self
    }

    #[must_use]
    pub fn with_end_actions(mut self, actions: impl IntoIterator<Item = impl ToString>) -> Self {
        self.end_actions
            .extend(actions.into_iter().map(|action| action.to_string()));
        self
    }

    #[must_use]
    pub fn with_start_actions(mut self, actions: impl IntoIterator<Item = impl ToString>) -> Self {
        self.start_actions
            .extend(actions.into_iter().map(|action| action.to_string()));
        self
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn build(self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
impl From<TitleBar> for Value {
    fn from(title_bar: TitleBar) -> Self {
        serde_json::to_value(title_bar).unwrap()
    }
}
