use leptos::prelude::*;
use leptos_meta::Title;
use pane_resizer::*;

use crate::icons::DotsSixVertical;

#[component]
pub fn IntroPage() -> impl IntoView {
    view! {
        <Title text="Intro" />
        <div class="m-4">
            <PaneGroup class="w-full">
                <Pane default_size=40.0 class="rounded-lg bg-gray-100 dark:bg-gray-900">
                    <div class="flex h-[400px] items-center justify-center p-6">
                        <span class="font-semibold text-black dark:text-white">"Left"</span>
                    </div>
                </Pane>
                <PaneResizer class="relative flex w-2 items-center justify-center bg-background data-[resizing=true]:*:bg-active">
                    <div class="z-5 flex h-7 w-5 items-center justify-center rounded-sm border bg-brand">
                        <DotsSixVertical class="size-4 text-black" />
                    </div>
                </PaneResizer>
                <Pane default_size=60.0 class="rounded-lg bg-gray-100 dark:bg-gray-900">
                    <div class="flex h-full items-center justify-center p-6">
                        <span class="font-semibold text-black dark:text-white">"Right"</span>
                    </div>
                </Pane>
            </PaneGroup>
        </div>
        <div class="m-4 h-[400px]">
            <PaneGroup direction=Direction::Vertical class="w-full rounded-lg">
                <Pane default_size=50.0 class="rounded-lg bg-gray-100 dark:bg-gray-900">
                    <div class="flex h-[200px] items-center justify-center p-6">
                        <span class="font-semibold text-black dark:text-white">"Top"</span>
                    </div>
                </Pane>
                <PaneResizer class="relative flex h-2 items-center justify-center bg-background data-[resizing=true]:*:bg-active">
                    <div class="z-5 flex h-5 w-7 items-center justify-center rounded-sm border bg-brand">
                        <DotsSixVertical class="size-4 text-black" />
                    </div>
                </PaneResizer>
                <Pane default_size=50.0 class="rounded-lg bg-gray-100 dark:bg-gray-900">
                    <div class="flex h-full items-center justify-center p-6">
                        <span class="font-semibold text-black dark:text-white">"Bottom"</span>
                    </div>
                </Pane>
            </PaneGroup>
        </div>
        <div class="m-4">
            <PaneGroup class="w-full">
                <Pane default_size=50.0>
                    <div class="flex h-[400px] items-center justify-center rounded-lg bg-gray-100 p-6 dark:bg-gray-900">
                        <span class="font-semibold text-black dark:text-white">"One"</span>
                    </div>
                </Pane>
                <PaneResizer class="relative flex w-2 items-center justify-center bg-background data-[resizing=true]:*:bg-active">
                    <div class="z-5 flex h-7 w-5 items-center justify-center rounded-sm border bg-brand">
                        <DotsSixVertical class="size-4 text-black" />
                    </div>
                </PaneResizer>
                <Pane default_size=50.0>
                    <PaneGroup direction=Direction::Vertical>
                        <Pane default_size=50.>
                            <div class="flex h-full items-center justify-center rounded-lg bg-gray-100 p-6 dark:bg-gray-900">
                                <span class="font-semibold text-black dark:text-white">"Two"</span>
                            </div>
                        </Pane>
                        <PaneResizer class="relative flex h-2 items-center justify-center bg-background data-[resizing=true]:*:bg-active">
                            <div class="z-5 flex h-5 w-7 items-center justify-center rounded-sm border bg-brand">
                                <DotsSixVertical class="size-4 text-black" />
                            </div>
                        </PaneResizer>
                        <Pane default_size=50.>
                            <div class="flex h-full items-center justify-center rounded-lg bg-gray-100 p-6 dark:bg-gray-900">
                                <span class="font-semibold text-black dark:text-white">
                                    "Three"
                                </span>
                            </div>
                        </Pane>
                    </PaneGroup>
                </Pane>
            </PaneGroup>
        </div>
    }
}
