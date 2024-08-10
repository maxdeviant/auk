# auk

[![crates.io](https://img.shields.io/crates/v/auk.svg)](https://crates.io/crates/auk)
[![docs.rs](https://docs.rs/auk/badge.svg)](https://docs.rs/auk/)
[![crates.io](https://img.shields.io/crates/l/auk.svg)](https://github.com/maxdeviant/auk/blob/main/LICENSE)

Auk is an <abbr title="embedded domain-specific language">eDSL</abbr> for writing HTML using standard Rust syntax.

## Usage

```rust
use auk::*;
use auk::renderer::HtmlElementRenderer;

let content = html()
    .child(head().child(title().child("Auk")))
    .child(body().child(h1().child("Hello from Auk!")));

let rendered_html = HtmlElementRenderer::new()
    .render_to_string(&content)
    .unwrap();
```
