use auk::*;
use razorbill::render::RenderSectionContext;

use crate::components::Skeleton;

pub fn home(_ctx: &RenderSectionContext) -> HtmlElement {
    Skeleton::new()
        .child(
            body()
                .class("home")
                .child(div().class("home-container").child(header()))
                .child(hero())
                .child(
                    div()
                        .class("home-container")
                        .child(features())
                        .child(code_example())
                        .child(ecosystem())
                        .child(footer()),
                ),
        )
        .into()
}

fn header() -> HtmlElement {
    nav().class("home-nav").child(
        div()
            .class("home-nav-inner")
            .child(
                a().href("/")
                    .class("home-logo")
                    .child(span().class("home-logo-icon").child("🐧"))
                    .child(span().child("Auk")),
            )
            .child(
                div()
                    .class("home-nav-links")
                    .child(a().href("https://docs.rs/auk").child("Docs"))
                    .child(a().href("https://crates.io/crates/auk").child("Crates.io"))
                    .child(
                        a().href("https://github.com/maxdeviant/auk")
                            .child("GitHub"),
                    ),
            ),
    )
}

fn hero() -> HtmlElement {
    section().class("home-hero").child(
        div()
            .class("home-hero-content")
            .child(
                h1().class("home-hero-title")
                    .child("Write HTML with ")
                    .child(span().class("home-hero-highlight").child("Rust")),
            )
            .child(
                p().class("home-hero-subtitle")
                    .child("Auk is an eDSL for writing HTML using standard Rust syntax. Type-safe, composable, and elegant."),
            )
            .child(
                div()
                    .class("home-hero-actions")
                    .child(
                        a().href("https://docs.rs/auk")
                            .class("home-btn home-btn-primary")
                            .child("Get Started"),
                    )
                    .child(
                        a().href("https://github.com/maxdeviant/auk")
                            .class("home-btn home-btn-secondary")
                            .child("View on GitHub"),
                    ),
            ),
    )
}

fn features() -> HtmlElement {
    section().class("home-features").child(
        div()
            .class("home-features-grid")
            .child(feature_card(
                "🦀",
                "Pure Rust",
                "No templates, no macros magic. Just standard Rust code that's easy to read and write.",
            ))
            .child(feature_card(
                "🔒",
                "Type-Safe",
                "Catch HTML errors at compile time. Your IDE provides full autocomplete and documentation.",
            ))
            .child(feature_card(
                "🧩",
                "Composable",
                "Build reusable components using standard Rust patterns. Compose them freely.",
            ))
            .child(feature_card(
                "⚡",
                "Fast",
                "Zero runtime overhead. Renders directly to strings with no intermediate representation.",
            ))
            .child(feature_card(
                "🛡️",
                "XSS Protection",
                "HTML escaping by default keeps your application safe from cross-site scripting attacks.",
            ))
            .child(feature_card(
                "📦",
                "Ecosystem",
                "Includes Markdown support, utility CSS, and UI components out of the box.",
            )),
    )
}

fn feature_card(icon: &'static str, title: &'static str, description: &'static str) -> HtmlElement {
    div()
        .class("home-feature-card")
        .child(div().class("home-feature-icon").child(icon))
        .child(h3().class("home-feature-title").child(title))
        .child(p().class("home-feature-description").child(description))
}

fn code_example() -> HtmlElement {
    section().class("home-code-example").child(
        div()
            .class("home-code-container")
            .child(h2().class("home-section-title").child("Elegant by Design"))
            .child(
                div()
                    .class("home-code-grid")
                    .child(
                        div()
                            .class("home-code-block")
                            .child(div().class("home-code-header").child("Rust"))
                            .child(pre().class("home-code").child(code().child(
                                r#"use auk::*;

let page = html()
    .child(head()
        .child(title().child("Hello")))
    .child(body()
        .child(h1().child("Hello, Auk!"))
        .child(p()
            .class("greeting")
            .child("Welcome to type-safe HTML.")));"#,
                            ))),
                    )
                    .child(
                        div()
                            .class("home-code-block")
                            .child(div().class("home-code-header").child("Output"))
                            .child(pre().class("home-code").child(code().child(
                                r#"<!DOCTYPE html>
<html>
  <head>
    <title>Hello</title>
  </head>
  <body>
    <h1>Hello, Auk!</h1>
    <p class="greeting">
      Welcome to type-safe HTML.
    </p>
  </body>
</html>"#,
                            ))),
                    ),
            ),
    )
}

fn ecosystem() -> HtmlElement {
    section().class("home-ecosystem").child(
        div()
            .class("home-ecosystem-content")
            .child(h2().class("home-section-title").child("The Auk Ecosystem"))
            .child(
                div()
                    .class("home-ecosystem-grid")
                    .child(ecosystem_card(
                        "auk",
                        "The core library for building HTML with Rust.",
                    ))
                    .child(ecosystem_card(
                        "auk_markdown",
                        "Markdown parsing and rendering integration.",
                    ))
                    .child(ecosystem_card(
                        "auk_plumage",
                        "Utility-first CSS styling for Auk projects.",
                    )),
            ),
    )
}

fn ecosystem_card(name: &'static str, description: &'static str) -> HtmlElement {
    a().href(format!("https://crates.io/crates/{name}"))
        .class("home-ecosystem-card")
        .child(h3().class("home-ecosystem-name").child(name))
        .child(p().class("home-ecosystem-description").child(description))
}

fn footer() -> HtmlElement {
    auk::footer().class("home-footer").child(
        div()
            .class("home-footer-content")
            .child(p().child("Made with 🧡 for the Rust community"))
            .child(
                p().class("home-footer-links")
                    .child(
                        a().href("https://github.com/maxdeviant/auk")
                            .child("GitHub"),
                    )
                    .child(span().class("home-footer-separator").child("·"))
                    .child(a().href("https://crates.io/crates/auk").child("Crates.io"))
                    .child(span().class("home-footer-separator").child("·"))
                    .child(a().href("https://docs.rs/auk").child("Docs")),
            ),
    )
}
