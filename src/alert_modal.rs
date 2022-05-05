use crate::page_enum::Page;
use crate::widget_enum::Widget;
use crate::widget_list::WidgetList;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlertModal {
    title: String,
    widgets: Vec<Widget>,
}
impl AlertModal {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
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
    pub fn to_page(self) -> Page {
        Page::Alert {
            title: self.title,
            widgets: self.widgets,
        }
    }
}
impl From<AlertModal> for Page {
    fn from(src: AlertModal) -> Self {
        src.to_page()
    }
}
