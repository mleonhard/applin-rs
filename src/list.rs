use crate::widget_enum::Widget;
use crate::widget_list::WidgetList;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct List {
    title: Option<String>,
    widgets: Vec<Widget>,
}
impl List {
    /// Makes a new `list` empty widget with no title.
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            title: None,
            widgets: Vec::new(),
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
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn into_widget(self) -> Widget {
        Widget::List {
            title: self.title,
            widgets: self.widgets,
        }
    }
}
impl From<List> for Widget {
    fn from(src: List) -> Self {
        src.into_widget()
    }
}
