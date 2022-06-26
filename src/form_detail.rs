use crate::action::Action;
use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormDetail {
    actions: Vec<Action>,
    photo_url: Option<String>,
    sub_text: Option<String>,
    text: String,
}
impl FormDetail {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            actions: vec![],
            photo_url: None,
            sub_text: None,
            text: text.into(),
        }
    }

    /// Appends `action`.
    #[must_use]
    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    /// Appends `actions`.
    #[must_use]
    pub fn with_actions(mut self, actions: impl IntoIterator<Item = Action>) -> Self {
        self.actions.extend(actions.into_iter());
        self
    }

    #[must_use]
    pub fn with_photo_url(mut self, url: impl Into<String>) -> Self {
        self.photo_url = Some(url.into());
        self
    }

    #[must_use]
    pub fn with_sub_text(mut self, sub_text: impl Into<String>) -> Self {
        self.sub_text = Some(sub_text.into());
        self
    }

    #[must_use]
    pub fn into_widget(self) -> Widget {
        Widget::FormDetail {
            actions: self.actions,
            photo_url: self.photo_url,
            sub_text: self.sub_text,
            text: self.text,
        }
    }
}
impl From<FormDetail> for Widget {
    fn from(src: FormDetail) -> Self {
        src.into_widget()
    }
}
