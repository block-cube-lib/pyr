use super::prelude::*;
use super::{Vector, VectorElement};
use num::{pow, One, Zero};
#[cfg(feature = "serde")]
use Serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector1<T> {
    pub x: T,
}

impl<T: VectorElement> Vector for Vector1<T> {
    type ElementType = T;
    const DIMENSION: usize = 1;
}

//============================================================
// operator
//============================================================
impl_ops!(Vector1; x);

//============================================================
// impl num
//============================================================
impl<T: VectorElement> One for Vector1<T> {
    fn one() -> Self {
        Self::from_scalar(T::one())
    }
}

impl<T: VectorElement> Zero for Vector1<T> {
    fn zero() -> Self {
        Self::from_scalar(T::zero())
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

//============================================================
// prelude
//============================================================
impl<T: VectorElement> FromScalar for Vector1<T> {
    fn from_scalar(scalar: T) -> Self {
        Vector1 { x: scalar }
    }
}

impl<T: VectorElement> Vector1<T> {
    pub fn unit_x() -> Self {
        Vector1 { x: T::one() }
    }

    pub fn new(x: T) -> Self {
        Vector1 { x }
    }
}

impl<T: VectorElement> LengthSquared for Vector1<T> {
    /// Return the square of the length.
    fn length_squared(&self) -> T {
        pow(self.x, 2)
    }
}

impl<T: VectorElement> Dot for Vector1<T> {
    /// Dot product.
    fn dot(&self, rhs: Self) -> T {
        self.x * rhs.x
    }
}

//============================================================
// from
//============================================================
impl<T: VectorElement> From<[T; 1]> for Vector1<T> {
    fn from(v: [T; 1]) -> Vector1<T> {
        Vector1::<T> { x: v[0] }
    }
}

impl<T: VectorElement> From<T> for Vector1<T> {
    fn from(v: T) -> Vector1<T> {
        Vector1::<T> { x: v }
    }
}

//============================================================
// fmt
//============================================================
impl<T: std::fmt::Display + VectorElement> std::fmt::Display for Vector1<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", self.x)
    }
}

#[cfg(test)]
mod test {
    use super::Vector1;

    mod ops {
        use super::Vector1;
        use proptest::*;
        ops_test!(Vector1; x);
    }
}
