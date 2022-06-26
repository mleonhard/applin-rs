use crate::builder::BackButton;
use crate::page_enum::Page;
use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavPage {
    title: String,
    widget: Widget,
    start: Option<Widget>,
    end: Option<Widget>,
}
impl NavPage {
    #[must_use]
    pub fn new(title: impl Into<String>, widget: impl Into<Widget>) -> Self {
        Self {
            title: title.into(),
            widget: widget.into(),
            start: None,
            end: None,
        }
    }

    #[must_use]
    pub fn without_back(mut self) -> Self {
        self.start = Some(Widget::Empty);
        self
    }

    #[must_use]
    pub fn with_start(mut self, back_button: BackButton) -> Self {
        self.start = Some(back_button.into());
        self
    }

    #[must_use]
    pub fn with_end(mut self, widget: impl Into<Widget>) -> Self {
        self.end = Some(widget.into());
        self
    }

    #[must_use]
    pub fn to_page(self) -> Page {
        Page::Nav {
            title: self.title,
            widget: self.widget,
            start: self.start,
            end: self.end,
        }
    }
}
impl From<NavPage> for Page {
    fn from(src: NavPage) -> Self {
        src.to_page()
    }
}
