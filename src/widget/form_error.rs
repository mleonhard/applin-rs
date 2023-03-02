use crate::internal::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ErrorText {
    text: String,
}
impl ErrorText {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::ErrorTextVariant { text: self.text }
    }
}
impl From<ErrorText> for Widget {
    fn from(src: ErrorText) -> Self {
        src.to_widget()
    }
}
impl From<ErrorText> for Option<Widget> {
    fn from(src: ErrorText) -> Self {
        Some(src.to_widget())
    }
}
