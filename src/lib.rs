//! Provides a [`Vector2`].
//! ```
//! # use vecto::Vec2;
//! let mut v = Vec2::new(5.0, 7.0);
//! v *= 2.0;
//! assert_eq!(v, Vec2::new(10.0, 14.0));
//! ````
#![allow(mixed_script_confusables)]
#![warn(clippy::pedantic, clippy::dbg_macro, missing_docs)]
mod from;
mod ops;

#[doc(hidden)]
pub trait Kinda
where
    Self: Sized,
{
    fn kinda_eq(self, other: Self, tolerance: f32) -> bool;
    fn approx_eq(self, other: Self) -> bool;
}

impl Kinda for f32 {
    fn kinda_eq(self, other: Self, tolerance: f32) -> bool {
        if self == other {
            true
        } else {
            (self - other).abs() < tolerance
        }
    }

    fn approx_eq(self, other: Self) -> bool {
        self.kinda_eq(other, 0.00001)
    }
}

impl Kinda for Vec2 {
    fn kinda_eq(self, other: Self, tolerance: f32) -> bool {
        self.x.kinda_eq(other.x, tolerance) && self.y.kinda_eq(other.y, tolerance)
    }

    fn approx_eq(self, other: Self) -> bool {
        self.kinda_eq(other, 0.00001)
    }
}

use umath::generic_float::{FloatAlone, Rounding};

/// Alias for <code>[`Vector2`]<[`f32`]></code>
pub type Vec2 = Vector2<f32>;

/// Vector2.
#[derive(Copy, Clone, PartialEq, PartialOrd, Default, Hash, Eq, Ord)]
#[repr(C)]
pub struct Vector2<T> {
    /// The vector's X component.
    pub x: T,
    /// The vector's Y component.
    pub y: T,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<T> Vector2<T> {
    /// Construct a new [`Vector2`].
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy> Vector2<T> {
    /// Construct a new [`Vector2`] with x and y set to the given value.
    pub const fn splat(x: T) -> Self {
        Self { x, y: x }
    }
}

impl Vec2 {
    /// Zero unit vector. `(0, 0)`
    pub const ZERO: Vec2 = Vec2::new(0.0, 0.0);
    /// Right unit vector. `(1, 0)`
    pub const RIGHT: Vec2 = Vec2::new(1.0, 0.0);
    /// Left unit vector. `(-1, 0)`
    pub const LEFT: Vec2 = Vec2::new(-1.0, 0.0);
    /// Up unit vector. Y-Down, so points -Y. `(0, -1)`
    pub const UP: Vec2 = Vec2::new(0.0, -1.0);
    /// Down unit vector. Y-Down, so points +Y. `(0, 1)`
    pub const DOWN: Vec2 = Vec2::new(0.0, 1.0);
}

impl Vector2<f64> {
    /// Zero unit vector. `(0, 0)`
    pub const ZERO: Vec2 = Vec2::new(0.0, 0.0);
    /// Right unit vector. `(1, 0)`
    pub const RIGHT: Vec2 = Vec2::new(1.0, 0.0);
    /// Left unit vector. `(-1, 0)`
    pub const LEFT: Vec2 = Vec2::new(-1.0, 0.0);
    /// Up unit vector. Y-Down, so points -Y. `(0, -1)`
    pub const UP: Vec2 = Vec2::new(0.0, -1.0);
    /// Down unit vector. Y-Down, so points +Y. `(0, 1)`
    pub const DOWN: Vec2 = Vec2::new(0.0, 1.0);
}

impl<T: std::ops::Neg<Output = T>> Vector2<T> {
    /// Returns a perpendicular vector, rotated 90 degrees counter-clockwise, with the same length.
    #[must_use = "Does not modify in place."]
    pub fn orthogonal(self) -> Self {
        Self::new(self.y, -self.x)
    }
}

impl<T: FloatAlone> Vector2<T> {
    /// Creates a unit [`Vector2`] rotated to the given angle (radians).
    /// This is equivalent to `Vec2::new(angle.cos(), angle.sin())`.
    /// ```
    /// # use vecto::{Vec2, Kinda};
    /// # use std::f32::consts::PI;
    /// assert_eq!(Vec2::from_angle(0.0), Vec2::RIGHT);
    /// assert_eq!(Vec2::RIGHT.angle(), 0.0);
    /// assert!(Vec2::from_angle(PI / 2.0).approx_eq(Vec2::new(0.0, 1.0)));
    /// ```
    pub fn from_angle(angle: T) -> Self {
        Self::new(angle.cos(), angle.sin())
    }

    /// Returns a new vector with all components in absolute values (i.e. positive).
    #[must_use = "Does not modify in place."]
    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    /// Returns this vector's angle with respect to the positive X axis, or the [`Vec2::RIGHT`] vector, in radians.
    /// ```
    /// # use vecto::Vec2;
    /// # use std::f32::consts::PI;
    /// assert_eq!(Vec2::RIGHT.angle(), 0.0);
    /// assert_eq!(Vec2::DOWN.angle(), PI / 2.0); // 90 degrees
    /// assert_eq!(Vec2::new(1.0, -1.0).angle(), -PI / 4.0); // -45 degrees
    /// ```
    pub fn angle(&self) -> T {
        self.y.atan2(self.x)
    }

    /// Returns the cross product of `self` and `with`.
    pub fn cross(&self, with: &Self) -> T {
        self.x * with.y - self.y * with.x
    }

    /// Returns the distance from `self` to `to`.
    pub fn distance_to(&self, to: &Self) -> T {
        ((self.x - to.x) * (self.x - to.x) + (self.y - to.y) * (self.y - to.y)).sqrt()
    }

    /// Returns the dot product of `self` and `with`.
    pub fn dot(&self, with: &Self) -> T {
        self.x * with.x + self.y * with.y
    }

    /// Returns the length(magnitude) of `self`.
    /// ```
    /// # use vecto::Vec2;
    /// assert_eq!(Vec2::splat(10.0).length(), 10.0 * 2.0f32.sqrt());
    /// ```
    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns the squared length of `self`. Faster than [`Self::length`].
    /// ```
    /// # use vecto::Vec2;
    /// assert_eq!(Vec2::splat(10.0).length_squared(), 200.0);
    /// ```
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    /// Returns the vector with a new maximum length.
    /// ```
    /// # use vecto::{Kinda, Vec2};
    /// assert!(Vec2::splat(10.).limit_length(1.0).approx_eq(Vec2::splat(1.0 / 2.0f32.sqrt())));
    /// assert!(Vec2::splat(10.).limit_length(5.0).approx_eq(Vec2::splat(1.0 / 2.0f32.sqrt() * 5.0)));
    /// ```
    #[must_use = "Does not modify in place."]
    pub fn limit_length(self, len: T) -> Self {
        let l = self.length();
        if l > unsafe { T::zero() } && len < l {
            return (self / l) * len;
        }
        self
    }

    /// Returns the result of scaling the vector to unit length.
    /// Equivalent to v / v.length().
    ///
    /// Note: This function may struggle with denormal values.
    /// ```
    /// # use vecto::{Kinda, Vec2};
    /// assert!(Vec2::RIGHT.normalized().approx_eq(Vec2::RIGHT));
    /// assert!(Vec2::splat(1.0).normalized().approx_eq(Vec2::splat(0.5f32.sqrt())));
    /// ```
    #[must_use = "Does not modify in place."]
    pub fn normalized(self) -> Self {
        let l = self.length_squared();
        if l != unsafe { T::zero() } {
            return self / l.sqrt();
        }
        self
    }

    /// Rotates this vector by `angle` radians.
    /// ```
    /// # use vecto::{Kinda, Vec2};
    /// # use std::f32::consts::TAU;
    /// let v = Vec2::new(1.2, 3.4);
    /// assert!(v.rotated(TAU).approx_eq(Vec2::new(1.2, 3.4))); // full circle rotation
    /// assert!(v.rotated(TAU / 4.0).approx_eq(Vec2::new(-3.4, 1.2)));
    /// assert!(v.rotated(TAU / 3.0).approx_eq(Vec2::new(-3.5444863, -0.6607695)));
    /// assert!(v.rotated(TAU / 2.0).approx_eq(v.rotated(TAU / -2.0)));
    /// ```
    #[must_use = "Does not modify in place."]
    pub fn rotated(self, angle: T) -> Self {
        Vector2::new(
            self.x * angle.cos() - self.y * angle.sin(),
            self.x * angle.sin() + self.y * angle.cos(),
        )
    }
}

impl<T: Rounding> Vector2<T> {
    /// Returns a new vector with all components rounded up (towards positive infinity).
    #[must_use = "Does not modify in place."]
    pub fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }

    /// Returns a new vector with all components rounded down (towards negative infinity).
    #[must_use = "Does not modify in place."]
    pub fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }
}
