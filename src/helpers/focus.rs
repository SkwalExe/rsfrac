//! Contains the (Focus) enum.

/// Used to track which component is focused
#[derive(Default, PartialEq, Debug)]
pub(crate) enum Focus {
    #[default]
    Canvas,
    LogPanel,
    Input,
}
