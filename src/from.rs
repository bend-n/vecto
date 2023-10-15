use crate::Vector2;

impl<T> From<(T, T)> for Vector2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T: Copy> From<T> for Vector2<T> {
    /// Splats the value.
    fn from(value: T) -> Self {
        Self::splat(value)
    }
}

impl<T> From<[T; 2]> for Vector2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

impl<T: Copy> TryFrom<&[T]> for Vector2<T> {
    type Error = ();
    /// If the slice len is 2, constructs a new vec.
    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        value
            .len()
            .eq(&2)
            .then(|| Self::new(value[0], value[1]))
            .ok_or(())
    }
}

impl<T> From<Vector2<T>> for (T, T) {
    /// Tuplifys the vec, (x, y).
    fn from(value: Vector2<T>) -> Self {
        (value.x, value.y)
    }
}
