use crate::page_enum::Page;
use crate::widget_enum::Widget;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlainPage {
    title: String,
    widget: Widget,
}
impl PlainPage {
    #[must_use]
    pub fn new(title: impl Into<String>, widget: impl Into<Widget>) -> Self {
        Self {
            title: title.into(),
            widget: widget.into(),
        }
    }

    #[must_use]
    pub fn to_page(self) -> Page {
        Page::Plain {
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
