use crate::internal::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ErrorDetails;
impl ErrorDetails {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::ErrorDetailsVariant {}
    }
}
impl From<ErrorDetails> for Widget {
    fn from(src: ErrorDetails) -> Self {
        src.to_widget()
    }
}
impl From<ErrorDetails> for Option<Widget> {
    fn from(src: ErrorDetails) -> Self {
        Some(src.to_widget())
    }
}
