use crate::{HtmlElement, TextElement};

/// A DOM element.
#[derive(Debug, Clone)]
pub enum Element {
    /// A text element.
    Text(TextElement),

    /// An HTML element.
    Html(HtmlElement),
}

impl Element {
    /// Converts this [`Element`] into a [`TextElement`].
    pub fn text(self) -> Option<TextElement> {
        match self {
            Self::Text(text) => Some(text),
            Self::Html(_) => None,
        }
    }

    /// Converts this [`Element`] into an [`HtmlElement`].
    pub fn html(self) -> Option<HtmlElement> {
        match self {
            Self::Html(html) => Some(html),
            Self::Text(_) => None,
        }
    }
}
