pub mod markup;
use ratatui::layout::Rect;

#[derive(Default, Debug, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: impl Into<T>, y: impl Into<T>) -> Self {
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
pub enum Focus {
    #[default]
    Canvas,
    LogPanel,
    Input,
}

#[derive(Default)]
pub struct Chunks {
    pub canvas: Rect,
    pub side: Rect,
    pub log_panel: Rect,
    pub input: Rect,
    pub footer: Rect,
    pub body: Rect,
    pub canvas_inner: Rect,
}

pub enum ZoomDirection {
    In,
    Out,
}

pub enum VoidFill {
    Black,
    White,
    RGBNoise,
    Trensparent,
    RedNoise, 
    GreenNoise, 
    BlueNoise
}
