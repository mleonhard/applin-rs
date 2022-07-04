use crate::internal::{Widget, WidgetList};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormSection {
    title: Option<String>,
    widgets: Vec<Widget>,
}
impl FormSection {
    /// Makes a `form-section` widget.
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            title: None,
            widgets: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
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
        Widget::FormSection {
            title: self.title,
            widgets: self.widgets,
        }
    }
}
impl From<FormSection> for Widget {
    fn from(src: FormSection) -> Self {
        src.to_widget()
    }
}
