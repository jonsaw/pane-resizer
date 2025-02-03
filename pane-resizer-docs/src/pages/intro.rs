use leptos::prelude::*;
use leptos_meta::Title;
use pane_resizer::*;

use crate::icons::DotsSixVertical;

#[component]
pub fn IntroPage() -> impl IntoView {
    view! {
        <Title text="Introduction" />
        <article class="flex h-full flex-col pt-16 pb-10 m-4 md:m-8">
            <h1>"Introduction"</h1>
            <p class="mb-4">
                "Pane Resizer is a simple and lightweight component that allows you to resize panes horizontally or vertically. It is designed to be flexible and easy to use."
            </p>
            <p class="mb-2">"Here's an example:"</p>
            <div>
                <PaneGroup class="w-full">
                    <Pane default_size=50.0 class="pane rounded-lg">
                        <div class="flex h-[400px] items-center justify-center p-6">
                            <span class="font-semibold text-black dark:text-white">"One"</span>
                        </div>
                    </Pane>
                    <PaneResizer class="relative flex w-2 items-center justify-center bg-background data-[resizing=true]:*:bg-active">
                        <div class="z-5 flex h-7 w-5 items-center justify-center rounded-sm border border-[var(--color-text)] bg-brand">
                            <DotsSixVertical class="size-4 fill-[var(--color-text)]" />
                        </div>
                    </PaneResizer>
                    <Pane default_size=50.0>
                        <PaneGroup direction=Direction::Vertical class="w-full">
                            <Pane default_size=50.0 class="pane rounded-lg">
                                <div class="flex h-full items-center justify-center p-6">
                                    <span class="font-semibold text-black dark:text-white">
                                        "Two"
                                    </span>
                                </div>
                            </Pane>
                            <PaneResizer class="relative flex h-2 items-center justify-center bg-background data-[resizing=true]:*:bg-active">
                                <div class="z-5 flex h-5 w-7 items-center justify-center rounded-sm border border-[var(--color-text)] bg-brand">
                                    <DotsSixVertical class="size-4 fill-[var(--color-text)]" />
                                </div>
                            </PaneResizer>
                            <Pane default_size=50.0 class="pane rounded-lg">
                                <div class="flex h-full items-center justify-center p-6">
                                    <span class="font-semibold text-black dark:text-white">
                                        "Three"
                                    </span>
                                </div>
                            </Pane>
                        </PaneGroup>
                    </Pane>
                </PaneGroup>
            </div>
        </article>
    }
}
