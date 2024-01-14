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

impl From<HtmlElement> for Element {
    fn from(value: HtmlElement) -> Self {
        Self::Html(value)
    }
}
