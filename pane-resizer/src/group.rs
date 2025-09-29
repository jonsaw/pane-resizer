use leptos::{
    context::Provider,
    ev::{contextmenu, mouseup, touchend},
    prelude::*,
};
use leptos_use::{use_document, use_event_listener, use_mouse_in_element, UseMouseInElementReturn};

use crate::{context::PaneResizerContext, Style};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Style for Direction {
    fn style(&self) -> String {
        match self {
            Direction::Horizontal => "flex-direction: row".to_string(),
            Direction::Vertical => "flex-direction: column".to_string(),
        }
    }
}

impl Direction {
    pub fn cursor(&self) -> String {
        match self {
            Direction::Horizontal => "ew-resize".to_string(),
            Direction::Vertical => "ns-resize".to_string(),
        }
    }
}

#[component]
pub fn PaneGroup(
    #[prop(into, optional)] class: String,
    children: Children,
    #[prop(default = Direction::Horizontal)] direction: Direction,
) -> impl IntoView {
    let ctx = PaneResizerContext {
        direction,
        ..Default::default()
    };

    let UseMouseInElementReturn {
        element_width,
        element_height,
        element_x,
        element_y,
        ..
    } = use_mouse_in_element(ctx.group_ref);

    let handle_on_mouseup = move |_| {
        ctx.resizing.set(false);
    };

    let _ = use_event_listener(use_document(), mouseup, move |_| {
        ctx.resizing.set(false);
    });

    let _ = use_event_listener(use_document(), touchend, move |_| {
        ctx.resizing.set(false);
    });

    let _ = use_event_listener(use_document(), contextmenu, move |_| {
        ctx.resizing.set(false);
    });

    Effect::new(move |_| {
        if ctx.resizing.get() {
            let new_size = match direction {
                Direction::Horizontal => (element_x.get() / element_width.get()) * 100.0,
                Direction::Vertical => (element_y.get() / element_height.get()) * 100.0,
            };

            let panes = ctx.panes.get();

            if panes.keys().len() == 2 {
                let mut panes = panes.iter();
                let (_, first_pane_ctx) = panes.next().unwrap();
                let (_, second_pane_ctx) = panes.next().unwrap();

                let first_pane_size = new_size;
                let second_pane_size = 100.0 - new_size;

                first_pane_ctx.size.set(first_pane_size);
                second_pane_ctx.size.set(second_pane_size);
            }
        }
    });

    view! {
        <Provider value=ctx>
            <div
                node_ref=ctx.group_ref
                class=class
                style:display="flex"
                style:height="100%"
                style:width="100%"
                style:overflow="hidden"
                style=direction.style()
                on:mouseup=handle_on_mouseup
            >
                {children()}
            </div>
        </Provider>
    }
}
