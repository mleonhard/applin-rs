use crate::internal::{Page, Widget, WidgetList};
use crate::widget::ModalButton;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlertModal {
    text: Option<String>,
    title: String,
    widgets: Vec<Widget>,
}
impl AlertModal {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            text: None,
            title: title.into(),
            widgets: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_ok(mut self) -> Self {
        self.widgets.push(ModalButton::ok());
        self
    }

    #[must_use]
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
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
    pub fn to_page(self) -> Page {
        Page::Alert {
            text: self.text,
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
