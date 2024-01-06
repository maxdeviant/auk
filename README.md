# auk

[![crates.io](https://img.shields.io/crates/v/auk.svg)](https://crates.io/crates/auk)
[![docs.rs](https://docs.rs/auk/badge.svg)](https://docs.rs/auk/)
[![crates.io](https://img.shields.io/crates/l/auk.svg)](https://github.com/maxdeviant/auk/blob/main/LICENSE)

Auk is a <abbr title="domain-specific language">DSL</abbr> for writing HTML using standard Rust syntax.

## Usage

```rust
use auk::*;

let content = html()
    .child(head().child(title().text_content("Auk")))
    .child(body().child(h1().text_content("Hello from Auk!")));

let rendered_html = content.render_to_string().unwrap();
```
