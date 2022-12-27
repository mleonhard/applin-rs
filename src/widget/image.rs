use crate::internal::Widget;
use crate::widget::Real32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image {
    aspect_ratio: Real32,
    url: String,
}
impl Image {
    /// Creates an image.
    ///
    /// # Panics
    /// Panics when `aspect_ratio` is infinite, zero, negative, or NaN.
    #[must_use]
    pub fn new(aspect_ratio: f32, url: impl Into<String>) -> Self {
        let aspect_ratio = Real32::new(aspect_ratio);
        let url = url.into();
        Self { aspect_ratio, url }
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::ImageVariant {
            aspect_ratio: self.aspect_ratio,
            url: self.url,
        }
    }
}
impl From<Image> for Widget {
    fn from(src: Image) -> Self {
        src.to_widget()
    }
}
