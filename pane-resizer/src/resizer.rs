use leptos::prelude::*;

use crate::context::PaneResizerContext;

#[component]
pub fn PaneResizer(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let ctx = expect_context::<PaneResizerContext>();

    let handle_on_mousedown = move |_| {
        ctx.resizing.set(true);
    };

    let handle_on_touchstart = move |_| {
        ctx.resizing.set(true);
    };

    view! {
        <div
            node_ref=ctx.resizer_ref
            class=class
            style:cursor=ctx.direction.cursor()
            style:user-select="none"
            style:touch-action="none"
            on:mousedown=handle_on_mousedown
            on:touchstart=handle_on_touchstart
            data-resizing=move || if ctx.resizing.get() { "true" } else { "false" }
        >
            {children()}
        </div>
    }
}
