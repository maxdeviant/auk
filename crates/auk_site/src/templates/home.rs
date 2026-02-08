use auk::*;
use razorbill::render::RenderSectionContext;

use crate::components::Skeleton;
use crate::style::class;

pub fn home(_ctx: &RenderSectionContext) -> HtmlElement {
    Skeleton::new()
        .child(
            body()
                .class(class().min_h_screen())
                .class("home")
                .child(hero())
                .child(
                    div()
                        .class("home-main-container")
                        .child(header())
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
            .class(class().flex().justify_between().items_center())
            .child(
                a().href("/")
                    .class(
                        class()
                            .flex()
                            .items_center()
                            .gap_2()
                            .text_xl()
                            .font_bold()
                            .text_dark()
                            .hover_text_primary()
                            .no_underline(),
                    )
                    .child(span().class(class().text_2xl()).child("🐧"))
                    .child(span().child("Auk")),
            )
            .child(
                div()
                    .class(class().flex().gap_5())
                    .child(nav_link("https://docs.rs/auk", "Docs"))
                    .child(nav_link("https://crates.io/crates/auk", "Crates.io"))
                    .child(nav_link("https://github.com/maxdeviant/auk", "GitHub")),
            ),
    )
}

fn nav_link(href: &'static str, text: &'static str) -> HtmlElement {
    a().href(href)
        .class(
            class()
                .text_muted()
                .font_medium()
                .transition_colors()
                .hover_text_primary()
                .no_underline(),
        )
        .child(text)
}

fn hero() -> HtmlElement {
    section().class("home-hero").child(
        div()
            .class(class().text_center().mx_auto())
            .class("home-hero-content")
            .child(
                h1().class(class().text_4xl().font_extrabold().text_white().mb_4())
                    .class("home-hero-title")
                    .child("Write HTML with ")
                    .child(span().class(class().text_primary()).child("Rust")),
            )
            .child(
                p().class(class().text_xl().text_light().mb_6().mx_auto())
                    .class("home-hero-subtitle")
                    .child("Auk is an eDSL for writing HTML using standard Rust syntax. Type-safe, composable, and elegant."),
            )
            .child(
                div()
                    .class(class().flex().gap_3().justify_center().flex_wrap())
                    .child(
                        a().href("https://docs.rs/auk")
                            .class(
                                class()
                                    .inline_flex()
                                    .items_center()
                                    .p_3()
                                    .text_base()
                                    .font_semibold()
                                    .rounded_md()
                                    .bg_primary()
                                    .text_white()
                                    .hover_lift()
                                    .hover_bg_primary_dark()
                                    .no_underline(),
                            )
                            .child("Get Started"),
                    )
                    .child(
                        a().href("https://github.com/maxdeviant/auk")
                            .class(
                                class()
                                    .inline_flex()
                                    .items_center()
                                    .p_3()
                                    .text_base()
                                    .font_semibold()
                                    .rounded_md()
                                    .bg_dark()
                                    .text_white()
                                    .hover_lift()
                                    .no_underline(),
                            )
                            .child("View on GitHub"),
                    ),
            ),
    )
}

fn features() -> HtmlElement {
    section().class(class().p_6().bg_white()).child(
        div()
            .class(class().grid().gap_5().mx_auto())
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
        .class(
            class()
                .p_5()
                .bg_white()
                .rounded_lg()
                .transition()
                .hover_border_primary(),
        )
        .class("home-feature-card")
        .child(div().class(class().text_2xl().mb_3()).child(icon))
        .child(
            h3().class(class().text_xl().font_bold().text_dark().mb_2())
                .child(title),
        )
        .child(p().class(class().text_muted().m_0()).child(description))
}

fn code_example() -> HtmlElement {
    section().class(class().p_6().bg_alt()).child(
        div()
            .class(class().mx_auto())
            .class("home-code-container")
            .child(
                h2().class(
                    class()
                        .text_2xl()
                        .font_extrabold()
                        .text_center()
                        .text_dark()
                        .mb_6(),
                )
                .child("Elegant by Design"),
            )
            .child(
                div()
                    .class(class().grid().gap_4())
                    .class("home-code-grid")
                    .child(code_block("Rust", RUST_EXAMPLE))
                    .child(code_block("Output", HTML_OUTPUT)),
            ),
    )
}

const RUST_EXAMPLE: &str = r#"use auk::*;

let page = html()
    .child(head()
        .child(title().child("Hello")))
    .child(body()
        .child(h1().child("Hello, Auk!"))
        .child(p()
            .class("greeting")
            .child("Welcome to type-safe HTML.")));"#;

const HTML_OUTPUT: &str = r#"<!DOCTYPE html>
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
</html>"#;

fn code_block(header_text: &'static str, code_content: &'static str) -> HtmlElement {
    div()
        .class(class().bg_dark().rounded_lg().overflow_hidden())
        .class("home-code-block")
        .child(
            div()
                .class(class().p_3().text_primary_light().text_sm().font_semibold())
                .class("home-code-header")
                .child(header_text),
        )
        .child(
            pre().class(class().m_0().p_4().overflow_x_auto()).child(
                code()
                    .class(class().font_mono().text_sm().text_light().whitespace_pre())
                    .child(code_content),
            ),
        )
}

fn ecosystem() -> HtmlElement {
    section().class(class().p_6().bg_white()).child(
        div()
            .class(class().mx_auto())
            .class("home-ecosystem-content")
            .child(
                h2().class(
                    class()
                        .text_2xl()
                        .font_extrabold()
                        .text_center()
                        .text_dark()
                        .mb_6(),
                )
                .child("The Auk Ecosystem"),
            )
            .child(
                div()
                    .class(class().grid().gap_4())
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
        .class(
            class()
                .block()
                .p_4()
                .bg_white()
                .rounded_lg()
                .transition()
                .hover_lift()
                .hover_border_primary()
                .no_underline(),
        )
        .class("home-ecosystem-card")
        .child(
            h3().class(
                class()
                    .font_mono()
                    .text_lg()
                    .font_bold()
                    .text_primary()
                    .mb_2(),
            )
            .child(name),
        )
        .child(
            p().class(class().text_muted().text_sm().m_0())
                .child(description),
        )
}

fn footer() -> HtmlElement {
    auk::footer()
        .class(class().p_6().bg_alt().text_muted().text_center())
        .class("home-footer")
        .child(
            div()
                .class(class().mx_auto())
                .class("home-footer-content")
                .child(
                    p().class(class().m_0())
                        .child("Made with 🧡 for the Rust community"),
                )
                .child(
                    p().class(
                        class()
                            .flex()
                            .justify_center()
                            .items_center()
                            .gap_2()
                            .mt_3(),
                    )
                    .child(footer_link("https://github.com/maxdeviant/auk", "GitHub"))
                    .child(
                        span()
                            .class(class().text_muted())
                            .class("home-footer-separator")
                            .child("·"),
                    )
                    .child(footer_link("https://crates.io/crates/auk", "Crates.io"))
                    .child(
                        span()
                            .class(class().text_muted())
                            .class("home-footer-separator")
                            .child("·"),
                    )
                    .child(footer_link("https://docs.rs/auk", "Docs")),
                ),
        )
}

fn footer_link(href: &'static str, text: &'static str) -> HtmlElement {
    a().href(href)
        .class(
            class()
                .text_muted()
                .transition_colors()
                .hover_text_primary()
                .no_underline(),
        )
        .child(text)
}
