use auk::*;
use razorbill::render::RenderSectionContext;

use crate::components::Skeleton;
use crate::style::class;

pub fn home(_ctx: &RenderSectionContext) -> HtmlElement {
    Skeleton::new()
        .child(body().class(class().min_h_screen()))
        .child(hero())
        .child(
            div()
                .class(class().m_auto().max_w_7().bg_white())
                .style("margin-top: -16rem;")
                .child(site_header())
                .child(p().child("Lorem ipsum"))
                .child(p().child("Lorem ipsum"))
                .child(p().child("Lorem ipsum"))
                .child(p().child("Lorem ipsum"))
                .child(p().child("Lorem ipsum"))
                .child(p().child("Lorem ipsum"))
                .child(p().child("Lorem ipsum")),
        )
        .into()
}

fn hero() -> HtmlElement {
    section()
        .class(
            class()
                .white()
                .bg_dark()
                .min_h_screen()
                .text_center()
                .mx_auto(),
        )
        .child(
            h2().class(class().m_0().font_size_6().font_weight_6().white())
                .child("Write HTML in ")
                .child(span().class(class().primary()).child("Rust")),
        )
}

fn site_header() -> HtmlElement {
    header().child(
        nav()
            .class(
                class()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_2()
                    .py_3(),
            )
            .child(
                a().class(class().font_serif().font_size_4().no_underline())
                    .href("/")
                    .child("auk"),
            )
            .child(
                div()
                    .class(class().flex().gap_5())
                    .child(nav_link("https://docs.rs/auk", "Docs"))
                    .child(nav_link("https://crates.io/crates/auk", "crates.io"))
                    .child(nav_link("https://github.com/maxdeviant/auk", "GitHub")),
            ),
    )
}

fn nav_link(href: &str, text: &str) -> HtmlElement {
    a().class(class().no_underline()).href(href).child(text)
}
