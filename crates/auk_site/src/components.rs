pub use auk::*;

pub struct Skeleton {
    children: Vec<Element>,
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl With for Skeleton {}

impl WithChildren for Skeleton {
    fn extend(&mut self, children: impl IntoIterator<Item = Element>) {
        self.children.extend(children)
    }
}

impl Render for Skeleton {
    fn render(self) -> impl Into<HtmlElement> {
        html()
            .lang("en")
            .child(
                head()
                    .child(meta().charset("utf-8"))
                    .child(meta().http_equiv("x-ua-compatible").content("ie=edge"))
                    .child(
                        meta()
                            .name("viewport")
                            .content("width=device-width, initial-scale=1.0, maximum-scale=1"),
                    )
                    .child(link().rel("stylesheet").href("/style.css")),
            )
            .children(self.children)
    }
}
