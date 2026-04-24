use crate::{FloatScalar, Lerp, Scalar};
use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl_vec_ops!(Vec2, x, y);
impl_vec_scalar_ops!(Vec2, x, y);
impl_vec_index_ops!(Vec2, x, 0, y, 1);
impl_vec_neg!(Vec2, x, y);
impl_primitive_scalar_vec_mul!(Vec2, x, y);
impl_vec_approx!(Vec2, x, y);
impl_vec_cast!(Vec2, x, y);
impl_vec_float_math!(Vec2);

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn from_array(a: [T; 2]) -> Self {
        let (x, y) = a.into();
        Self::new(x, y)
    }

    #[inline]
    pub fn to_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const Vec2<T> as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut Vec2<T> as *mut T
    }

    #[inline]
    pub fn extend(self, z: T) -> crate::vector::Vec3<T> {
        crate::vector::Vec3::new(self.x, self.y, z)
    }
}

impl<T: Scalar> Vec2<T> {
    pub const ONE: Self = Self::new(T::ONE, T::ONE);
    pub const ZERO: Self = Self::new(T::ZERO, T::ZERO);
    pub const X: Self = Self::new(T::ONE, T::ZERO);
    pub const Y: Self = Self::new(T::ZERO, T::ONE);
}

macro_rules! impl_vec2_neg_consts {
    ($($t:ty),+) => {
        $(
            impl Vec2<$t> {
                pub const NEG_X: Self = Self { x: -1 as $t, y: 0 as $t };
                pub const NEG_Y: Self = Self { x: 0 as $t, y: -1 as $t };
            }
        )+
    };
}
impl_vec2_neg_consts!(i8, i16, i32, i64, f32, f64, isize);

impl<T: Scalar> Vec2<T> {
    #[inline]
    pub fn length_squared(self) -> T {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl<T> Vec2<T>
where
    T: Scalar + Neg<Output = T>,
{
    /// Returns a vector that is perpendicular to the original vector, rotated 90 degrees
    /// counterclockwise.
    #[inline]
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

impl<T: FloatScalar> Vec2<T> {
    /// Returns the angle between two vectors in radians, in the range [0, π].
    #[inline]
    pub fn angle_between(self, other: Self) -> T {
        self.signed_angle(other).abs()
    }

    /// / Returns the signed angle from `self` to `other` in radians, in the range [-π, π].
    #[inline]
    pub fn signed_angle(self, other: Self) -> T {
        self.cross(other).atan2(self.dot(other))
    }

    #[inline]
    pub fn rotate(self, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    #[inline]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self { x: cos, y: sin }
    }
}

impl<T: FloatScalar> Lerp for Vec2<T> {
    type Factor = T;

    #[inline]
    fn lerp(self, rhs: Self, t: T) -> Self {
        self * (T::ONE - t) + rhs * t
    }
}

impl<T> std::convert::From<(T, T)> for Vec2<T> {
    #[inline]
    fn from(tuple: (T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> std::convert::From<[T; 2]> for Vec2<T> {
    #[inline]
    fn from(array: [T; 2]) -> Self {
        Self::from_array(array)
    }
}

impl<T: Scalar> AsRef<[T; 2]> for Vec2<T> {
    #[inline]
    fn as_ref(&self) -> &[T; 2] {
        unsafe { &*(self as *const Vec2<T> as *const [T; 2]) }
    }
}

impl<T: Scalar> AsMut<[T; 2]> for Vec2<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; 2] {
        unsafe { &mut *(self as *mut Vec2<T> as *mut [T; 2]) }
    }
}

impl<T> std::convert::From<crate::vector::Vec3<T>> for Vec2<T> {
    #[inline]
    fn from(v: crate::vector::Vec3<T>) -> Self {
        Self::new(v.x, v.y)
    }
}
