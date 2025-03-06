//! Contains useful structs and data structures used across all the codebase.

mod chunks;
mod focus;
mod increment_wrap;
mod saved_state;
mod vec2;
mod void_fills;
mod zoom_direction;

pub(crate) mod markup;
pub(crate) use chunks::Chunks;
pub(crate) use focus::Focus;
pub(crate) use increment_wrap::{decrement_wrap, increment_wrap};
pub(crate) use saved_state::SavedState;
pub(crate) use vec2::Vec2;
pub(crate) use void_fills::{void_fills, VoidFill};
pub(crate) use zoom_direction::ZoomDirection;
