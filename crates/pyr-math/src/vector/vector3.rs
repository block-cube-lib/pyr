use crate::{FloatScalar, Lerp, Scalar};
use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl_vec_ops!(Vec3, x, y, z);
impl_vec_scalar_ops!(Vec3, x, y, z);
impl_vec_index_ops!(Vec3, x, 0, y, 1, z, 2);
impl_vec_neg!(Vec3, x, y, z);
impl_primitive_scalar_vec_mul!(Vec3, x, y, z);
impl_vec_approx!(Vec3, x, y, z);
impl_vec_float_math!(Vec3);
impl_vec_cast!(Vec3, x, y, z);

impl<T> Vec3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn truncate(self) -> crate::vector::Vec2<T> {
        crate::vector::Vec2::new(self.x, self.y)
    }

    #[inline]
    pub fn from_array(a: [T; 3]) -> Self {
        let (x, y, z) = a.into();
        Self::new(x, y, z)
    }

    #[inline]
    pub fn to_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const Vec3<T> as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut Vec3<T> as *mut T
    }
}

impl<T: Scalar> Vec3<T> {
    pub const ONE: Self = Self::new(T::ONE, T::ONE, T::ONE);
    pub const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO);
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
}

macro_rules! impl_vec3_neg_consts {
    ($($t:ty),+) => {
        $(
            impl Vec3<$t> {
                pub const NEG_X: Self = Self { x: -1 as $t, y: 0 as $t, z: 0 as $t };
                pub const NEG_Y: Self = Self { x: 0 as $t, y: -1 as $t, z: 0 as $t };
                pub const NEG_Z: Self = Self { x: 0 as $t, y: 0 as $t, z: -1 as $t };
            }
        )+
    };
}
impl_vec3_neg_consts!(i8, i16, i32, i64, f32, f64, isize);

impl<T: Scalar> Vec3<T> {
    #[inline]
    pub fn length_squared(self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T: FloatScalar> Vec3<T> {
    #[inline]
    pub fn angle_between(self, other: Self) -> T {
        let dot = self.dot(other);
        let len_sq_product = self.length_squared() * other.length_squared();
        if len_sq_product.approx_eq(&T::ZERO) {
            T::ZERO
        } else {
            let cos_theta = (dot / len_sq_product.sqrt()).clamp(-T::ONE, T::ONE);
            cos_theta.acos()
        }
    }

    #[inline]
    pub fn signed_angle(self, other: Self, normal: Self) -> T {
        let angle = self.angle_between(other);
        let cross = self.cross(other);
        if cross.dot(normal) < T::ZERO {
            -angle
        } else {
            angle
        }
    }
}

impl<T: FloatScalar> Lerp for Vec3<T> {
    type Factor = T;

    #[inline]
    fn lerp(self, rhs: Self, t: T) -> Self {
        self * (T::ONE - t) + rhs * t
    }
}

impl<T> std::convert::From<(T, T, T)> for Vec3<T> {
    #[inline]
    fn from(tuple: (T, T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl<T> std::convert::From<(crate::vector::Vec2<T>, T)> for Vec3<T> {
    #[inline]
    fn from(tuple: (crate::vector::Vec2<T>, T)) -> Self {
        Self {
            x: tuple.0.x,
            y: tuple.0.y,
            z: tuple.1,
        }
    }
}

impl<T> std::convert::From<[T; 3]> for Vec3<T> {
    #[inline]
    fn from(array: [T; 3]) -> Self {
        Self::from_array(array)
    }
}

impl<T: Scalar> AsRef<[T; 3]> for Vec3<T> {
    #[inline]
    fn as_ref(&self) -> &[T; 3] {
        unsafe { &*(self as *const Vec3<T> as *const [T; 3]) }
    }
}

impl<T: Scalar> AsMut<[T; 3]> for Vec3<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; 3] {
        unsafe { &mut *(self as *mut Vec3<T> as *mut [T; 3]) }
    }
}

impl<T> std::convert::From<crate::vector::Vec4<T>> for Vec3<T> {
    #[inline]
    fn from(v: crate::vector::Vec4<T>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}
