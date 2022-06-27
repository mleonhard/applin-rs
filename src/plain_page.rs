use crate::page_enum::Page;
use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlainPage {
    poll_seconds: u32,
    stream: bool,
    title: String,
    widget: Widget,
}
impl PlainPage {
    #[must_use]
    pub fn new(title: impl Into<String>, widget: impl Into<Widget>) -> Self {
        Self {
            poll_seconds: 0,
            stream: false,
            title: title.into(),
            widget: widget.into(),
        }
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
    pub fn to_page(self) -> Page {
        Page::Plain {
            poll_seconds: if self.stream { 0 } else { self.poll_seconds },
            stream: self.stream,
            title: self.title,
            widget: self.widget,
        }
    }
}
impl From<PlainPage> for Page {
    fn from(src: PlainPage) -> Self {
        src.to_page()
    }
}
