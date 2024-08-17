#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod element;
pub mod renderer;
pub mod visitor;

use std::iter;

use indexmap::IndexMap;

pub use crate::element::*;

/// An HTML element.
#[derive(Debug, Clone)]
pub struct HtmlElement {
    /// The tag name for this element.
    pub tag_name: String,

    /// The attributes of this element.
    pub attrs: IndexMap<String, String>,

    /// The child nodes of this element.
    pub children: Vec<Element>,
}

impl HtmlElement {
    /// Returns a new [`HtmlElement`] with the given tag name.
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag_name: tag.into(),
            attrs: IndexMap::new(),
            children: Vec::new(),
        }
    }

    /// Returns whether this element is a [void element](https://developer.mozilla.org/en-US/docs/Glossary/Void_element).
    pub fn is_void(&self) -> bool {
        match self.tag_name.as_str() {
            "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta"
            | "param" | "source" | "track" | "wbr" => true,
            _ => false,
        }
    }

    /// Sets the specified attribute on this element.
    ///
    /// Will overwrite the existing value for the attribute, if one exists.
    pub fn attr<V>(mut self, name: impl Into<String>, value: impl Into<Option<V>>) -> Self
    where
        V: Into<String>,
    {
        let name = name.into();
        match value.into() {
            Some(id) => {
                *self.attrs.entry(name).or_default() = id.into();
            }
            None => {
                self.attrs.remove(&name);
            }
        }

        self
    }
}

/// A trait for elements that can be modified.
pub trait With {
    /// Applies the given closure to modify the element and return a new copy.
    #[inline(always)]
    fn with(self, f: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        f(self)
    }
}

impl With for HtmlElement {}

/// A trait for elements that can have children.
pub trait WithChildren {
    /// Extends this element's children with the given children.
    fn extend(&mut self, children: impl IntoIterator<Item = Element>);

    /// Adds a new child element to this element.
    fn child(mut self, child: impl Into<Element>) -> Self
    where
        Self: Sized,
    {
        self.extend(iter::once(child.into()));
        self
    }

    /// Adds the specified child elements to this element.
    fn children(mut self, children: impl IntoIterator<Item = impl Into<Element>>) -> Self
    where
        Self: Sized,
    {
        self.extend(children.into_iter().map(Into::into));
        self
    }
}

impl WithChildren for HtmlElement {
    #[inline(always)]
    fn extend(&mut self, children: impl IntoIterator<Item = Element>) {
        self.children.extend(children)
    }
}

/// A text element.
#[derive(Debug, Clone)]
pub struct TextElement {
    /// The text content of this element.
    pub text: String,
}

impl TextElement {
    /// Returns a new [`TextElement`] with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

macro_rules! create_attribute_methods {
    ($($name:ident),*) => {
        $(
            #[doc = concat!("Sets the `", stringify!($name), "` attribute to the provided value.")]
            pub fn $name<V>(self, value: impl Into<Option<V>>) -> Self
            where
                V: Into<String>,
            {
                self.attr(stringify!($name), value)
            }
        )*
    }
}

impl HtmlElement {
    create_attribute_methods!(
        alt,
        charset,
        class,
        content,
        crossorigin,
        defer,
        href,
        id,
        lang,
        loading,
        max,
        name,
        rel,
        role,
        src,
        start,
        style,
        tabindex,
        target,
        title,
        translate,
        value
    );

    /// Sets the `async` attribute to the provided value.
    pub fn async_<V>(self, value: impl Into<Option<V>>) -> Self
    where
        V: Into<String>,
    {
        self.attr("async", value)
    }

    /// Sets the `for` attribute to the provided value.
    pub fn for_<V>(self, value: impl Into<Option<V>>) -> Self
    where
        V: Into<String>,
    {
        self.attr("for", value)
    }

    /// Sets the `http-equiv` attribute to the provided value.
    pub fn http_equiv<V>(self, value: impl Into<Option<V>>) -> Self
    where
        V: Into<String>,
    {
        self.attr("http-equiv", value)
    }

    /// Sets the `type` attribute to the provided value.
    pub fn type_<V>(self, value: impl Into<Option<V>>) -> Self
    where
        V: Into<String>,
    {
        self.attr("type", value)
    }
}

macro_rules! html_elements {
    ($($name:ident),*) => {
        $(
            #[doc = concat!("[`<", stringify!($name), ">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($name), ")")]
            pub fn $name() -> HtmlElement {
                HtmlElement::new(stringify!($name))
            }
        )*
    }
}

html_elements!(
    a, abbr, address, area, article, aside, audio, b, base, bdi, bdo, blockquote, body, br, button,
    canvas, caption, cite, code, col, colgroup, data, datalist, dd, del, dfn, div, dl, dt, em,
    embed, fieldset, figcaption, figure, footer, form, h1, h2, h3, h4, h5, h6, head, header,
    hgroup, hr, html, i, iframe, img, input, ins, kbd, label, legend, li, link, main, map, mark,
    math, menu, meta, meter, nav, noscript, object, ol, optgroup, option, output, p, picture,
    portal, pre, progress, q, rp, rt, ruby, s, samp, script, search, section, select, small,
    source, span, strong, style, sub, sup, svg, table, tbody, td, textarea, tfoot, th, thead, time,
    title, tr, track, u, ul, var, video, wbr, details, dialog, summary, slot, template
);

#[cfg(test)]
mod tests {
    use crate::renderer::HtmlElementRenderer;

    use super::*;

    fn render_to_string(element: &HtmlElement) -> String {
        HtmlElementRenderer::new()
            .render_to_string(element)
            .unwrap()
    }

    #[test]
    fn test_new_html_element() {
        let element = HtmlElement::new("custom");

        assert_eq!(element.tag_name, "custom".to_owned());
    }

    #[test]
    fn test_attributes() {
        let element = a().attr("foo", "a").attr("bar", "b");

        assert_eq!(element.attrs.get("foo"), Some(&"a".to_string()));
        assert_eq!(element.attrs.get("bar"), Some(&"b".to_string()));
    }

    #[test]
    fn test_render_to_string() {
        let element = div().class("outer").child(
            div()
                .class("inner")
                .child(h1().class("heading").child("Hello, world!")),
        );

        insta::assert_yaml_snapshot!(render_to_string(&element));
    }

    #[test]
    fn test_doctype_auto_insertion() {
        insta::assert_yaml_snapshot!(render_to_string(&html()));
    }

    #[test]
    fn test_raw_text() {
        insta::assert_yaml_snapshot!(render_to_string(
            &p().child("This is a ")
                .child(a().href("https://example.com").child("link"))
                .child(" that you should click on.")
        ))
    }

    #[test]
    fn test_empty_attributes() {
        insta::assert_yaml_snapshot!(render_to_string(
            &script()
                .async_("")
                .defer("")
                .attr("data-domain", "example.com")
                .src("https://plausible.io/js/plausible.js"),
        ));
    }

    #[test]
    fn test_crossorigin_attr() {
        insta::assert_yaml_snapshot!(render_to_string(
            &link()
                .rel("preconnect")
                .href("https://fonts.gstatic.com")
                .crossorigin("")
        ))
    }
}
