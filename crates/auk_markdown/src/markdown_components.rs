use auk::{HtmlElement, With, WithChildren};

/// The props for an `<a>` element.
#[derive(Debug)]
pub struct AProps {
    /// The URL that the hyperlink points to.
    pub href: String,

    /// The title of the hyperlink.
    pub title: Option<String>,
}

/// The props for a `<code>` element.
#[derive(Debug)]
pub struct CodeProps {
    /// The language of the code in this `<code>` element.
    pub language: Option<String>,
}

/// The props for an `<img>` element.
#[derive(Debug)]
pub struct ImgProps {
    /// The source URL of the image.
    pub src: String,

    /// The text that can replace the image.
    pub alt: Option<String>,

    /// The title of the image.
    pub title: Option<String>,
}

/// The props for a `<pre>` element.
#[derive(Debug)]
pub struct PreProps {
    /// The language of the code in this `<pre>` element.
    pub language: Option<String>,
}

/// A trait for customizing the rendering of Markdown elements.
pub trait MarkdownComponents: Send + Sync {
    /// Renders a `<div>`.
    fn div(&self) -> HtmlElement {
        auk::div()
    }

    /// Renders a `<p>`.
    fn p(&self) -> HtmlElement {
        auk::p()
    }

    /// Renders an `<h1>`.
    fn h1(&self) -> HtmlElement {
        auk::h1()
    }

    /// Renders an `<h2>`.
    fn h2(&self) -> HtmlElement {
        auk::h2()
    }

    /// Renders an `<h3>`.
    fn h3(&self) -> HtmlElement {
        auk::h3()
    }

    /// Renders an `<h4>`.
    fn h4(&self) -> HtmlElement {
        auk::h4()
    }

    /// Renders an `<h5>`.
    fn h5(&self) -> HtmlElement {
        auk::h5()
    }

    /// Renders an `<h6>`.
    fn h6(&self) -> HtmlElement {
        auk::h6()
    }

    /// Renders a `<table>`.
    fn table(&self) -> HtmlElement {
        auk::table()
    }

    /// Renders a `<thead>`.
    fn thead(&self) -> HtmlElement {
        auk::thead()
    }

    /// Renders a `<tbody>`.
    fn tbody(&self) -> HtmlElement {
        auk::tbody()
    }

    /// Renders a `<tr>`.
    fn tr(&self) -> HtmlElement {
        auk::tr()
    }

    /// Renders a `<th>`.
    fn th(&self) -> HtmlElement {
        auk::th()
    }

    /// Renders a `<td>`.
    fn td(&self) -> HtmlElement {
        auk::td()
    }

    /// Renders a `<blockquote>`.
    fn blockquote(&self) -> HtmlElement {
        auk::blockquote()
    }

    /// Renders a `<pre>`.
    fn pre(&self, props: PreProps) -> HtmlElement {
        auk::pre().with(|parent| {
            if let Some(language) = props.language {
                parent
                    .class(format!("language-{language}"))
                    .attr("data-lang", &language)
            } else {
                parent
            }
        })
    }

    /// Renders a `<code>`.
    fn code(&self, props: CodeProps) -> HtmlElement {
        auk::code().with(|parent| {
            if let Some(language) = props.language {
                parent
                    .class(format!("language-{language}"))
                    .attr("data-lang", &language)
            } else {
                parent
            }
        })
    }

    /// Handles the end of a code block.
    fn on_code_block_end(&self, pre: HtmlElement, code: HtmlElement) -> HtmlElement {
        pre.child(code)
    }

    /// Renders an `<ol>`.
    fn ol(&self) -> HtmlElement {
        auk::ol()
    }

    /// Renders a `<ul>`.
    fn ul(&self) -> HtmlElement {
        auk::ul()
    }

    /// Renders an `<li>`.
    fn li(&self) -> HtmlElement {
        auk::li()
    }

    /// Renders an `<em>`.
    fn em(&self) -> HtmlElement {
        auk::em()
    }

    /// Renders a `<strong>`.
    fn strong(&self) -> HtmlElement {
        auk::strong()
    }

    /// Renders a `<del>`.
    fn del(&self) -> HtmlElement {
        auk::del()
    }

    /// Renders an `<a>`.
    fn a(&self, props: AProps) -> HtmlElement {
        auk::a().href(props.href).title::<String>(props.title)
    }

    /// Renders an `<img>`.
    fn img(&self, props: ImgProps) -> HtmlElement {
        auk::img()
            .src(props.src)
            .alt::<String>(props.alt)
            .title::<String>(props.title)
    }

    /// Renders a `<br>`.
    fn br(&self) -> HtmlElement {
        auk::br()
    }

    /// Renders a `<hr>`.
    fn hr(&self) -> HtmlElement {
        auk::hr()
    }

    /// Renders a `<sup>`.
    fn sup(&self) -> HtmlElement {
        auk::sup()
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct DefaultMarkdownComponents;

impl DefaultMarkdownComponents {
    #[cfg(test)]
    pub fn boxed(self) -> Box<dyn MarkdownComponents> {
        Box::new(self)
    }
}

impl MarkdownComponents for DefaultMarkdownComponents {}
