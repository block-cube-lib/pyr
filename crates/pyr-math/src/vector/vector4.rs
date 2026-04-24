use crate::{FloatScalar, Lerp, Scalar};
use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl_vec_ops!(Vec4, x, y, z, w);
impl_vec_scalar_ops!(Vec4, x, y, z, w);
impl_vec_index_ops!(Vec4, x, 0, y, 1, z, 2, w, 3);
impl_vec_neg!(Vec4, x, y, z, w);
impl_primitive_scalar_vec_mul!(Vec4, x, y, z, w);
impl_vec_approx!(Vec4, x, y, z, w);
impl_vec_float_math!(Vec4);
impl_vec_cast!(Vec4, x, y, z, w);

impl<T> Vec4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub fn truncate(self) -> crate::vector::Vec3<T> {
        crate::vector::Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    pub fn from_array(a: [T; 4]) -> Self {
        let (x, y, z, w) = a.into();
        Self::new(x, y, z, w)
    }

    #[inline]
    pub fn to_array(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const Vec4<T> as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut Vec4<T> as *mut T
    }
}

impl<T: Scalar> Vec4<T> {
    pub const ONE: Self = Self::new(T::ONE, T::ONE, T::ONE, T::ONE);
    pub const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
    pub const X: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);
    pub const Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);
    pub const Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    pub const W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

macro_rules! impl_vec4_neg_consts {
    ($($t:ty),+) => {
        $(
            impl Vec4<$t> {
                pub const NEG_X: Self = Self { x: -1 as $t, y: 0 as $t, z: 0 as $t, w:  0 as $t };
                pub const NEG_Y: Self = Self { x: 0 as $t, y: -1 as $t, z: 0 as $t, w:  0 as $t };
                pub const NEG_Z: Self = Self { x: 0 as $t, y: 0 as $t, z: -1 as $t, w:  0 as $t };
                pub const NEG_W: Self = Self { x: 0 as $t, y: 0 as $t, z:  0 as $t, w: -1 as $t };
            }
        )+
    };
}
impl_vec4_neg_consts!(i8, i16, i32, i64, f32, f64, isize);

impl<T: Scalar> Vec4<T> {
    #[inline]
    pub fn length_squared(self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    #[inline]
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl<T: FloatScalar> Vec4<T> {
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
}

impl<T: FloatScalar> Lerp for Vec4<T> {
    type Factor = T;

    #[inline]
    fn lerp(self, rhs: Self, t: T) -> Self {
        self * (T::ONE - t) + rhs * t
    }
}

impl<T> std::convert::From<(T, T, T, T)> for Vec4<T> {
    #[inline]
    fn from(tuple: (T, T, T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
            w: tuple.3,
        }
    }
}

impl<T> std::convert::From<(crate::vector::Vec2<T>, T, T)> for Vec4<T> {
    #[inline]
    fn from(tuple: (crate::vector::Vec2<T>, T, T)) -> Self {
        Self {
            x: tuple.0.x,
            y: tuple.0.y,
            z: tuple.1,
            w: tuple.2,
        }
    }
}

impl<T> std::convert::From<(crate::vector::Vec3<T>, T)> for Vec4<T> {
    #[inline]
    fn from(tuple: (crate::vector::Vec3<T>, T)) -> Self {
        Self {
            x: tuple.0.x,
            y: tuple.0.y,
            z: tuple.0.z,
            w: tuple.1,
        }
    }
}

impl<T> std::convert::From<[T; 4]> for Vec4<T> {
    #[inline]
    fn from(array: [T; 4]) -> Self {
        Self::from_array(array)
    }
}

impl<T: Scalar> AsRef<[T; 4]> for Vec4<T> {
    #[inline]
    fn as_ref(&self) -> &[T; 4] {
        unsafe { &*(self as *const Vec4<T> as *const [T; 4]) }
    }
}

impl<T: Scalar> AsMut<[T; 4]> for Vec4<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; 4] {
        unsafe { &mut *(self as *mut Vec4<T> as *mut [T; 4]) }
    }
}
