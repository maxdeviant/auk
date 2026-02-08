use auk::With;
use auk_plumage::style_methods;

pub fn class() -> StyleBuilder {
    StyleBuilder {
        classes: Vec::new(),
    }
}

pub struct StyleBuilder {
    classes: Vec<String>,
}

impl With for StyleBuilder {}

impl From<StyleBuilder> for String {
    fn from(value: StyleBuilder) -> Self {
        value.classes.join(" ")
    }
}

impl StyleBuilder {
    #[inline(always)]
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    pub fn extend(mut self, classes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.classes.extend(classes.into_iter().map(Into::into));
        self
    }

    auk_plumage::all!();

    style_methods! {
        font_serif : "font-serif",
        font_monospace : "font-monospace"
    }

    style_methods! {
        white : "white",
        primary : "primary",
        bg_white : "bg-white",
        bg_dark : "bg-dark"
    }
}
