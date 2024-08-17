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

impl From<String> for Element {
    fn from(value: String) -> Self {
        Self::Text(TextElement::from(value))
    }
}

impl From<&String> for Element {
    fn from(value: &String) -> Self {
        Self::Text(TextElement::from(value))
    }
}

impl From<&str> for Element {
    fn from(value: &str) -> Self {
        Self::Text(TextElement::from(value))
    }
}

impl<T> From<T> for Element
where
    T: Into<HtmlElement>,
{
    fn from(value: T) -> Self {
        Self::Html(value.into())
    }
}
