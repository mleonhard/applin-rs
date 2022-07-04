use crate::internal::{Action, Widget};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormButton {
    actions: Vec<Action>,
    is_destructive: bool,
    text: String,
}
impl FormButton {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            actions: Vec::new(),
            is_destructive: false,
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
    pub fn with_is_destructive(mut self) -> Self {
        self.is_destructive = true;
        self
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::FormButtonVariant {
            actions: self.actions,
            is_destructive: self.is_destructive,
            text: self.text,
        }
    }
}
impl From<FormButton> for Widget {
    fn from(src: FormButton) -> Self {
        src.to_widget()
    }
}
