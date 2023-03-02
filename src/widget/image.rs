use crate::internal::{ImageDisposition, Widget};
use crate::widget::Real32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image {
    aspect_ratio: Real32,
    disposition: ImageDisposition,
    url: String,
}
impl Image {
    /// Creates an image with `Fit` disposition.
    ///
    /// # Panics
    /// Panics when `aspect_ratio` is infinite, zero, negative, or NaN.
    #[must_use]
    pub fn new(aspect_ratio_width_over_height: f32, url: impl Into<String>) -> Self {
        let aspect_ratio = Real32::new(aspect_ratio_width_over_height);
        let url = url.into();
        Self {
            aspect_ratio,
            disposition: ImageDisposition::Fit,
            url,
        }
    }

    #[must_use]
    pub fn with_disposition(mut self, disposition: ImageDisposition) -> Self {
        self.disposition = disposition;
        self
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::ImageVariant {
            aspect_ratio: self.aspect_ratio,
            disposition: self.disposition,
            url: self.url,
        }
    }
}
impl From<Image> for Widget {
    fn from(src: Image) -> Self {
        src.to_widget()
    }
}
impl From<Image> for Option<Widget> {
    fn from(src: Image) -> Self {
        Some(src.to_widget())
    }
}
