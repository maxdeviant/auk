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

    auk_plumage::style_methods! {
        text_primary: "text-primary",
        text_primary_light: "text-primary-light",
        text_muted: "text-muted",
        text_dark: "text-dark",
        text_white: "text-white",
        text_light: "text-light"
    }

    auk_plumage::style_methods! {
        bg_primary: "bg-primary",
        bg_dark: "bg-dark",
        bg_alt: "bg-alt",
        bg_white: "bg-white"
    }

    auk_plumage::style_methods! {
        border_primary: "border-primary",
        border_default: "border-default"
    }

    auk_plumage::style_methods! {
        rounded_sm: "rounded-sm",
        rounded_md: "rounded-md",
        rounded_lg: "rounded-lg",
        rounded_xl: "rounded-xl"
    }

    auk_plumage::style_methods! {
        font_medium: "font-medium",
        font_semibold: "font-semibold",
        font_bold: "font-bold",
        font_extrabold: "font-extrabold"
    }

    auk_plumage::style_methods! {
        font_mono: "font-mono"
    }

    auk_plumage::style_methods! {
        text_xs: "text-xs",
        text_sm: "text-sm",
        text_base: "text-base",
        text_lg: "text-lg",
        text_xl: "text-xl",
        text_2xl: "text-2xl",
        text_3xl: "text-3xl",
        text_4xl: "text-4xl"
    }

    auk_plumage::style_methods! {
        transition: "transition",
        transition_colors: "transition-colors",
        transition_transform: "transition-transform",
        transition_all: "transition-all"
    }

    auk_plumage::style_methods! {
        shadow_sm: "shadow-sm",
        shadow_md: "shadow-md",
        shadow_lg: "shadow-lg"
    }

    auk_plumage::style_methods! {
        hover_lift: "hover-lift",
        hover_text_primary: "hover-text-primary",
        hover_border_primary: "hover-border-primary",
        hover_bg_primary_dark: "hover-bg-primary-dark"
    }

    auk_plumage::style_methods! {
        min_h_screen: "min-h-screen",
        overflow_x_auto: "overflow-x-auto",
        overflow_hidden: "overflow-hidden",
        whitespace_pre: "whitespace-pre"
    }
}
