use crate::builder::ModalButton;
use crate::page_enum::Page;
use crate::widget_enum::Widget;
use crate::widget_list::WidgetList;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlertModal {
    poll_seconds: u32,
    stream: bool,
    text: Option<String>,
    title: String,
    widgets: Vec<Widget>,
}
impl AlertModal {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            poll_seconds: 0,
            stream: false,
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
    pub fn with_poll(mut self, seconds: u32) -> Self {
        self.poll_seconds = seconds;
        self
    }

    #[must_use]
    pub fn with_stream(mut self) -> Self {
        self.stream = true;
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
            poll_seconds: if self.stream { 0 } else { self.poll_seconds },
            stream: self.stream,
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
