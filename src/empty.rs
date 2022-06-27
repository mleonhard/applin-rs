use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Empty;
impl Empty {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    #[allow(clippy::unused_self)]
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
