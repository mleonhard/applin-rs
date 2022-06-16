use crate::action::Action;
use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Checkbox {
    actions: Vec<Action>,
    id: String,
    initial_bool: bool,
}
impl Checkbox {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            actions: Vec::new(),
            id: id.into(),
            initial_bool: false,
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
    pub fn with_initial_value(mut self, value: bool) -> Self {
        self.initial_bool = value;
        self
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::CheckBox {
            actions: self.actions,
            id: self.id,
            initial_bool: self.initial_bool,
        }
    }
}
impl From<Checkbox> for Widget {
    fn from(src: Checkbox) -> Self {
        src.to_widget()
    }
}
