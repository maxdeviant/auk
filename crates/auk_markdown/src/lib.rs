#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod markdown_components;
mod table_of_contents;

use std::collections::{HashMap, VecDeque};

use auk::{Element, HtmlElement, WithChildren};
use pulldown_cmark::{
    self as md, Alignment, CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, Tag,
};

pub use crate::markdown_components::*;
pub use crate::table_of_contents::*;

/// Renders the provided Markdown text into [`Element`]s and its [`TableOfContents`].
///
/// Uses the provided [`MarkdownComponents`] to render specific Markdown elements.
pub fn render_markdown(
    text: &str,
    components: &Box<dyn MarkdownComponents>,
) -> (Vec<Element>, TableOfContents) {
    let mut options = md::Options::empty();
    options.insert(md::Options::ENABLE_TABLES);
    options.insert(md::Options::ENABLE_FOOTNOTES);
    options.insert(md::Options::ENABLE_STRIKETHROUGH);
    options.insert(md::Options::ENABLE_TASKLISTS);
    options.insert(md::Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = md::Parser::new_ext(text, options);

    let mut elements = HtmlElementWriter::new(parser, components).run();
    let table_of_contents = TableOfContents::from_markdown(&mut elements);

    (elements, table_of_contents)
}

enum TableState {
    Head,
    Body,
}

struct HtmlElementWriter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    input: I,
    components: &'a Box<dyn MarkdownComponents>,
    elements: Vec<Element>,
    current_element_stack: VecDeque<HtmlElement>,
    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
    footnotes: HashMap<CowStr<'a>, usize>,
}

impl<'a, I> HtmlElementWriter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub fn new(input: I, components: &'a Box<dyn MarkdownComponents>) -> Self {
        Self {
            input,
            components,
            elements: Vec::new(),
            current_element_stack: VecDeque::new(),
            table_state: TableState::Head,
            table_alignments: Vec::new(),
            table_cell_index: 0,
            footnotes: HashMap::new(),
        }
    }

    fn run(mut self) -> Vec<Element> {
        while let Some(event) = self.input.next() {
            match event {
                Event::Start(tag) => {
                    self.start_tag(tag);
                }
                Event::End(tag) => {
                    self.end_tag(tag);
                }
                Event::Text(text) => {
                    if let Some(element) = self.current_element_stack.iter_mut().last() {
                        element.extend([text.to_string().into()]);
                    }
                }
                Event::Code(text) => {
                    self.write(
                        self.components
                            .code(CodeProps { language: None })
                            .child(text.to_string()),
                    );
                }
                Event::Html(html) => self.write_raw_html(&html),
                Event::SoftBreak => {
                    self.write_raw_html("\n");
                }
                Event::HardBreak => self.write(self.components.br()),
                Event::Rule => self.write(self.components.hr()),
                Event::FootnoteReference(name) => {
                    let next_footnote_number = self.footnotes.len() + 1;
                    let number = *self
                        .footnotes
                        .entry(name.clone())
                        .or_insert(next_footnote_number);

                    self.write(
                        self.components.sup().class("footnote-reference").child(
                            self.components
                                .a(AProps {
                                    href: format!("#{name}"),
                                    title: None,
                                })
                                .child(number.to_string()),
                        ),
                    );
                }
                Event::TaskListMarker(_checked) => todo!(),
            }
        }

        self.elements
    }

    fn run_raw_text(&mut self) -> String {
        let mut nest = 0;
        let mut raw_text = String::new();

        while let Some(event) = self.input.next() {
            match event {
                Event::Start(_) => {
                    nest += 1;
                }
                Event::End(_) => {
                    if nest == 0 {
                        break;
                    }

                    nest -= 1;
                }
                Event::Html(_) => {}
                Event::Text(text) | Event::Code(text) => {
                    raw_text.push_str(&text);
                }
                Event::SoftBreak | Event::HardBreak | Event::Rule => {
                    raw_text.push(' ');
                }
                Event::FootnoteReference(_name) => {}
                Event::TaskListMarker(_checked) => {}
            }
        }

        raw_text
    }

    fn write(&mut self, element: HtmlElement) {
        if let Some(parent) = self.current_element_stack.back_mut() {
            parent.extend([element.into()]);
        } else {
            self.elements.push(element.into());
        }
    }

    fn write_raw_html(&mut self, html: &str) {
        if let Some(parent) = self.current_element_stack.back_mut() {
            parent.extend([html.into()]);
        } else {
            self.elements.push(html.into());
        }
    }

    fn push(&mut self, element: HtmlElement) {
        self.current_element_stack.push_back(element);
    }

    fn pop(&mut self) {
        if let Some(element) = self.current_element_stack.pop_back() {
            self.write(element);
        }
    }

    fn start_tag(&mut self, tag: Tag<'a>) {
        match tag {
            Tag::Paragraph => self.push(self.components.p()),
            Tag::Heading(level, id, classes) => {
                let heading = match level {
                    HeadingLevel::H1 => self.components.h1(),
                    HeadingLevel::H2 => self.components.h2(),
                    HeadingLevel::H3 => self.components.h3(),
                    HeadingLevel::H4 => self.components.h4(),
                    HeadingLevel::H5 => self.components.h5(),
                    HeadingLevel::H6 => self.components.h6(),
                };

                self.push(
                    heading.id::<String>(id.map(Into::into)).class::<String>(
                        Some(classes)
                            .filter(|classes| !classes.is_empty())
                            .map(|classes| classes.into_iter().collect::<Vec<_>>().join(" ")),
                    ),
                )
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments;

                self.push(self.components.table())
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.table_cell_index = 0;

                self.push(self.components.thead());
                self.push(self.components.tr());
            }
            Tag::TableRow => {
                self.table_cell_index = 0;

                self.push(self.components.tr());
            }
            Tag::TableCell => match self.table_state {
                TableState::Head => self.push(self.components.th()),
                TableState::Body => self.push(self.components.td()),
            },
            Tag::BlockQuote => self.push(self.components.blockquote()),
            Tag::CodeBlock(kind) => {
                let language = match kind {
                    CodeBlockKind::Fenced(info) => Some(info.split(' ').next().unwrap())
                        .filter(|language| !language.trim().is_empty())
                        .map(Into::into),
                    CodeBlockKind::Indented => None,
                };

                self.push(self.components.pre(PreProps {
                    language: language.clone(),
                }));
                self.push(self.components.code(CodeProps { language }));
            }
            Tag::List(Some(1)) => self.push(self.components.ol()),
            Tag::List(Some(start)) => self.push(self.components.ol().start(start.to_string())),
            Tag::List(None) => self.push(self.components.ul()),
            Tag::Item => self.push(self.components.li()),
            Tag::Emphasis => self.push(self.components.em()),
            Tag::Strong => self.push(self.components.strong()),
            Tag::Strikethrough => self.push(self.components.del()),
            Tag::Link(LinkType::Email, dest, title) => self.push(
                self.components.a(AProps {
                    href: format!("mailto:{dest}"),
                    title: Some(title)
                        .filter(|title| !title.is_empty())
                        .map(|title| title.to_string()),
                }),
            ),
            Tag::Link(_link_type, dest, title) => self.push(
                self.components.a(AProps {
                    href: dest.to_string(),
                    title: Some(title)
                        .filter(|title| !title.is_empty())
                        .map(|title| title.to_string()),
                }),
            ),
            Tag::Image(_link_type, dest, title) => {
                let alt = Some(self.run_raw_text()).filter(|alt| !alt.trim().is_empty());
                let title = Some(title)
                    .filter(|title| !title.is_empty())
                    .map(|title| title.to_string());

                self.write(self.components.img(ImgProps {
                    src: dest.to_string(),
                    alt,
                    title,
                }));
            }
            Tag::FootnoteDefinition(name) => {
                let next_footnote_number = self.footnotes.len() + 1;
                let number = *self
                    .footnotes
                    .entry(name.clone())
                    .or_insert(next_footnote_number);

                self.push(
                    self.components
                        .div()
                        .class("footnote-definition")
                        .id(name.to_string()),
                );
                self.write(
                    self.components
                        .sup()
                        .class("footnote-definition-label")
                        .child(number.to_string()),
                );
            }
        }
    }

    fn end_tag(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => self.pop(),
            Tag::Heading(_, _, _) => self.pop(),
            Tag::Table(_) => {
                self.pop();
                self.pop();
            }
            Tag::TableHead => {
                self.pop();
                self.pop();
                self.push(self.components.tbody());

                self.table_state = TableState::Body;
            }
            Tag::TableRow => self.pop(),
            Tag::TableCell => {
                self.pop();

                self.table_cell_index += 1;
            }
            Tag::BlockQuote => self.pop(),
            Tag::CodeBlock(_) => {
                let code = self.current_element_stack.pop_back();
                let pre = self.current_element_stack.pop_back();

                if let Some((pre, code)) = pre.zip(code) {
                    self.write(self.components.on_code_block_end(pre, code));
                }
            }
            Tag::List(_) => self.pop(),
            Tag::Item => self.pop(),
            Tag::Emphasis => self.pop(),
            Tag::Strong => self.pop(),
            Tag::Strikethrough => self.pop(),
            Tag::Link(_, _, _) => self.pop(),
            Tag::Image(_, _, _) => self.pop(),
            Tag::FootnoteDefinition(_) => {
                self.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use auk::renderer::HtmlElementRenderer;
    use indoc::indoc;

    use super::*;

    fn parse_and_render_markdown(text: &str) -> String {
        let (elements, _table_of_contents) =
            render_markdown(text, &DefaultMarkdownComponents.boxed());

        elements
            .into_iter()
            .map(|element| match element {
                Element::Text(element) => element.text,
                Element::Html(element) => HtmlElementRenderer::new()
                    .render_to_string(&element)
                    .unwrap(),
            })
            .collect::<Vec<_>>()
            .join("")
    }

    #[test]
    fn test_markdown() {
        let text = indoc! {"
            # Hello, world!

            ## This is

            ### A markdown document

            Here are some items:
            - Apple
            - Banana
            - Fruit

            And some ordered items:
            1. One
            1. Two
            1. Three
        "};

        insta::assert_yaml_snapshot!(parse_and_render_markdown(text));
    }

    #[test]
    fn test_markdown_link() {
        let text = indoc! {"
            Here is a [link](https://example.com) that you should click!
        "};

        insta::assert_yaml_snapshot!(parse_and_render_markdown(text));
    }

    #[test]
    fn test_markdown_list_with_inline_code() {
        let text = indoc! {"
            - `One`
            - `Two`
            - `Three`
        "};

        insta::assert_yaml_snapshot!(parse_and_render_markdown(text));
    }

    #[test]
    fn test_markdown_image() {
        let text = indoc! {"
            Check out this cool image:

            ![very cool image](https://example.com/cool-image.png)

            This painting is beautiful:

            ![A photo of _Sunflowers_ by Van Gogh](https://example.com/sunflowers.png)

            Here's a picture of a less-than sign:

            ![A picture of a < sign](https://example.com/less-than.png)

            ![A screenshot of `ls` output](https://example.com/ls-output.png)
        "};

        insta::assert_yaml_snapshot!(parse_and_render_markdown(text));
    }

    #[test]
    fn test_markdown_table() {
        let text = indoc! {"
            # Table

            | Name | Value | Value 2 |
            | ---- | ----- | ------- |
            | A    | 1     | 17      |
            | B    | 2     | 25      |
            | C    | 3     | 32      |
        "};

        insta::assert_yaml_snapshot!(parse_and_render_markdown(text));
    }

    #[test]
    fn test_markdown_footnotes() {
        let text = indoc! {"
            The quick[^1] brown fox jumped over the lazy[^2] dog.

            ---

            [^1]: The fox wasn't all that quick.

            [^2]: The dog wasn't all that lazy.
        "};

        insta::assert_yaml_snapshot!(parse_and_render_markdown(text));
    }

    #[test]
    fn test_markdown_code_block() {
        let text = indoc! {r#"
            ```html
            <div>
              <h1 ng-if="isLoggedIn">Welcome back!</h1>
              <h1 ng-if="!isLoggedIn">Please sign in.</h1>
            </div>
            ```
        "#};

        insta::assert_yaml_snapshot!(parse_and_render_markdown(text));
    }
}
