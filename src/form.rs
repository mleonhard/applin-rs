use crate::widget_enum::Widget;
use crate::widget_list::WidgetList;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Form {
    widgets: Vec<Widget>,
}
impl Form {
    /// Makes a `form` widget.
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new(widgets: impl Into<WidgetList>) -> Self {
        Self {
            widgets: widgets.into().0,
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
    pub fn into_widget(self) -> Widget {
        Widget::Form {
            widgets: self.widgets,
        }
    }
}
impl From<Form> for Widget {
    fn from(src: Form) -> Self {
        src.into_widget()
    }
}
