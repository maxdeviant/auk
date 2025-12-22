use auk::*;

pub struct Button {
    children: Vec<Element>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl With for Button {}

impl WithChildren for Button {
    fn extend(&mut self, children: impl IntoIterator<Item = Element>) {
        self.children.extend(children)
    }
}

impl Render for Button {
    fn render(self) -> impl Into<HtmlElement> {
        button().children(self.children)
    }
}
