//! Constructs for rendering [`HtmlElement`]s to HTML.

use std::fmt::Write;

use crate::visitor::Visitor;
use crate::HtmlElement;

/// A renderer for [`HtmlElement`]s to a string of HTML.
pub struct HtmlElementRenderer {
    html: String,
}

impl HtmlElementRenderer {
    /// Returns a new [`HtmlElementRenderer`].
    pub fn new() -> Self {
        Self {
            html: String::new(),
        }
    }

    /// Renders the given [`HtmlElement`] to a string of HTML.
    pub fn render_to_string(mut self, element: &HtmlElement) -> Result<String, std::fmt::Error> {
        self.visit(element)?;

        Ok(self.html)
    }
}

impl Visitor for HtmlElementRenderer {
    type Error = std::fmt::Error;

    fn visit(&mut self, element: &HtmlElement) -> Result<(), Self::Error> {
        if element.tag_name == HtmlElement::RAW_TEXT_TAG {
            write!(&mut self.html, "{}", element.content.as_ref().unwrap())?;
            return Ok(());
        }

        if element.tag_name == "html" {
            write!(&mut self.html, "<!DOCTYPE html>")?;
        }

        write!(&mut self.html, "<{}", element.tag_name)?;

        for (name, value) in &element.attrs {
            self.visit_attr(name, value)?;
        }

        write!(&mut self.html, ">")?;

        if element.is_void() {
            return Ok(());
        }

        if let Some(content) = &element.content {
            write!(&mut self.html, "{}", content)?;
        }

        self.visit_children(&element.children)?;

        write!(&mut self.html, "</{}>", element.tag_name)?;

        Ok(())
    }

    fn visit_attr(&mut self, name: &str, value: &str) -> Result<(), Self::Error> {
        write!(&mut self.html, " ")?;
        write!(&mut self.html, r#"{name}="{value}""#)?;

        Ok(())
    }
}
