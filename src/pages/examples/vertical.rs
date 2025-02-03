use leptos::prelude::*;
use leptos_meta::Title;
use pane_resizer::*;

use crate::{components::Code, icons::DotsSixVertical};

#[component]
pub fn VerticalGroupExamplePage() -> impl IntoView {
    let example_code = r#"
    use leptos::prelude::*;
    use pane_resizer::*;

    #[component]
    pub fn HorizontalGroupExample() -> impl IntoView {
        <PaneGroup direction=Direction::Vertical>
            <Pane default_size=50.0 />
            <PaneResizer />
            <Pane default_size=50.0 />
        </PaneGroup>
    }"#;

    view! {
        <Title text="Vertical Group Example" />
        <article class="flex h-full flex-col pt-16 pb-10 m-4 md:m-8">
            <h1>"Vertical Group"</h1>
            <div class="h-[400px]">
                <PaneGroup direction=Direction::Vertical class="w-full">
                    <Pane default_size=50.0 class="pane rounded-lg">
                        <div class="flex h-[200px] items-center justify-center p-6">
                            <span class="font-semibold text-black dark:text-white">"Top"</span>
                        </div>
                    </Pane>
                    <PaneResizer class="relative flex h-2 items-center justify-center bg-background data-[resizing=true]:*:bg-active">
                        <div class="z-5 flex h-5 w-7 items-center justify-center rounded-sm border border-[var(--color-text)] bg-brand">
                            <DotsSixVertical class="size-4 fill-[var(--color-text)]" />
                        </div>
                    </PaneResizer>
                    <Pane default_size=50.0 class="pane rounded-lg">
                        <div class="flex h-full items-center justify-center p-6">
                            <span class="font-semibold text-black dark:text-white">"Bottom"</span>
                        </div>
                    </Pane>
                </PaneGroup>
            </div>
            <h2 class="mt-8">"Anatomy"</h2>
            <p>"Here's the high-level structure of the example above:"</p>
            <Code
                code=example_code
                language="rust"
                class="[&>.shiki]:overflow-x-auto [&>.shiki]:p-4 [&>.shiki]:rounded-lg [&>.shiki]:text-sm"
            />
        </article>
    }
}
