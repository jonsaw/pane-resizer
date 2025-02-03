use std::time::Duration;

use biji_ui::components::dialog::{self as dialogui, context::DialogContext};
use leptos::{portal::Portal, prelude::*};
use leptos_router::{components::Outlet, hooks::use_location};
use leptos_use::use_media_query;

use crate::{components::ThemeMode, icons};

#[component]
pub fn DocsScaffold() -> impl IntoView {
    view! {
        <div class="h-full lg:ml-72 xl:ml-80">
            <header class="contents lg:fixed lg:inset-0 lg:flex lg:pointer-events-none lg:z-40">
                <div class="contents lg:block lg:overflow-y-auto lg:pointer-events-auto lg:w-72 lg:border-r lg:border-zinc-900/10 lg:px-6 lg:pb-8 lg:pt-4 lg:dark:border-white/10 xl:w-80">
                    <div class="hidden lg:flex">
                        <a class="text-black dark:text-white" aria-label="Home" href="/">
                            "Pane Resizer"
                        </a>
                    </div>
                    <TopNav />
                    <SidebarNav class="hidden lg:block lg:mt-10" />
                </div>
            </header>
            <main class="mt-15">
                <Outlet />
            </main>
        </div>
    }
}

#[component]
pub fn TopNav() -> impl IntoView {
    view! {
        <div
            style="--bg-opacity-light: 0.5; --bg-opacity-dark: 0.2; --scrollbar-width-nav: var(--scrollbar-width, 0px);"
            class="fixed inset-x-0 top-0 z-50 flex h-14 items-center justify-between gap-12 pl-4 pr-[calc(var(--scrollbar-width-nav)+1rem)] transition sm:pl-6 sm:pr-[calc(var(--scrollbar-width-nav)+1.5rem)] lg:left-72 lg:z-30 lg:pl-8 lg:pr-[calc(var(--scrollbar-width-nav)+2rem)] xl:left-80 backdrop-blur-sm lg:left-72 xl:left-80 dark:backdrop-blur bg-white/[var(--bg-opacity-light)] dark:bg-zinc-900/[var(--bg-opacity-dark)]"
        >
            <div class="absolute inset-x-0 top-full h-px bg-zinc-900/10 transition dark:bg-white/10"></div>
            <div class="hidden lg:block lg:max-w-md lg:flex-auto"></div>
            <div class="flex items-center gap-5 lg:hidden">
                <Sidebar />
                <a class="text-black dark:text-white" aria-label="Home" href="/">
                    "Pane Resizer"
                </a>
            </div>
            <div class="flex items-center gap-5">
                <nav class="hidden md:block">
                    <ul role="list" class="flex items-center gap-5">
                        <li>
                            <a
                                class="text-sm leading-5 text-zinc-600 transition dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                href="https://docs.rs/pane-resizer/latest/pane_resizer/"
                                title="Documentation"
                            >
                                <icons::BookText class="h-5 w-5"></icons::BookText>
                            </a>
                        </li>
                        <li>
                            <a
                                class="text-sm leading-5 text-zinc-600 transition dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                href="https://github.com/jonsaw/pane-resizer"
                                title="Github"
                            >
                                <icons::Github class="h-5 w-5"></icons::Github>
                            </a>
                        </li>
                        <li>
                            <a
                                class="text-sm leading-5 text-zinc-600 transition dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                href="https://github.com/jonsaw/pane-resizer/issues"
                                title="Report an issue"
                            >
                                <icons::Bug class="h-5 w-5"></icons::Bug>
                            </a>
                        </li>
                    </ul>
                </nav>
                <div class="hidden md:block md:h-5 md:bg-zinc-900/10 md:w-px md:dark:bg-white/15"></div>
                <div class="flex gap-4">
                    <ThemeMode />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SidebarTrigger() -> impl IntoView {
    let ctx = expect_context::<DialogContext>();

    let is_large_screen = use_media_query("(min-width: 1024px)");

    Effect::new(move |_| {
        if is_large_screen.get() {
            ctx.close();
        }
    });

    view! {
        <Show
            when=move || !ctx.open.get()
            fallback=|| {
                view! { <icons::X class="w-5 text-zinc-900 dark:text-white"></icons::X> }
            }
        >

            <icons::AlignJustify class="w-5 text-zinc-900 dark:text-white"></icons::AlignJustify>
        </Show>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <dialogui::Root hide_delay=Duration::from_millis(300)>
            <dialogui::Trigger class="flex h-6 w-6 items-center justify-center rounded-md transition dark:hover:bg-white/5 hover:bg-zinc-900/5">
                <SidebarTrigger />
            </dialogui::Trigger>
            <Portal>
                <dialogui::Overlay
                    class="fixed z-30 inset-0 top-14 bg-zinc-400/20 backdrop-blur-sm transition-opacity duration-300 ease-linear dark:bg-black/40"
                    show_class="opacity-100"
                    hide_class="opacity-0"
                ></dialogui::Overlay>
                <dialogui::Content
                    class="fixed z-30 bottom-0 left-0 top-14 w-full overflow-y-auto bg-white px-4 pb-4 pt-6 shadow-lg shadow-zinc-900/10 ring-1 ring-zinc-900/10 transition duration-300 ease-in-out min-[416px]:max-w-sm sm:pb-10 sm:px-6 dark:bg-zinc-900 dark:ring-zinc-800"
                    show_class="translate-x-0"
                    hide_class="-translate-x-full"
                >
                    <div>
                        <SidebarNav />
                    </div>
                </dialogui::Content>
            </Portal>
        </dialogui::Root>
    }
}

#[component]
pub fn SidebarNav(#[prop(into, optional)] class: String) -> impl IntoView {
    let location = use_location();

    let introduction = [("/docs/intro", "Introduction")];

    let examples = [
        ("/docs/horizontal-group", "Horizontal Group"),
        ("/docs/vertical-group", "Vertical Group"),
        ("/docs/nested-groups", "Nested Groups"),
    ];

    let dialog_ctx = use_context::<DialogContext>();

    view! {
        <nav class=class>
            <ul role="list">
                <li class="md:hidden">
                    <a
                        class="block flex py-1 text-sm text-zinc-600 transition dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                        href="https://docs.rs/pane-resizer/latest/pane_resizer/"
                    >
                        "Documentation"
                    </a>
                </li>
                <li class="md:hidden">
                    <a
                        class="block flex py-1 text-sm text-zinc-600 transition dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                        href="https://github.com/jonsaw/pane-resizer"
                    >
                        "Github"
                    </a>
                </li>
                <li class="md:hidden">
                    <a
                        class="block flex py-1 text-sm text-zinc-600 transition dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                        href="https://github.com/jonsaw/pane-resizer/issues"
                    >
                        "Report an issue"
                    </a>
                </li>
            </ul>
            <ul role="list">
                <li class="relative mt-6 md:mt-0">
                    <h2 class="text-xs font-semibold text-zinc-900 dark:text-white">
                        "Introduction"
                    </h2>
                    <ul class="border-l border-transparent">
                        {introduction
                            .into_iter()
                            .map(|(path, title)| {
                                view! {
                                    <li class="relative">
                                        <a
                                            href=path
                                            class="flex justify-between gap-2 py-1 pl-4 pr-3 text-sm text-zinc-600 transition dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                            style:font-weight=move || {
                                                if location.pathname.get() == path { "500" } else { "400" }
                                            }

                                            on:click=move |_| {
                                                if let Some(ctx) = dialog_ctx {
                                                    ctx.close();
                                                }
                                            }
                                        >

                                            {title}
                                        </a>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                </li>
                <li class="relative mt-6">
                    <h2 class="text-xs font-semibold text-zinc-900 dark:text-white">"Examples"</h2>
                    <ul class="border-l border-transparent">
                        {examples
                            .into_iter()
                            .map(|(path, title)| {
                                view! {
                                    <li class="relative">
                                        <a
                                            href=path
                                            class="flex justify-between gap-2 py-1 pl-4 pr-3 text-sm text-zinc-600 transition dark:text-zinc-400 dark:hover:text-white hover:text-zinc-900"
                                            style:font-weight=move || {
                                                if location.pathname.get() == path { "500" } else { "400" }
                                            }

                                            on:click=move |_| {
                                                if let Some(ctx) = dialog_ctx {
                                                    ctx.close();
                                                }
                                            }
                                        >

                                            {title}
                                        </a>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                </li>
            </ul>
        </nav>
    }
}
