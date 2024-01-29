pub use super::traits::{VectorElement, VectorLike};
use pyr_math_derive::Vector;
use serde::{Deserialize, Serialize};

/// 4-dimensional vector.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vector, Serialize, Deserialize)]
pub struct Vector4<T: VectorElement> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: VectorElement> Vector4<T> {
    ///  Creates a 4-dimensional vector.
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: VectorElement> Vector4<T> {
    /// ```
    /// let zero = pyr::math::Vector4::<i32>::ZERO;
    /// assert_eq!((zero.x, zero.y, zero.z, zero.w), (0, 0, 0, 0));
    /// ```
    pub const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);

    /// ```
    /// let one = pyr::math::Vector4::<i32>::ONE;
    /// assert_eq!((one.x, one.y, one.z, one.w), (1, 1, 1, 1));
    /// ```
    pub const ONE: Self = Self::new(T::ONE, T::ONE, T::ONE, T::ONE);

    /// A unit vector pointing along the positive X axis.
    /// ```
    /// let unit_x = pyr::math::Vector4::<i32>::UNIT_X;
    /// assert_eq!((unit_x.x, unit_x.y, unit_x.z, unit_x.w), (1, 0, 0, 0));
    /// ```
    pub const UNIT_X: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);

    /// A unit vector pointing along the positive Y axis.
    /// ```
    /// let unit_y = pyr::math::Vector4::<i32>::UNIT_Y;
    /// assert_eq!((unit_y.x, unit_y.y, unit_y.z, unit_y.w), (0, 1, 0, 0));
    /// ```
    pub const UNIT_Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);

    /// A unit vector pointing along the positive Z axis.
    /// ```
    /// let unit_z = pyr::math::Vector4::<i32>::UNIT_Z;
    /// assert_eq!((unit_z.x, unit_z.y, unit_z.z, unit_z.w), (0, 0, 1, 0));
    /// ```
    pub const UNIT_Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);

    /// A unit vector pointing along the positive W axis.
    /// ```
    /// let unit_w = pyr::math::Vector4::<i32>::UNIT_W;
    /// assert_eq!((unit_w.x, unit_w.y, unit_w.z, unit_w.w), (0, 0, 0, 1));
    /// ```
    pub const UNIT_W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

impl<T: VectorElement> VectorLike<T, 4> for Vector4<T> {
    fn get(&self, index: usize) -> T {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!("out of range"),
        }
    }

    fn set(&mut self, index: usize, value: T) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            3 => self.w = value,
            _ => panic!("out of range"),
        }
    }
}

impl<T: VectorElement> Vector4<T> {
    /// Create a new vector from a 1-dimensional vector.
    /// ```
    /// use pyr::math::vector::{Vector1, Vector4};
    /// let v1 = Vector1::new(1);
    /// let v4 = Vector4::from_v1_with_yzw(v1, 2, 3, 4);
    /// assert_eq!(v4, Vector4::new(1, 2, 3, 4));
    /// ```
    pub fn from_v1_with_yzw<V: VectorLike<T, 1>>(v: V, y: T, z: T, w: T) -> Self {
        Self {
            x: v.get(0),
            y,
            z,
            w,
        }
    }

    /// Create a new vector from a 2-dimensional vector.
    /// ```
    /// use pyr::math::vector::{Vector2, Vector4};
    /// let v2 = Vector2::new(1, 2);
    /// let v4 = Vector4::from_v2_with_zw(v2, 3, 4);
    /// assert_eq!(v4, Vector4::new(1, 2, 3, 4));
    /// ```
    pub fn from_v2_with_zw<V: VectorLike<T, 2>>(v: V, z: T, w: T) -> Self {
        Self {
            x: v.get(0),
            y: v.get(1),
            z,
            w,
        }
    }

    /// Create a new vector from a 3-dimensional vector.
    /// ```
    /// use pyr::math::vector::{Vector3, Vector4};
    /// let v3 = Vector3::new(1, 2, 3);
    /// let v4 = Vector4::from_v3_with_w(v3, 4);
    /// assert_eq!(v4, Vector4::new(1, 2, 3, 4));
    /// ```
    pub fn from_v3_with_w<V: VectorLike<T, 3>>(v: V, w: T) -> Self {
        Self {
            x: v.get(0),
            y: v.get(1),
            z: v.get(2),
            w,
        }
    }
}
