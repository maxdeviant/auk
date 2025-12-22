use auk::*;
use auk_ui::{Button, Stack};
use razorbill::render::RenderPageContext;

use crate::components::Skeleton;

pub fn stack(_ctx: &RenderPageContext) -> HtmlElement {
    Skeleton::new()
        .child(
            body()
                .child(h1().child("Stack"))
                .child(
                    Stack::vertical()
                        .child(div().child("A"))
                        .child(div().child("B"))
                        .child(div().child("C")),
                )
                .child(
                    Stack::horizontal()
                        .child(div().child("A"))
                        .child(div().child("B"))
                        .child(div().child("C")),
                ),
        )
        .into()
}

pub fn button(_ctx: &RenderPageContext) -> HtmlElement {
    Skeleton::new()
        .child(
            body()
                .child(h1().child("Button"))
                .child(Button::new().child("Click Me")),
        )
        .into()
}
