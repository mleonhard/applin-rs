use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Scroll {
    widget: Widget,
}
impl Scroll {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new(widget: impl Into<Widget>) -> Self {
        Self {
            widget: widget.into(),
        }
    }

    #[must_use]
    pub fn into_widget(self) -> Widget {
        Widget::Scroll {
            widget: Box::new(self.widget),
        }
    }
}
impl From<Scroll> for Widget {
    fn from(src: Scroll) -> Self {
        src.into_widget()
    }
}