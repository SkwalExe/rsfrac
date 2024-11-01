pub(crate) mod markup;
use ratatui::layout::Rect;

#[derive(Default, Debug, Clone)]
pub(crate) struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Vec2<T> {
    pub(crate) fn new(x: impl Into<T>, y: impl Into<T>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

// TEMPORARY
// Todo: Create an implementation for Vec2<U> -> Vec2<T> where U: Into<T>
impl From<Vec2<u16>> for Vec2<i32> {
    fn from(value: Vec2<u16>) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
        }
    }
}

/// Used to track which component is focused
#[derive(Default, PartialEq, Debug)]
pub(crate) enum Focus {
    #[default]
    Canvas,
    LogPanel,
    Input,
}

#[derive(Default)]
pub(crate) struct Chunks {
    pub(crate) canvas: Rect,
    pub(crate) side: Rect,
    pub(crate) log_panel: Rect,
    pub(crate) input: Rect,
    pub(crate) footer: Rect,
    pub(crate) body: Rect,
    pub(crate) canvas_inner: Rect,
}

pub(crate) enum ZoomDirection {
    In,
    Out,
}
