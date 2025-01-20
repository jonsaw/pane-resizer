# Pane Resizer

A simple pane resizer for your Leptos apps.

[See](https://jonsaw.github.io/pane-resizer/) resizers in action.

## Getting Started

Install Pane Resizer via Cargo:

```bash
cargo add pane-resizer
```

## Basic Usage

Simple horizontal pane resizer:

```rust
use icons::DotsSixVertical;
use leptos::prelude::*;
use pane_resizer::{Pane, PaneGroup, PaneResizer};

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
                    <PaneResizer class="relative flex w-2 items-center justify-center bg-background">
                        <div class="z-10 flex h-7 w-5 items-center justify-center rounded-sm border bg-brand">
                            <DotsSixVertical class="size-4 bg-brand text-black" />
                        </div>
                    </PaneResizer>
                    <Pane default_size=60.0 class="rounded-lg bg-muted">
                        <div class="flex h-full items-center justify-center p-6">
                            <span class="font-semibold">"Right"</span>
                        </div>
                    </Pane>
                </PaneGroup>
            </div>
        }
    })
}

```
