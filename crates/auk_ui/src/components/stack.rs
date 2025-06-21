use auk::*;

use crate::class;

enum Direction {
    Vertical,
    Horizontal,
}

pub struct Stack {
    direction: Direction,
    classes: Vec<String>,
    children: Vec<Element>,
}

impl Stack {
    pub fn vertical() -> Self {
        Self {
            direction: Direction::Vertical,
            classes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn horizontal() -> Self {
        Self {
            direction: Direction::Horizontal,
            classes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    auk_plumage::align_items!();
    auk_plumage::justify_content!();
    auk_plumage::gap!();
}

impl With for Stack {}

impl WithChildren for Stack {
    fn extend(&mut self, children: impl IntoIterator<Item = Element>) {
        self.children.extend(children)
    }
}

impl Into<HtmlElement> for Stack {
    fn into(self) -> HtmlElement {
        div()
            .class(
                class()
                    .flex()
                    .with(|class| match self.direction {
                        Direction::Vertical => class.flex_col(),
                        Direction::Horizontal => class.flex_row(),
                    })
                    .extend(self.classes),
            )
            .children(self.children)
    }
}
