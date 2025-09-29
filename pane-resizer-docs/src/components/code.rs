use jspackages::shiki::code_to_html;
use leptos::prelude::*;
use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

use crate::utils::align_indentation;

#[component]
pub fn Code(
    #[prop(into, optional)] class: &'static str,
    code: &'static str,
    language: &'static str,
    /// Align the code block with the indentation of the shortest identation.
    #[prop(default = true)]
    align_indent: bool,
    /// Trim the code block.
    #[prop(default = true)]
    trim: bool,
) -> impl IntoView {
    let UseColorModeReturn { mode, .. } = use_color_mode();

    let (_highlighted, set_highlighted) = signal(String::new());

    let eff = RenderEffect::new(move |_| {
        let theme = match mode.get() {
            ColorMode::Dark => "vesper",
            ColorMode::Light => "solarized-light",
            _ => "vesper",
        };

        let code = if align_indent {
            align_indentation(code.to_string())
        } else {
            code.to_string()
        };

        let code = if trim { code.trim() } else { &code };

        let code = code_to_html(code, language, theme);

        set_highlighted.set(code);
    });

    on_cleanup(move || {
        drop(eff);
    });

    view! { <code inner_html=_highlighted class=class></code> }
}
