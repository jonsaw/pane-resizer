use leptos::prelude::*;
use leptos_router::{components::*, path};
use pages::*;

pub mod components;
pub mod icons;
pub mod pages;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <Router>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=|| view! { <Redirect path="/docs/intro" /> } />
                    <ParentRoute path=path!("/docs") view=DocsScaffold>
                        <Route path=path!("/intro") view=|| view! { <IntroPage /> } />
                    </ParentRoute>
                </Routes>
            </Router>
        }
    })
}
