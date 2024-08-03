use crate::{HtmlElement, TextElement};

/// A DOM element.
#[derive(Debug, Clone)]
pub enum Element {
    /// A text element.
    Text(TextElement),

    /// An HTML element.
    Html(HtmlElement),
}

impl From<TextElement> for Element {
    fn from(value: TextElement) -> Self {
        Self::Text(value)
    }
}

impl<T: Into<String>> From<T> for Element {
    fn from(value: T) -> Self {
        Self::Text(TextElement::new(value))
    }
}

impl From<HtmlElement> for Element {
    fn from(value: HtmlElement) -> Self {
        Self::Html(value)
    }
}
