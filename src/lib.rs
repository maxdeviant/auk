#![doc = include_str!("../README.md")]

use std::fmt::Write;

use indexmap::IndexMap;

#[derive(Debug)]
pub struct HtmlElement {
    pub tag_name: String,
    pub content: Option<String>,
    pub children: Vec<HtmlElement>,
    pub attrs: IndexMap<String, String>,
}

impl HtmlElement {
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag_name: tag.into(),
            content: None,
            children: Vec::new(),
            attrs: IndexMap::new(),
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

    pub fn text_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn child(mut self, child: HtmlElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = HtmlElement>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn render_to_string(&self) -> Result<String, std::fmt::Error> {
        let mut html = String::new();

        if self.tag_name == "html" {
            write!(&mut html, "<!DOCTYPE html>")?;
        }

        write!(&mut html, "<{}", self.tag_name)?;

        for (name, value) in &self.attrs {
            write!(&mut html, " ")?;
            write!(&mut html, r#"{name}="{value}""#)?;
        }

        write!(&mut html, ">")?;

        if self.is_void() {
            return Ok(html);
        }

        if let Some(content) = &self.content {
            write!(&mut html, "{}", content)?;
        }

        for child in &self.children {
            write!(&mut html, "{}", child.render_to_string()?)?;
        }

        write!(&mut html, "</{}>", self.tag_name)?;

        Ok(html)
    }
}

macro_rules! create_attribute_methods {
    ($($name:ident),*) => {
        $(
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
        charset, class, content, defer, href, id, lang, name, rel, role, src, start, style,
        tabindex, title, translate
    );

    pub fn async_<V>(self, value: impl Into<Option<V>>) -> Self
    where
        V: Into<String>,
    {
        self.attr("async", value)
    }

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

    #[test]
    fn it_works() {
        let element = div()
            .class("container")
            .child(h1().class("heading").text_content("Hello, world!"));

        dbg!(element);
    }

    #[test]
    fn test_render() {
        let element = div().class("outer").child(
            div()
                .class("inner")
                .child(h1().class("heading").text_content("Hello, world!")),
        );

        dbg!(element.render_to_string().unwrap());
    }
}
