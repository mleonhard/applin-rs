use crate::action::Action;
use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DetailCell {
    text: String,
    actions: Vec<Action>,
    photo_url: Option<String>,
}
impl DetailCell {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            actions: vec![],
            photo_url: None,
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
    pub fn into_widget(self) -> Widget {
        Widget::DetailCell {
            text: self.text,
            actions: self.actions,
            photo_url: self.photo_url,
        }
    }
}
impl From<DetailCell> for Widget {
    fn from(src: DetailCell) -> Self {
        src.into_widget()
    }
}
