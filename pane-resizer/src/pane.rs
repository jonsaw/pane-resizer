use leptos::prelude::*;

use crate::context::{PaneContext, PaneResizerContext};

#[component]
pub fn Pane(
    #[prop(into, optional)] class: String,
    #[prop(default = 50.0)] default_size: f64,
    children: Children,
) -> impl IntoView {
    let group_ctx = expect_context::<PaneResizerContext>();

    let index = group_ctx.next_index();

    let pane_ctx = PaneContext {
        index,
        size: RwSignal::new(default_size),
        ..Default::default()
    };

    group_ctx.upsert_pane(index, pane_ctx);

    on_cleanup(move || {
        group_ctx.remove_pane(index);
    });

    view! {
        <div
            node_ref=pane_ctx.pane_ref
            class=class
            style:flex=move || format!("{} 1 0px", pane_ctx.size.get())
            style:overflow="hidden"
            data-index=index
        >
            {children()}
        </div>
    }
}
