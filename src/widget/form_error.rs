use crate::internal::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormError {
    text: String,
}
impl FormError {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::FormErrorVariant { text: self.text }
    }
}
impl From<FormError> for Widget {
    fn from(src: FormError) -> Self {
        src.to_widget()
    }
}
