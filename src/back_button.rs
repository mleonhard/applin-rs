use crate::action::Action;
use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BackButton {
    actions: Vec<Action>,
}
impl BackButton {
    /// Makes a `back-button` widget that does nothing.
    /// To make the button work as expected, call `with_action(Action::Pop)`.
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
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
    pub fn to_widget(self) -> Widget {
        Widget::BackButton {
            actions: self.actions,
        }
    }
}
impl From<BackButton> for Widget {
    fn from(src: BackButton) -> Self {
        src.to_widget()
    }
}
