use leptos::prelude::*;
use leptos_meta::Title;
use pane_resizer::*;

use crate::icons::DotsSixVertical;

#[component]
pub fn HorizontalGroupExamplePage() -> impl IntoView {
    view! {
        <Title text="Horizontal Group Example" />
        <article class="flex h-full flex-col pt-16 pb-10 m-4 md:m-8">
            <h1>"Horizontal Group"</h1>
            <div>
                <PaneGroup class="w-full">
                    <Pane default_size=40.0 class="pane rounded-lg">
                        <div class="flex h-[400px] items-center justify-center p-6">
                            <span class="font-semibold text-black dark:text-white">"Left"</span>
                        </div>
                    </Pane>
                    <PaneResizer class="relative flex w-2 items-center justify-center bg-background data-[resizing=true]:*:bg-active">
                        <div class="z-5 flex h-7 w-5 items-center justify-center rounded-sm border border-[var(--color-text)] bg-brand">
                            <DotsSixVertical class="size-4 fill-[var(--color-text)]" />
                        </div>
                    </PaneResizer>
                    <Pane default_size=60.0 class="pane rounded-lg">
                        <div class="flex h-full items-center justify-center p-6">
                            <span class="font-semibold text-black dark:text-white">"Right"</span>
                        </div>
                    </Pane>
                </PaneGroup>
            </div>
        </article>
    }
}
