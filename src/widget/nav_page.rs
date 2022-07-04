use crate::internal::{Page, Widget};
use crate::widget::BackButton;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavPage {
    end: Option<Widget>,
    poll_seconds: u32,
    start: Option<Widget>,
    stream: bool,
    title: String,
    widget: Widget,
}
impl NavPage {
    #[must_use]
    pub fn new(title: impl Into<String>, widget: impl Into<Widget>) -> Self {
        Self {
            end: None,
            poll_seconds: 0,
            start: None,
            stream: false,
            title: title.into(),
            widget: widget.into(),
        }
    }

    #[must_use]
    pub fn without_back(mut self) -> Self {
        self.start = Some(Widget::Empty);
        self
    }

    #[must_use]
    pub fn with_end(mut self, widget: impl Into<Widget>) -> Self {
        self.end = Some(widget.into());
        self
    }

    #[must_use]
    pub fn with_poll(mut self, seconds: u32) -> Self {
        self.poll_seconds = seconds;
        self
    }

    #[must_use]
    pub fn with_start(mut self, back_button: BackButton) -> Self {
        self.start = Some(back_button.into());
        self
    }

    #[must_use]
    pub fn with_stream(mut self) -> Self {
        self.stream = true;
        self
    }

    #[must_use]
    pub fn to_page(self) -> Page {
        Page::Nav {
            end: self.end,
            poll_seconds: if self.stream { 0 } else { self.poll_seconds },
            start: self.start,
            stream: self.stream,
            title: self.title,
            widget: self.widget,
        }
    }
}
impl From<NavPage> for Page {
    fn from(src: NavPage) -> Self {
        src.to_page()
    }
}
