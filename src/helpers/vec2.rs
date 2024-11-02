//! Contains the (Vec2) struct.

/// Used to store two values of the same type and nature.
/// Such as height and width, abscissa and ordinate of a point, etc.
#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Vec2<T> {
    /// Creates a new `Vec2`.
    pub(crate) fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Converts a `Vec2<T>` into a `Vec2<U>`, where `U: From<T>`
    pub(crate) fn convert<U>(self) -> Vec2<U>
    where
        U: From<T>,
    {
        Vec2::new(self.x.into(), self.y.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_convert() {
        let vec_i32 = Vec2::new(1, 2);
        let vec_f64 = vec_i32.convert();
        assert_eq!(vec_f64, Vec2::new(1.0, 2.0));
    }
}
