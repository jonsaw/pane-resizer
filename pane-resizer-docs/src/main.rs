use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{components::*, path};
use pages::*;
use scaffold::EmptyScaffold;

pub mod components;
pub mod icons;
pub mod pages;
pub mod utils;

fn main() {
    leptos::mount::mount_to_body(|| {
        provide_meta_context();
        view! {
            <Router>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=|| view! { <Redirect path="/docs/intro" /> } />
                    <ParentRoute path=path!("/docs") view=DocsScaffold>
                        <Route path=path!("/intro") view=|| view! { <IntroPage /> } />
                        <ParentRoute path=path!("/examples") view=EmptyScaffold>
                            <Route
                                path=path!("/horizontal-group")
                                view=|| view! { <HorizontalGroupExamplePage /> }
                            />
                            <Route
                                path=path!("/vertical-group")
                                view=|| view! { <VerticalGroupExamplePage /> }
                            />
                            <Route
                                path=path!("/nested-groups")
                                view=|| view! { <NestedGroupsExamplePage /> }
                            />
                        </ParentRoute>
                    </ParentRoute>
                </Routes>
            </Router>
        }
    })
}
