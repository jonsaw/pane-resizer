use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn DocsScaffold() -> impl IntoView {
    view! { <Outlet /> }
}
