use std::collections::HashMap;

use auk::visitor::{noop_visit_element, MutVisitor};
use auk::{Element, HtmlElement};
use derive_more::{Deref, DerefMut};
use slug::slugify;

/// The table of contents for a Markdown document.
#[derive(Debug, Default, Deref, DerefMut)]
pub struct TableOfContents(Vec<Heading>);

impl TableOfContents {
    /// Returns a [`TableOfContents`] from the provided list of [`Element`]s.
    pub fn from_markdown(elements: &mut Vec<Element>) -> Self {
        let mut heading_identifier = HeadingIdentifier::new();
        heading_identifier.visit_children(elements).unwrap();

        Self::from_headings(heading_identifier.headings)
    }

    /// Returns a [`TableOfContents`] from the provided list of [`Heading`]s.
    pub fn from_headings(headings: Vec<Heading>) -> Self {
        let mut table_of_contents = vec![];
        for heading in headings {
            if table_of_contents.is_empty()
                || !Self::insert_into_parent(table_of_contents.iter_mut().last(), &heading)
            {
                table_of_contents.push(heading);
            }
        }

        Self(table_of_contents)
    }

    fn insert_into_parent(parent: Option<&mut Heading>, heading: &Heading) -> bool {
        let Some(parent) = parent else {
            return false;
        };

        if heading.level <= parent.level {
            return false;
        }

        if heading.level + 1 == parent.level {
            parent.children.push(heading.clone());
            return true;
        }

        if !Self::insert_into_parent(parent.children.iter_mut().last(), heading) {
            parent.children.push(heading.clone());
        }

        true
    }
}

/// A heading in a Markdown document.
#[derive(Debug, Clone)]
pub struct Heading {
    /// The level of this heading.
    pub level: u32,

    /// The ID of this heading.
    pub id: String,

    /// The title of this heading.
    pub title: String,

    /// The child headings nested under this heading.
    pub children: Vec<Heading>,
}

struct HeadingIdentifier {
    headings: Vec<Heading>,
    heading_id_counts: HashMap<String, usize>,
    inside_header: bool,
    title: Option<String>,
}

impl HeadingIdentifier {
    fn new() -> Self {
        Self {
            headings: Vec::new(),
            heading_id_counts: HashMap::new(),
            inside_header: false,
            title: None,
        }
    }
}

impl MutVisitor for HeadingIdentifier {
    type Error = ();

    fn visit(&mut self, element: &mut HtmlElement) -> Result<(), Self::Error> {
        match element.tag_name.as_str() {
            "h2" | "h3" | "h4" | "h5" | "h6" => {
                self.inside_header = true;

                noop_visit_element(self, element)?;

                if let Some(title) = self.title.take() {
                    let mut id = slugify(
                        title
                            // HACK: Remove undesired remnants from escaping.
                            // We should figure out how to avoid escaping in the first place.
                            .replace("&quot;", ""),
                    );

                    let id_count = self.heading_id_counts.entry(id.clone()).or_insert(0);
                    if *id_count > 0 {
                        id.push_str("-");
                        id.push_str(&id_count.to_string());
                    }

                    *id_count += 1;

                    if element.attrs.get("id").is_none() {
                        element.attrs.insert("id".to_string(), id.clone());
                    }

                    self.headings.push(Heading {
                        level: match element.tag_name.as_str() {
                            "h2" => 2,
                            "h3" => 3,
                            "h4" => 4,
                            "h5" => 5,
                            "h6" => 6,
                            _ => unreachable!(),
                        },
                        id,
                        title,
                        children: Vec::new(),
                    });
                }

                self.inside_header = false;
            }
            _ => {}
        }

        noop_visit_element(self, element)
    }

    fn visit_text(&mut self, text: &mut String, _safe: &mut bool) -> Result<(), Self::Error> {
        if self.inside_header {
            let mut title = self.title.take().unwrap_or_default();
            title.push_str(&text);
            self.title = Some(title);
        }

        Ok(())
    }
}
