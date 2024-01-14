# auk

[![crates.io](https://img.shields.io/crates/v/auk.svg)](https://crates.io/crates/auk)
[![docs.rs](https://docs.rs/auk/badge.svg)](https://docs.rs/auk/)
[![crates.io](https://img.shields.io/crates/l/auk.svg)](https://github.com/maxdeviant/auk/blob/main/LICENSE)

Auk is a <abbr title="domain-specific language">DSL</abbr> for writing HTML using standard Rust syntax.

## Usage

```rust
use auk::*;
use auk::renderer::HtmlElementRenderer;

let content = html()
    .child(head().child(title().child(text("Auk"))))
    .child(body().child(h1().child(text("Hello from Auk!"))));

let rendered_html = HtmlElementRenderer::new()
    .render_to_string(&content)
    .unwrap();
```
