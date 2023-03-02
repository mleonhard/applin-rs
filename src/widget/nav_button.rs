use crate::internal::{Action, Widget};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NavButton {
    actions: Vec<Action>,
    badge_text: Option<String>,
    photo_url: Option<String>,
    sub_text: Option<String>,
    text: String,
}
impl NavButton {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            actions: vec![],
            badge_text: None,
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
    pub fn with_badge_text(mut self, badge: impl Into<String>) -> Self {
        self.badge_text = Some(badge.into());
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
    pub fn to_widget(self) -> Widget {
        Widget::NavButtonVariant {
            actions: self.actions,
            badge_text: self.badge_text,
            photo_url: self.photo_url,
            sub_text: self.sub_text,
            text: self.text,
        }
    }
}
impl From<NavButton> for Widget {
    fn from(src: NavButton) -> Self {
        src.to_widget()
    }
}
impl From<NavButton> for Option<Widget> {
    fn from(src: NavButton) -> Self {
        Some(src.to_widget())
    }
}
