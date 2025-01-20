use std::collections::BTreeMap;

use leptos::{html::Div, prelude::*};

use crate::Direction;

#[derive(Copy, Clone)]
pub struct PaneContext {
    pub index: usize,
    pub size: RwSignal<f64>,
    pub pane_ref: NodeRef<Div>,
}

impl Default for PaneContext {
    fn default() -> Self {
        Self {
            index: 0,
            size: RwSignal::new(50.0),
            pane_ref: NodeRef::new(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct PaneResizerContext {
    pub direction: Direction,
    pub group_ref: NodeRef<Div>,
    pub resizing: RwSignal<bool>,
    pub resizer_ref: NodeRef<Div>,
    pub panes: RwSignal<BTreeMap<usize, PaneContext>>,
}

impl Default for PaneResizerContext {
    fn default() -> Self {
        Self {
            direction: Direction::Horizontal,
            group_ref: NodeRef::new(),
            resizing: RwSignal::new(false),
            resizer_ref: NodeRef::new(),
            panes: RwSignal::new(BTreeMap::new()),
        }
    }
}

impl PaneResizerContext {
    pub fn next_index(&self) -> usize {
        self.panes.get_untracked().len()
    }

    pub fn upsert_pane(&self, index: usize, pane: PaneContext) {
        self.panes.update(|panes| {
            *panes.entry(index).or_insert(pane) = pane;
        });
    }

    pub fn remove_pane(&self, index: usize) {
        self.panes.update(|panes| {
            panes.remove(&index);
        });
    }
}
