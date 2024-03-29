use crate::internal::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Checkbox {
    var: String,
    initial: bool,
    rpc: Option<String>,
    text: String,
}
impl Checkbox {
    /// # Panics
    /// Panics when `id` is empty.
    #[must_use]
    pub fn new(var: impl Into<String>, text: impl Into<String>) -> Self {
        let var = var.into();
        assert!(!var.is_empty());
        Self {
            var,
            initial: false,
            rpc: None,
            text: text.into(),
        }
    }

    #[must_use]
    pub fn with_rpc(mut self, rpc: impl Into<String>) -> Self {
        self.rpc = Some(rpc.into());
        self
    }

    #[must_use]
    pub fn with_initial(mut self, checked: bool) -> Self {
        self.initial = checked;
        self
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::CheckboxVariant {
            var: self.var,
            initial_bool: self.initial,
            rpc: self.rpc,
            text: self.text,
        }
    }
}
impl From<Checkbox> for Widget {
    fn from(src: Checkbox) -> Self {
        src.to_widget()
    }
}
impl From<Checkbox> for Option<Widget> {
    fn from(src: Checkbox) -> Self {
        Some(src.to_widget())
    }
}
