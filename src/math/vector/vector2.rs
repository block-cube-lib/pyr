use super::prelude::*;
use super::{Vector, VectorElement};
use num::{pow, One, Zero};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector2<T: VectorElement> {
    pub x: T,
    pub y: T,
}

impl<T: VectorElement> Vector for Vector2<T> {
    type ElementType = T;
    const DIMENSION: usize = 2;
}

impl<T: VectorElement + Eq> Eq for Vector2<T> {}

//============================================================
// operator
//============================================================
impl_ops!(Vector2; x, y);

//============================================================
// impl num
//============================================================
/// Make a zero vector.
/// Same to Vector2 { x: 0, y: 0 }.
impl<T: VectorElement> Zero for Vector2<T> {
    fn zero() -> Self {
        Self::from_scalar(T::zero())
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

/// Make a Vector2 { x: 1, y: 1 }.
impl<T: VectorElement> One for Vector2<T> {
    fn one() -> Self {
        Self::from_scalar(T::one())
    }
}

//============================================================
// prelude
//============================================================
impl<T: VectorElement> Vector2<T> {
    /// Make a Vector2 from x and y
    pub fn new(x: T, y: T) -> Self {
        Vector2::<T> { x, y }
    }

    /// Make a unit vector.
    pub fn unit_x() -> Self {
        Vector2::new(T::one(), T::zero())
    }

    /// ```
    /// use pyrite::math::vector2::Vector2;
    /// let v = Vector2::unit_y();
    /// assert_eq!(v, Vector2::new(0, 1));
    /// ```
    pub fn unit_y() -> Self {
        Vector2::new(T::zero(), T::one())
    }
}

impl<T: VectorElement> FromScalar for Vector2<T> {
    fn from_scalar(scalar: T) -> Self {
        Self::new(scalar, scalar)
    }
}

impl<T: VectorElement> LengthSquared for Vector2<T> {
    /// Return the square of the length.
    fn length_squared(&self) -> T {
        pow(self.x, 2) + pow(self.y, 2)
    }
}

impl<T: VectorElement> Dot for Vector2<T> {
    /// Dot product.
    fn dot(&self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: VectorElement> Cross for Vector2<T> {
    type Output = T;

    /// Cross product.
    fn cross(&self, rhs: Self) -> Self::Output {
        self.x * rhs.y - self.y * rhs.x
    }
}

//============================================================
// from
//============================================================
impl<T: VectorElement> From<[T; 2]> for Vector2<T> {
    fn from(v: [T; 2]) -> Vector2<T> {
        Vector2::<T> { x: v[0], y: v[1] }
    }
}

impl<T: VectorElement> From<(T, T)> for Vector2<T> {
    fn from(v: (T, T)) -> Vector2<T> {
        Vector2::<T> { x: v.0, y: v.1 }
    }
}

//============================================================
// fmt
//============================================================
impl<T: std::fmt::Display + VectorElement> std::fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::Vector2;

    mod ops {
        use super::Vector2;
        use proptest::*;
        ops_test!(Vector2; x, y);
    }
}
