use auk::*;

struct Greeting {
    name: String,
}

impl Greeting {
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl From<Greeting> for HtmlElement {
    fn from(this: Greeting) -> Self {
        div().child(h1().child(format!("Hello {}", this.name)))
    }
}

#[test]
fn test_component_to_element() {
    let _ = div().child(Greeting::new("world"));
}
