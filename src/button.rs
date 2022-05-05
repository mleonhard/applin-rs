use crate::action::Action;
use crate::action_builders::pop;
use crate::widget_enum::Widget;

#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn ok_button() -> Widget {
    Button::new("OK")
        .with_action(pop())
        .with_is_default()
        .to_widget()
}

#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn cancel_button() -> Widget {
    Button::new("Cancel").with_action(pop()).to_widget()
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Button {
    text: String,
    actions: Vec<Action>,
    is_default: bool,
    is_destructive: bool,
}
impl Button {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            actions: Vec::new(),
            is_default: false,
            is_destructive: false,
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
