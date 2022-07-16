use crate::internal::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormCheckbox {
    id: String,
    initial: bool,
    rpc: Option<String>,
    text: String,
}
impl FormCheckbox {
    /// # Panics
    /// Panics when `id` is empty.
    #[must_use]
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        let id = id.into();
        assert!(!id.is_empty());
        Self {
            id,
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
        Widget::FormCheckboxVariant {
            id: self.id,
            initial_bool: self.initial,
            rpc: self.rpc,
            text: self.text,
        }
    }
}
impl From<FormCheckbox> for Widget {
    fn from(src: FormCheckbox) -> Self {
        src.to_widget()
    }
}
