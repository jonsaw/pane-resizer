use leptos::prelude::*;

#[component]
pub fn DotsSixVertical(#[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" class=class>
            <rect width="256" height="256" fill="none" />
            <circle cx="92" cy="60" r="12" />
            <circle cx="164" cy="60" r="12" />
            <circle cx="92" cy="128" r="12" />
            <circle cx="164" cy="128" r="12" />
            <circle cx="92" cy="196" r="12" />
            <circle cx="164" cy="196" r="12" />
        </svg>
    }
}
