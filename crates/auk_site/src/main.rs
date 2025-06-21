mod components;
mod templates;

use anyhow::Result;
use auk::*;
use clap::{Parser, Subcommand};
use razorbill::Site;
use razorbill::render::{RenderPageContext, RenderSectionContext};

use crate::components::Skeleton;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Serve,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let site = Site::builder()
        .root("crates/auk_site")
        .base_url("http://localhost:3000")
        .title("Auk")
        .templates(index, section, page)
        .add_page_template("auk_ui.stack", templates::auk_ui::stack)
        .with_sass("sass")
        .add_sass_load_path(auk_plumage::sass_load_path())
        .build();

    match cli.command {
        Command::Serve => {
            site.serve().await?;
        }
    }

    Ok(())
}

fn index(ctx: &RenderSectionContext) -> HtmlElement {
    Skeleton::new()
        .child(body().child(div().children(ctx.section.content.clone())))
        .into()
}

fn section(ctx: &RenderSectionContext) -> HtmlElement {
    Skeleton::new()
        .child(body().child(div().children(ctx.section.content.clone())))
        .into()
}

fn page(ctx: &RenderPageContext) -> HtmlElement {
    Skeleton::new()
        .child(body().child(div().children(ctx.page.content.clone())))
        .into()
}
