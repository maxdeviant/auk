use crate::{HtmlElement, TextElement};

/// A DOM element.
#[derive(Debug, Clone)]
pub enum Element {
    /// A text element.
    Text(TextElement),

    /// An HTML element.
    Html(HtmlElement),
}
