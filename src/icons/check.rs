use leptos::prelude::*;

#[component]
pub fn Check(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" class=class>
            <rect width="256" height="256" fill="none" />
            <polyline
                points="40 144 96 200 224 72"
                fill="none"
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="16"
            />
        </svg>
    }
}
