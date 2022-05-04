use crate::action::Action;
use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Button {
    text: String,
    actions: Vec<Action>,
}
impl Button {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
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
        Widget::Button {
            text: self.text,
            actions: self.actions,
        }
    }
}
impl From<Button> for Widget {
    fn from(src: Button) -> Self {
        src.to_widget()
    }
}
