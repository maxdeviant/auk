use auk::*;

struct Greeting {
    name: String,
}

impl Greeting {
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Render for Greeting {
    fn render(self) -> impl Into<Element> {
        div().child(h1().child(format!("Hello {}", self.name)))
    }
}

#[test]
fn test_component_to_element() {
    let _ = div().child(Greeting::new("world"));
}
