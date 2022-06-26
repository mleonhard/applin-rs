use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Empty;
impl Empty {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::Empty {}
    }
}
impl From<Empty> for Widget {
    fn from(src: Empty) -> Self {
        src.to_widget()
    }
}
