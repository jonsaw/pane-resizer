use std::time::Duration;

use biji_ui::components::menu;
use leptos::prelude::*;
use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

use crate::icons;

#[component]
pub fn ThemeMode() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

    let modes = [("Light", &ColorMode::Light), ("Dark", &ColorMode::Dark)];

    view! {
        <menu::Menu class="relative">
            <menu::Trigger class="flex h-6 w-6 cursor-pointer items-center justify-center rounded-md transition dark:hover:bg-white/5 hover:bg-zinc-900/5">
                <icons::Sun class="h-5 w-5 text-zinc-600 dark:hidden"></icons::Sun>
                <icons::Moon class="hidden h-5 w-5 dark:text-zinc-400 dark:block"></icons::Moon>
            </menu::Trigger>
            <menu::Content
                class="absolute right-0 flex w-40 min-w-[8rem] flex-col rounded-md border border-zinc-900/10 p-1 shadow-md transition bg-white text-zinc-600 dark:bg-zinc-900 dark:text-zinc-400 dark:border-white/10 focus:outline-none"
                show_class="z-50 translate-y-0 opacity-100 duration-150 ease-in"
                hide_class="-z-50 translate-y-1 opacity-0 duration-200 ease-out"
                hide_delay=Duration::from_millis(200)
            >
                {modes
                    .into_iter()
                    .map(|(title, m)| {
                        view! {
                            <menu::Item class="flex items-center text-sm rounded-sm cursor-pointer outline-none select-none focus:outline-none hover:bg-accent hover:text-accent-foreground !ring-0 !ring-transparent data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-muted">
                                <button
                                    on:click=move |_| { set_mode.set(m.clone()) }
                                    class="flex w-full justify-between px-2 py-1.5 align-center cursor-pointer"
                                >
                                    <div class="flex gap-2">
                                        {match m.clone() {
                                            ColorMode::Light => {
                                                view! { <icons::Sun class="w-4"></icons::Sun> }.into_any()
                                            }
                                            ColorMode::Dark => {
                                                view! { <icons::Moon class="w-4"></icons::Moon> }.into_any()
                                            }
                                            _ => {
                                                view! { <icons::SunMoon class="w-4"></icons::SunMoon> }
                                                    .into_any()
                                            }
                                        }} {title}
                                    </div>
                                    <Show when=move || m.clone() == mode.get()>
                                        <icons::Check class="w-4"></icons::Check>
                                    </Show>
                                </button>
                            </menu::Item>
                        }
                    })
                    .collect_view()}
            </menu::Content>
        </menu::Menu>
    }
}
