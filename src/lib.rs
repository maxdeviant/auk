#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod renderer;
pub mod visitor;

use indexmap::IndexMap;

use crate::renderer::HtmlElementRenderer;

/// An HTML element.
#[derive(Debug)]
pub struct HtmlElement {
    /// The tag name for this element.
    tag_name: String,

    /// The attributes of this element.
    attrs: IndexMap<String, String>,

    /// The rendered text content of this element.
    content: Option<String>,

    /// The child nodes of this element.
    children: Vec<HtmlElement>,
}

impl HtmlElement {
    const RAW_TEXT_TAG: &'static str = "__RAW_TEXT__";

    /// Returns a new [`HtmlElement`] with the given tag name.
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag_name: tag.into(),
            attrs: IndexMap::new(),
            content: None,
            children: Vec::new(),
        }
    }

    // TODO: Find a better way of representing raw text without modeling it as an `HtmlElement`.
    #[doc(hidden)]
    pub fn unstable_raw_text(text: impl Into<String>) -> Self {
        Self {
            tag_name: Self::RAW_TEXT_TAG.into(),
            attrs: IndexMap::new(),
            content: Some(text.into()),
            children: Vec::new(),
        }
    }

    /// Returns whether this element is a [void element](https://developer.mozilla.org/en-US/docs/Glossary/Void_element).
    fn is_void(&self) -> bool {
        match self.tag_name.as_str() {
            "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta"
            | "param" | "source" | "track" | "wbr" => true,
            _ => false,
        }
    }

    /// Sets the specified attribute on this element.
    ///
    /// Will overwrite the existing value for the attribute, if one exists.
    fn attr<V>(mut self, name: impl Into<String>, value: impl Into<Option<V>>) -> Self
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

    /// Sets the rendered text for this element.
    pub fn text_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Returns a mutable reference to the rendered text for this element.
    pub fn text_content_mut(&mut self) -> &mut Option<String> {
        &mut self.content
    }

    /// Renders this element to an HTML string.
    #[deprecated(note = "Use `HtmlElementRenderer` directly.")]
    pub fn render_to_string(&self) -> Result<String, std::fmt::Error> {
        HtmlElementRenderer::new().render_to_string(self)
    }
}

/// A trait for elements that can have children.
pub trait WithChildren {
    /// Returns a mutable reference to this element's children.
    fn children_mut(&mut self) -> &mut Vec<HtmlElement>;

    /// Adds a new child element to this element.
    fn child(mut self, child: HtmlElement) -> Self
    where
        Self: Sized,
    {
        self.children_mut().push(child);
        self
    }

    /// Adds the specified child elements to this element.
    fn children(mut self, children: impl IntoIterator<Item = HtmlElement>) -> Self
    where
        Self: Sized,
    {
        self.children_mut().extend(children);
        self
    }
}

impl WithChildren for HtmlElement {
    #[inline(always)]
    fn children_mut(&mut self) -> &mut Vec<HtmlElement> {
        &mut self.children
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
        charset, class, content, defer, href, id, lang, max, name, rel, role, src, start, style,
        tabindex, target, title, translate, value
    );

    /// Sets the `async` attribute to the provided value.
    pub fn async_<V>(self, value: impl Into<Option<V>>) -> Self
    where
        V: Into<String>,
    {
        self.attr("async", value)
    }

    /// Sets the `http-equiv` attribute to the provided value.
    pub fn http_equiv<V>(self, value: impl Into<Option<V>>) -> Self
    where
        V: Into<String>,
    {
        self.attr("http-equiv", value)
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
    source, span, strong, style, sub, sup, svg, table, td, textarea, tfoot, th, thead, time, title,
    tr, track, u, ul, var, video, wbr, details, dialog, summary, slot, template
);

#[cfg(test)]
mod tests {
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
                .child(h1().class("heading").text_content("Hello, world!")),
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
            &p().child(HtmlElement::unstable_raw_text("This is a "))
                .child(a().href("https://example.com").text_content("link"))
                .child(HtmlElement::unstable_raw_text(" that you should click on."))
        ))
    }
}
