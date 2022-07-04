use crate::action::pop;
use crate::internal::{Action, Widget};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModalButton {
    actions: Vec<Action>,
    is_cancel: bool,
    is_default: bool,
    is_destructive: bool,
    text: String,
}
impl ModalButton {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            actions: Vec::new(),
            is_cancel: false,
            is_default: false,
            is_destructive: false,
            text: text.into(),
        }
    }

    #[must_use]
    pub fn cancel() -> Widget {
        Self::new("Cancel").with_action(pop()).to_widget()
    }

    #[must_use]
    pub fn ok() -> Widget {
        Self::new("OK")
            .with_action(pop())
            .with_is_default()
            .to_widget()
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
    pub fn with_is_cancel(mut self) -> Self {
        self.is_cancel = true;
        self
    }

    #[must_use]
    pub fn with_is_default(mut self) -> Self {
        self.is_default = true;
        self
    }

    #[must_use]
    pub fn with_is_destructive(mut self) -> Self {
        self.is_destructive = true;
        self
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::ModalButtonVariant {
            actions: self.actions,
            is_cancel: self.is_cancel,
            is_default: self.is_default,
            is_destructive: self.is_destructive,
            text: self.text,
        }
    }
}
impl From<ModalButton> for Widget {
    fn from(src: ModalButton) -> Self {
        src.to_widget()
    }
}
