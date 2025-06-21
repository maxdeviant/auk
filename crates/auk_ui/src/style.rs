use auk::With;

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
}
