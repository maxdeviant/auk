#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::path::PathBuf;

/// Private internals used by macros.
///
/// **Do not** depend on these directly, or your code is likely to break.
#[doc(hidden)]
pub mod __private {
    pub use paste::paste;
}

/// Returns the load path for the Sass files bundled with `auk_plumage`.
///
/// Include this path in the load paths in your Sass compiler.
pub fn sass_load_path() -> PathBuf {
    let out_dir = PathBuf::from(env!("OUT_DIR"));
    out_dir.join("sass")
}

/// Generates style methods that correspond to the specified CSS class names.
///
/// # Examples
///
/// ```
/// use auk_plumage::style_methods;
///
/// style_methods! {
///     primary: "primary",
///     secondary: "secondary",
///     success: "success",
///     danger: "danger",
///     warning: "warning",
///     info: "info",
///     light: "light",
///     dark: "dark",
/// }
/// ```
#[macro_export]
macro_rules! style_methods {
    ($($method_name:ident : $class_name:expr),*) => {
        $(
            pub fn $method_name(self) -> Self {
                self.class($class_name)
            }
        )*
    }
}

/// Generates responsive style methods that correspond to the specified CSS class names.
///
/// Each generated method will have a responsive variant for each responsive breakpoint.
#[macro_export]
macro_rules! responsive_style_methods {
    ($($method_name:ident : $class_name:expr),*) => {
        $(
            pub fn $method_name(self) -> Self {
                self.class($class_name)
            }

            $crate::__private::paste! {
                pub fn [<$method_name _ns>](self) -> Self {
                    self.class(format!("{}-ns", $class_name))
                }

                pub fn [<$method_name _m>](self) -> Self {
                    self.class(format!("{}-m", $class_name))
                }

                pub fn [<$method_name _l>](self) -> Self {
                    self.class(format!("{}-l", $class_name))
                }
            }
        )*
    }
}

/// Generates style methods for all of the styles provided by `auk_plumage`.
#[macro_export]
macro_rules! all {
    () => {
        $crate::appearance!();
        $crate::display!();
        $crate::flex_grid!();
        $crate::align_items!();
        $crate::justify_content!();
        $crate::gap!();
        $crate::grid_template_columns!();
        $crate::font_size!();
        $crate::font_weight!();
        $crate::leading!();
        $crate::position!();
        $crate::sizing!();
        $crate::max_width!();
        $crate::spacing!();
        $crate::text_align!();
        $crate::text_decoration!();
        $crate::text_transform!();
        $crate::tracking!();
        $crate::list_style_type!();
        $crate::border_width!();
        $crate::border_style!();
        $crate::border_radius!();
    };
}

/// Generates style methods for the spacing classes.
#[macro_export]
macro_rules! spacing {
    () => {
        $crate::responsive_style_methods! {
            m_0 : "m-0",
            m_1 : "m-1",
            m_2 : "m-2",
            m_3 : "m-3",
            m_4 : "m-4",
            m_5 : "m-5",
            m_6 : "m-6",
            m_7 : "m-7",
            m_8 : "m-8"
        }

        $crate::responsive_style_methods! {
            ml_0 : "ml-0",
            ml_1 : "ml-1",
            ml_2 : "ml-2",
            ml_3 : "ml-3",
            ml_4 : "ml-4",
            ml_5 : "ml-5",
            ml_6 : "ml-6",
            ml_7 : "ml-7",
            ml_8 : "ml-8"
        }

        $crate::responsive_style_methods! {
            mr_0 : "mr-0",
            mr_1 : "mr-1",
            mr_2 : "mr-2",
            mr_3 : "mr-3",
            mr_4 : "mr-4",
            mr_5 : "mr-5",
            mr_6 : "mr-6",
            mr_7 : "mr-7",
            mr_8 : "mr-8"
        }

        $crate::responsive_style_methods! {
            mt_0 : "mt-0",
            mt_1 : "mt-1",
            mt_2 : "mt-2",
            mt_3 : "mt-3",
            mt_4 : "mt-4",
            mt_5 : "mt-5",
            mt_6 : "mt-6",
            mt_7 : "mt-7",
            mt_8 : "mt-8"
        }

        $crate::responsive_style_methods! {
            mb_0 : "mb-0",
            mb_1 : "mb-1",
            mb_2 : "mb-2",
            mb_3 : "mb-3",
            mb_4 : "mb-4",
            mb_5 : "mb-5",
            mb_6 : "mb-6",
            mb_7 : "mb-7",
            mb_8 : "mb-8"
        }

        $crate::responsive_style_methods! {
            mx_0 : "mx-0",
            mx_1 : "mx-1",
            mx_2 : "mx-2",
            mx_3 : "mx-3",
            mx_4 : "mx-4",
            mx_5 : "mx-5",
            mx_6 : "mx-6",
            mx_7 : "mx-7",
            mx_8 : "mx-8"
        }

        $crate::responsive_style_methods! {
            my_0 : "my-0",
            my_1 : "my-1",
            my_2 : "my-2",
            my_3 : "my-3",
            my_4 : "my-4",
            my_5 : "my-5",
            my_6 : "my-6",
            my_7 : "my-7",
            my_8 : "my-8"
        }

        $crate::responsive_style_methods! {
            m_auto : "m-auto",
            mt_auto : "mt-auto",
            mr_auto : "mr-auto",
            mb_auto : "mb-auto",
            ml_auto : "ml-auto",
            mx_auto : "mx-auto",
            my_auto : "my-auto"
        }

        $crate::responsive_style_methods! {
            p_0 : "p-0",
            p_1 : "p-1",
            p_2 : "p-2",
            p_3 : "p-3",
            p_4 : "p-4",
            p_5 : "p-5",
            p_6 : "p-6",
            p_7 : "p-7",
            p_8 : "p-8"
        }

        $crate::responsive_style_methods! {
            pl_0 : "pl-0",
            pl_1 : "pl-1",
            pl_2 : "pl-2",
            pl_3 : "pl-3",
            pl_4 : "pl-4",
            pl_5 : "pl-5",
            pl_6 : "pl-6",
            pl_7 : "pl-7",
            pl_8 : "pl-8"
        }

        $crate::responsive_style_methods! {
            pr_0 : "pr-0",
            pr_1 : "pr-1",
            pr_2 : "pr-2",
            pr_3 : "pr-3",
            pr_4 : "pr-4",
            pr_5 : "pr-5",
            pr_6 : "pr-6",
            pr_7 : "pr-7",
            pr_8 : "pr-8"
        }

        $crate::responsive_style_methods! {
            pt_0 : "pt-0",
            pt_1 : "pt-1",
            pt_2 : "pt-2",
            pt_3 : "pt-3",
            pt_4 : "pt-4",
            pt_5 : "pt-5",
            pt_6 : "pt-6",
            pt_7 : "pt-7",
            pt_8 : "pt-8"
        }

        $crate::responsive_style_methods! {
            pb_0 : "pb-0",
            pb_1 : "pb-1",
            pb_2 : "pb-2",
            pb_3 : "pb-3",
            pb_4 : "pb-4",
            pb_5 : "pb-5",
            pb_6 : "pb-6",
            pb_7 : "pb-7",
            pb_8 : "pb-8"
        }

        $crate::responsive_style_methods! {
            px_0 : "px-0",
            px_1 : "px-1",
            px_2 : "px-2",
            px_3 : "px-3",
            px_4 : "px-4",
            px_5 : "px-5",
            px_6 : "px-6",
            px_7 : "px-7",
            px_8 : "px-8"
        }

        $crate::responsive_style_methods! {
            py_0 : "py-0",
            py_1 : "py-1",
            py_2 : "py-2",
            py_3 : "py-3",
            py_4 : "py-4",
            py_5 : "py-5",
            py_6 : "py-6",
            py_7 : "py-7",
            py_8 : "py-8"
        }
    };
}

/// Generates style methods for the sizing classes.
#[macro_export]
macro_rules! sizing {
    () => {
        $crate::responsive_style_methods! {
            w_0 : "w-0",
            w_1 : "w-1",
            w_2 : "w-2",
            w_3 : "w-3",
            w_4 : "w-4",
            w_5 : "w-5",
            w_6 : "w-6",
            w_7 : "w-7",
            w_8 : "w-8",
            w_20 : "w-20",
            w_25 : "w-25",
            w_50 : "w-50",
            w_75 : "w-75",
            w_80 : "w-80",
            w_one_third : "w-one-third",
            w_two_thirds : "w-two-thirds",
            w_full : "w-full",
            w_fit : "w-fit",
            w_auto : "w-auto"
        }

        $crate::responsive_style_methods! {
            h_0 : "h-0",
            h_1 : "h-1",
            h_2 : "h-2",
            h_3 : "h-3",
            h_4 : "h-4",
            h_5 : "h-5",
            h_6 : "h-6",
            h_7 : "h-7",
            h_8 : "h-8",
            h_full : "h-full",
            h_fit : "h-fit",
            h_auto : "h-auto"
        }

        $crate::responsive_style_methods! {
            size_0 : "size-0",
            size_1 : "size-1",
            size_2 : "size-2",
            size_3 : "size-3",
            size_4 : "size-4",
            size_5 : "size-5",
            size_6 : "size-6",
            size_7 : "size-7",
            size_8 : "size-8",
            size_full : "size-full",
            size_fit : "size-fit",
            size_auto : "size-auto"
        }
    };
}

/// Generates style methods for the `max-width` classes.
///
/// [MDN: `max-width`](https://developer.mozilla.org/en-US/docs/Web/CSS/max-width)
#[macro_export]
macro_rules! max_width {
    () => {
        $crate::responsive_style_methods! {
            max_w_0 : "max-w-0",
            max_w_1 : "max-w-1",
            max_w_2 : "max-w-2",
            max_w_3 : "max-w-3",
            max_w_4 : "max-w-4",
            max_w_5 : "max-w-5",
            max_w_6 : "max-w-6",
            max_w_7 : "max-w-7",
            max_w_8 : "max-w-8"
        }
    };
}

/// Generates style methods for the `display` classes.
///
/// [MDN: `display`](https://developer.mozilla.org/en-US/docs/Web/CSS/display)
#[macro_export]
macro_rules! display {
    () => {
        $crate::style_methods! {
            block : "block",
            inline_block : "inline-block",
            inline : "inline",
            flex : "flex",
            inline_flex : "inline-flex",
            grid : "grid",
            inline_grid : "inline-grid",
            hidden : "hidden"
        }
    };
}

/// Generates style methods for the `position` classes.
///
/// [MDN: `position`](https://developer.mozilla.org/en-US/docs/Web/CSS/position)
#[macro_export]
macro_rules! position {
    () => {
        $crate::style_methods! {
            r#static : "static",
            relative : "relative",
            absolute : "absolute",
            fixed : "fixed",
            sticky : "sticky"
        }
    };
}

/// Generates style methods for the `appearance` classes.
///
/// [MDN: `appearance`](https://developer.mozilla.org/en-US/docs/Web/CSS/appearance)
#[macro_export]
macro_rules! appearance {
    () => {
        $crate::style_methods! {
            appearance_none : "appearance-none",
            appearance_auto : "appearance-auto"
        }
    };
}

/// Generates style methods for the flexbox and CSS grid classes.
#[macro_export]
macro_rules! flex_grid {
    () => {
        $crate::style_methods! {
            flex_1 : "flex-1",
            flex_auto : "flex-auto",
            flex_initial : "flex-initial",
            flex_none : "flex-none"
        }

        $crate::style_methods! {
            flex_row : "flex-row",
            flex_row_reverse : "flex-row-reverse",
            flex_col : "flex-col",
            flex_col_reverse : "flex-col-reverse"
        }

        $crate::style_methods! {
            flex_wrap : "flex-wrap",
            flex_wrap_reverse : "flex-wrap-reverse",
            flex_nowrap : "flex-nowrap"
        }

        $crate::style_methods! {
            gap_x_0 : "gap-x-0",
            gap_x_1 : "gap-x-1",
            gap_x_2 : "gap-x-2",
            gap_x_3 : "gap-x-3",
            gap_x_4 : "gap-x-4",
            gap_x_5 : "gap-x-5",
            gap_x_6 : "gap-x-6",
            gap_x_7 : "gap-x-7",
            gap_x_8 : "gap-x-8"
        }

        $crate::style_methods! {
            gap_y_0 : "gap-y-0",
            gap_y_1 : "gap-y-1",
            gap_y_2 : "gap-y-2",
            gap_y_3 : "gap-y-3",
            gap_y_4 : "gap-y-4",
            gap_y_5 : "gap-y-5",
            gap_y_6 : "gap-y-6",
            gap_y_7 : "gap-y-7",
            gap_y_8 : "gap-y-8"
        }
    };
}

/// Generates style methods for the `align-items` classes.
///
/// [MDN: `align-items`](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)
#[macro_export]
macro_rules! align_items {
    () => {
        $crate::style_methods! {
            items_start : "items-start",
            items_end : "items-end",
            items_center : "items-center",
            items_baseline : "items-baseline",
            items_stretch : "items-stretch"
        }
    };
}

/// Generates style methods for the `justify-content` classes.
///
/// [MDN: `justify-content`](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content)
#[macro_export]
macro_rules! justify_content {
    () => {
        $crate::style_methods! {
            justify_start : "justify-start",
            justify_end : "justify-end",
            justify_center : "justify-center",
            justify_between : "justify-between",
            justify_around : "justify-around",
            justify_evenly : "justify-evenly"
        }
    };
}

/// Generates style methods for the `gap` classes.
///
/// [MDN: `gap`](https://developer.mozilla.org/en-US/docs/Web/CSS/gap)
#[macro_export]
macro_rules! gap {
    () => {
        $crate::style_methods! {
            gap_0 : "gap-0",
            gap_1 : "gap-1",
            gap_2 : "gap-2",
            gap_3 : "gap-3",
            gap_4 : "gap-4",
            gap_5 : "gap-5",
            gap_6 : "gap-6",
            gap_7 : "gap-7",
            gap_8 : "gap-8"
        }
    };
}

/// Generates style methods for the `grid-template-columns` classes.
///
/// [MDN: `grid-template-columns`](https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns)
#[macro_export]
macro_rules! grid_template_columns {
    () => {
        $crate::style_methods! {
            grid_cols_1 : "grid-cols-1",
            grid_cols_2 : "grid-cols-2",
            grid_cols_3 : "grid-cols-3",
            grid_cols_4 : "grid-cols-4",
            grid_cols_5 : "grid-cols-5",
            grid_cols_6 : "grid-cols-6",
            grid_cols_7 : "grid-cols-7",
            grid_cols_8 : "grid-cols-8",
            grid_cols_9 : "grid-cols-9",
            grid_cols_10 : "grid-cols-10",
            grid_cols_11 : "grid-cols-11",
            grid_cols_12 : "grid-cols-12",
            grid_cols_none : "grid-cols-none",
            grid_cols_subgrid : "grid-cols-subgrid"
        }
    };
}

/// Generates style methods for the `font-size` classes.
///
/// [MDN: `font-size`](https://developer.mozilla.org/en-US/docs/Web/CSS/font-size)
#[macro_export]
macro_rules! font_size {
    () => {
        $crate::style_methods! {
            font_size_1 : "font-size-1",
            font_size_2 : "font-size-2",
            font_size_3 : "font-size-3",
            font_size_4 : "font-size-4",
            font_size_5 : "font-size-5",
            font_size_6 : "font-size-6",
            font_size_7 : "font-size-7",
            font_size_8 : "font-size-8"
        }
    };
}

/// Generates style methods for the `font-weight` classes.
///
/// [MDN: `font-weight`](https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight)
#[macro_export]
macro_rules! font_weight {
    () => {
        $crate::style_methods! {
            font_weight_1 : "font-weight-1",
            font_weight_2 : "font-weight-2",
            font_weight_3 : "font-weight-3",
            font_weight_4 : "font-weight-4",
            font_weight_5 : "font-weight-5",
            font_weight_6 : "font-weight-6",
            font_weight_7 : "font-weight-7",
            font_weight_8 : "font-weight-8",
            font_weight_9 : "font-weight-9"
        }
    };
}

/// Generates style methods for the leading (`line-height`) classes.
///
/// [MDN: `line-height`](https://developer.mozilla.org/en-US/docs/Web/CSS/line-height)
#[macro_export]
macro_rules! leading {
    () => {
        $crate::style_methods! {
            leading_none : "leading-none",
            leading_tight : "leading-tight",
            leading_snug : "leading-snug",
            leading_normal : "leading-normal",
            leading_relaxed : "leading-relaxed",
            leading_loose : "leading-loose"
        }
    };
}

/// Generates style methods for the tracking (`letter-spacing`) classes.
///
/// [MDN: `letter-spacing`](https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing)
#[macro_export]
macro_rules! tracking {
    () => {
        $crate::style_methods! {
            tracking_tighter : "tracking-tighter",
            tracking_tight : "tracking-tight",
            tracking_normal : "tracking-normal",
            tracking_wide : "tracking-wide",
            tracking_wider : "tracking-wider",
            tracking_widest : "tracking-widest"
        }
    };
}

/// Generates style methods for the `text-align` classes.
///
/// [MDN: `text-align`](https://developer.mozilla.org/en-US/docs/Web/CSS/text-align)
#[macro_export]
macro_rules! text_align {
    () => {
        $crate::style_methods! {
            text_left : "text-left",
            text_center : "text-center",
            text_right : "text-right",
            text_justify : "text-justify"
        }
    };
}

/// Generates style methods for the `text-decoration` classes.
///
/// [MDN: `text-decoration`](https://developer.mozilla.org/en-US/docs/Web/CSS/text-decoration)
#[macro_export]
macro_rules! text_decoration {
    () => {
        $crate::style_methods! {
            underline : "underline",
            no_underline : "no-underline",
            line_through : "line-through",
            overline : "overline"
        }

        $crate::style_methods! {
            hover_underline : "hover:underline",
            hover_no_underline : "hover:no-underline",
            hover_line_through : "hover:line-through",
            hover_overline : "hover:overline"
        }
    };
}

/// Generates style methods for the `text-transform` classes.
///
/// [MDN: `text-transform`](https://developer.mozilla.org/en-US/docs/Web/CSS/text-transform)
#[macro_export]
macro_rules! text_transform {
    () => {
        $crate::style_methods! {
            uppercase : "uppercase",
            lowercase : "lowercase",
            capitalize : "capitalize",
            normal_case : "normal-case"
        }
    };
}

/// Generates style methods for the `list-style-type` classes.
///
/// [MDN: `list-style-type`](https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-type)
#[macro_export]
macro_rules! list_style_type {
    () => {
        $crate::style_methods! {
            list_none : "list-none",
            list_disc : "list-disc",
            list_decimal : "list-decimal"
        }
    };
}

/// Generates style methods for the `border-width` classes.
///
/// [MDN: `border-width`](https://developer.mozilla.org/en-US/docs/Web/CSS/border-width)
#[macro_export]
macro_rules! border_width {
    () => {
        $crate::style_methods! {
            border_0 : "border-0",
            border_1 : "border-1",
            border_2 : "border-2",
            border_3 : "border-3",
            border_4 : "border-4",
            border_5 : "border-5",
            border_px : "border-px"
        }

        $crate::style_methods! {
            border_t_0 : "border-t-0",
            border_t_1 : "border-t-1",
            border_t_2 : "border-t-2",
            border_t_3 : "border-t-3",
            border_t_4 : "border-t-4",
            border_t_5 : "border-t-5",
            border_t_px : "border-t-px"
        }

        $crate::style_methods! {
            border_r_0 : "border-r-0",
            border_r_1 : "border-r-1",
            border_r_2 : "border-r-2",
            border_r_3 : "border-r-3",
            border_r_4 : "border-r-4",
            border_r_5 : "border-r-5",
            border_r_px : "border-r-px"
        }

        $crate::style_methods! {
            border_b_0 : "border-b-0",
            border_b_1 : "border-b-1",
            border_b_2 : "border-b-2",
            border_b_3 : "border-b-3",
            border_b_4 : "border-b-4",
            border_b_5 : "border-b-5",
            border_b_px : "border-b-px"
        }

        $crate::style_methods! {
            border_l_0 : "border-l-0",
            border_l_1 : "border-l-1",
            border_l_2 : "border-l-2",
            border_l_3 : "border-l-3",
            border_l_4 : "border-l-4",
            border_l_5 : "border-l-5",
            border_l_px : "border-l-px"
        }

        $crate::style_methods! {
            border_x_0 : "border-x-0",
            border_x_1 : "border-x-1",
            border_x_2 : "border-x-2",
            border_x_3 : "border-x-3",
            border_x_4 : "border-x-4",
            border_x_5 : "border-x-5",
            border_x_px : "border-x-px"
        }

        $crate::style_methods! {
            border_y_0 : "border-y-0",
            border_y_1 : "border-y-1",
            border_y_2 : "border-y-2",
            border_y_3 : "border-y-3",
            border_y_4 : "border-y-4",
            border_y_5 : "border-y-5",
            border_y_px : "border-y-px"
        }
    };
}

/// Generates style methods for the `border-style` classes.
///
/// [MDN: `border-style`](https://developer.mozilla.org/en-US/docs/Web/CSS/border-style)
#[macro_export]
macro_rules! border_style {
    () => {
        $crate::style_methods! {
            border_solid : "border-solid",
            border_dashed : "border-dashed",
            border_dotted : "border-dotted",
            border_double : "border-double",
            border_hidden : "border-hidden",
            border_none : "border-none"
        }
    };
}

/// Generates style methods for the `border-radius` classes.
///
/// [MDN: `border-radius`](https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius)
#[macro_export]
macro_rules! border_radius {
    () => {
        $crate::style_methods! {
            rounded_0 : "rounded-0",
            rounded_1 : "rounded-1",
            rounded_2 : "rounded-2",
            rounded_3 : "rounded-3",
            rounded_4 : "rounded-4",
            rounded_5 : "rounded-5",
            rounded_6 : "rounded-6",
            rounded_7 : "rounded-7",
            rounded_8 : "rounded-8",
            rounded_full : "rounded-full"
        }
    };
}
