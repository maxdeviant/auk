use crate::{HtmlElement, TextElement};

/// A DOM element.
#[derive(Debug)]
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

impl From<String> for Element {
    fn from(value: String) -> Self {
        Self::Text(TextElement::new(value))
    }
}

impl From<&str> for Element {
    fn from(value: &str) -> Self {
        Self::Text(TextElement::new(value))
    }
}

impl From<HtmlElement> for Element {
    fn from(value: HtmlElement) -> Self {
        Self::Html(value)
    }
}
