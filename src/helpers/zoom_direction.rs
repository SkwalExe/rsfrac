//! Contains the `ZoomDirection` enum.

/// Represents the direction when zooming into the canvas.
/// `ZoomDirection::In` means we are "diving" into the canvas,
/// making the details bigger.
pub(crate) enum ZoomDirection {
    In,
    Out,
}
