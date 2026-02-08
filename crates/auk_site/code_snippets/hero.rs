use auk::renderer::HtmlElementRenderer;
use auk::*;

fn main() {
    let content = html()
        .child(head().child(title().child("Auk")))
        .child(body().child(h1().child("Hello from Auk!")));

    let rendered_html = HtmlElementRenderer::new()
        .render_to_string(&content)
        .unwrap();
}
