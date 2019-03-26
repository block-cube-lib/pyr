use super::prelude::*;
use super::{Vector, VectorElement};
use num::{pow, One, Zero};
use std;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector4<T: VectorElement> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: VectorElement> Vector for Vector4<T> {
    type ElementType = T;
    const DIMENSION: usize = 4;
}

impl<T: VectorElement> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Vector4 { x, y, z, w }
    }
}

//============================================================
// operator
//============================================================
impl_ops!(Vector4; x, y, z, w);

//============================================================
// impl num
//============================================================
/// Make a zero vector.
/// Same to Vector4 { x: 0, y: 0, z: 0 , w: 0}.
impl<T: VectorElement> Zero for Vector4<T> {
    fn zero() -> Self {
        Self::from_scalar(T::zero())
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

/// Make a Vector4 { x: 1, y: 1, z: 1, w: 0 }.
impl<T: VectorElement> One for Vector4<T> {
    fn one() -> Self {
        Self::from_scalar(T::one())
    }
}

//============================================================
// construct traits
//============================================================
impl<T: VectorElement> FromScalar for Vector4<T> {
    fn from_scalar(scalar: Self::ElementType) -> Self {
        Vector4 {
            x: scalar,
            y: scalar,
            z: scalar,
            w: scalar,
        }
    }
}

//============================================================
// function traits
//============================================================
impl<T: VectorElement> LengthSquared for Vector4<T> {
    fn length_squared(&self) -> Self::ElementType {
        pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2) + pow(self.w, 2)
    }
}

impl<T: VectorElement> Dot for Vector4<T> {
    fn dot(&self, rhs: Self) -> Self::ElementType {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

#[cfg(test)]
mod test {
    use super::super::prelude::*;
    use super::Vector4;
    use num::pow;

    mod ops {
        use super::Vector4;
        use proptest::*;
        ops_test!(Vector4; x, y, z, w);
    }

    #[test]
    fn length_squared() {
        use proptest::*;
        proptest! (|(x in -100..100, y in -100..100, z in -100..100, w in -100..100)| {
            let v = Vector4 { x, y, z, w };
            let ls = pow(x, 2) + pow(y, 2) + pow(z, 2) + pow(w, 2);
            assert_eq!(v.length_squared(), ls);
        });
    }
}
