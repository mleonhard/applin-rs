use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Text {
    text: String,
}
impl Text {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::Text { text: self.text }
    }
}
impl From<Text> for Widget {
    fn from(src: Text) -> Self {
        src.to_widget()
    }
}