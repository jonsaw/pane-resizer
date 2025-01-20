pub mod context;
pub mod group;
pub mod pane;
pub mod resizer;

pub use group::{Direction, PaneGroup};
pub use pane::Pane;
pub use resizer::PaneResizer;

pub trait Style {
    fn style(&self) -> String;
}
