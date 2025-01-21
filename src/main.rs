use icons::DotsSixVertical;
use leptos::prelude::*;

use pane_resizer::{Direction, Pane, PaneGroup, PaneResizer};

pub mod icons;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <div class="m-4">
                <PaneGroup class="w-full">
                    <Pane default_size=40.0 class="rounded-lg bg-muted">
                        <div class="flex h-[400px] items-center justify-center p-6">
                            <span class="font-semibold">"Left"</span>
                        </div>
                    </Pane>
                    <PaneResizer class="relative flex w-2 items-center justify-center bg-background [&>div]:data-[resizing=true]:bg-active">
                        <div class="z-10 flex h-7 w-5 items-center justify-center rounded-sm border bg-brand">
                            <DotsSixVertical class="size-4 text-black" />
                        </div>
                    </PaneResizer>
                    <Pane default_size=60.0 class="rounded-lg bg-muted">
                        <div class="flex h-full items-center justify-center p-6">
                            <span class="font-semibold">"Right"</span>
                        </div>
                    </Pane>
                </PaneGroup>
            </div>
            <div class="m-4 h-[400px]">
                <PaneGroup direction=Direction::Vertical class="w-full rounded-lg">
                    <Pane default_size=50.0 class="rounded-lg bg-muted">
                        <div class="flex h-[200px] items-center justify-center p-6">
                            <span class="font-semibold">"Top"</span>
                        </div>
                    </Pane>
                    <PaneResizer class="relative flex h-2 items-center justify-center bg-background [&>div]:data-[resizing=true]:bg-active">
                        <div class="z-10 flex h-5 w-7 items-center justify-center rounded-sm border bg-brand">
                            <DotsSixVertical class="size-4 text-black" />
                        </div>
                    </PaneResizer>
                    <Pane default_size=50.0 class="rounded-lg bg-muted">
                        <div class="flex h-full items-center justify-center p-6">
                            <span class="font-semibold">"Bottom"</span>
                        </div>
                    </Pane>
                </PaneGroup>
            </div>
            <div class="m-4">
                <PaneGroup class="w-full">
                    <Pane default_size=50.0>
                        <div class="flex h-[400px] items-center justify-center rounded-lg bg-muted p-6">
                            <span class="font-semibold">"One"</span>
                        </div>
                    </Pane>
                    <PaneResizer class="relative flex w-2 items-center justify-center bg-background [&>div]:data-[resizing=true]:bg-active">
                        <div class="z-10 flex h-7 w-5 items-center justify-center rounded-sm border bg-brand">
                            <DotsSixVertical class="size-4 text-black" />
                        </div>
                    </PaneResizer>
                    <Pane default_size=50.0>
                        <PaneGroup direction=Direction::Vertical>
                            <Pane default_size=50.>
                                <div class="flex h-full items-center justify-center rounded-lg bg-muted p-6">
                                    <span class="font-semibold">"Two"</span>
                                </div>
                            </Pane>
                            <PaneResizer class="relative flex h-2 items-center justify-center bg-background [&>div]:data-[resizing=true]:bg-active">
                                <div class="z-10 flex h-5 w-7 items-center justify-center rounded-sm border bg-brand">
                                    <DotsSixVertical class="size-4 text-black" />
                                </div>
                            </PaneResizer>
                            <Pane default_size=50.>
                                <div class="flex h-full items-center justify-center rounded-lg bg-muted p-6">
                                    <span class="font-semibold">"Three"</span>
                                </div>
                            </Pane>
                        </PaneGroup>
                    </Pane>
                </PaneGroup>
            </div>
        }
    })
}
