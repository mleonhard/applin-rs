use crate::h_alignment::HAlignment;
use crate::widget_enum::Widget;
use crate::widget_list::WidgetList;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Column {
    widgets: Vec<Widget>,
    h_alignment: HAlignment,
    spacing: u16,
}
impl Column {
    /// Makes an empty `column` widget with horizontal alignment `start` and spacing `0`.
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new(widgets: impl Into<WidgetList>) -> Self {
        Self {
            widgets: widgets.into().0,
            h_alignment: HAlignment::Start,
            spacing: 0,
        }
    }

    /// Appends `widget`.
    #[must_use]
    pub fn with_widget(mut self, widget: impl Into<Widget>) -> Self {
        self.widgets.push(widget.into());
        self
    }

    /// Appends `widgets`.
    #[must_use]
    pub fn with_widgets(mut self, widgets: impl Into<WidgetList>) -> Self {
        self.widgets.extend(widgets.into().0);
        self
    }

    #[must_use]
    pub fn with_spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    #[must_use]
    pub fn into_widget(self) -> Widget {
        Widget::Column {
            widgets: self.widgets,
            h_alignment: self.h_alignment,
            spacing: self.spacing,
        }
    }
}
impl From<Column> for Widget {
    fn from(src: Column) -> Self {
        src.into_widget()
    }
}
