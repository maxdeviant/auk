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

impl From<HtmlElement> for Element {
    fn from(value: HtmlElement) -> Self {
        Self::Html(value)
    }
}
