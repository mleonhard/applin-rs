use crate::internal::{Widget, WidgetList};

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
    pub fn to_widget(self) -> Widget {
        Widget::FormVariant {
            widgets: self.widgets,
        }
    }
}
impl From<Form> for Widget {
    fn from(src: Form) -> Self {
        src.to_widget()
    }
}
impl From<Form> for Option<Widget> {
    fn from(src: Form) -> Self {
        Some(src.to_widget())
    }
}
