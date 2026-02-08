use arborium::Highlighter;
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
                .mx_auto()
                .px_3()
                .py_6(),
        )
        .child(
            div()
                .class(class().text_center())
                .child(
                    h1().class(class().m_0().mb_5().font_serif().font_size_8().primary())
                        .child("auk"),
                )
                .child(
                    h2().class(class().m_0().font_size_6().font_weight_6().white())
                        .child("Write HTML in ")
                        .child(span().class(class().primary()).child("Rust")),
                )
                .child(
                    p().child("Auk is an ")
                        .child(
                            abbr()
                                .title("embedded domain-specific language")
                                .child("eDSL"),
                        )
                        .child(" for writing HTML using standard Rust syntax."),
                ),
        )
        .child(hero_code_example())
}

fn hero_code_example() -> HtmlElement {
    let highlighter_config = arborium::Config {
        html_format: arborium::HtmlFormat::CustomElements,
        ..Default::default()
    };
    let mut highlighter = Highlighter::with_config(highlighter_config);

    let input = highlighter
        .highlight("rust", include_str!("../../code_snippets/hero.rs"))
        .unwrap();
    let output = highlighter
        .highlight("html", include_str!("../../code_snippets/hero.html"))
        .unwrap();

    section()
        .class(
            class()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap_4(),
        )
        .child(code_block(input))
        .child(down_arrow_icon())
        .child(code_block(output))
}

fn down_arrow_icon() -> HtmlElement {
    HtmlElement::new("svg")
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr("width", "32")
        .attr("height", "32")
        .attr("viewBox", "0 0 24 24")
        .attr("fill", "none")
        .attr("stroke", "currentColor")
        .attr("stroke-width", "2")
        .attr("stroke-linecap", "round")
        .attr("stroke-linejoin", "round")
        .child(HtmlElement::new("path").attr("d", "M12 5v14"))
        .child(HtmlElement::new("path").attr("d", "m19 12-7 7-7-7"))
}

fn code_block(source_code: String) -> HtmlElement {
    div()
        .class(
            class()
                .m_auto()
                .px_5()
                .py_3()
                .border_solid()
                .border_px()
                .rounded_2(),
        )
        .style("border-color: var(--color-white); width: 48rem;")
        .child(pre().child(code().child(TextElement {
            text: source_code,
            safe: true,
        })))
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
