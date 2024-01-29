pub use super::traits::{VectorElement, VectorLike};
use num::Float;
use pyr_math_derive::Vector;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A 3-dimensional vector.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Vector, Serialize, Deserialize)]
pub struct Vector3<T: VectorElement> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: VectorElement> Vector3<T> {
    /// Create a new vector.
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Create a new vector from a 1-dimensional vector.
    /// ```
    /// use pyr::math::vector::{Vector1, Vector3};
    /// let v1 = Vector1::new(1);
    /// let v3 = Vector3::from_v1_with_yz(v1, 2, 3);
    /// assert_eq!(v3, Vector3::new(1, 2, 3));
    /// ```
    pub fn from_v1_with_yz<V>(v: V, y: T, z: T) -> Self
    where
        V: VectorLike<T, 1>,
    {
        Self { x: v.get(0), y, z }
    }

    /// Create a new vector from a 2-dimensional vector.
    /// ```
    /// use pyr::math::vector::{Vector2, Vector3};
    /// let v2 = Vector2::new(1, 2);
    /// let v3 = Vector3::from_v2_with_z(v2, 3);
    /// assert_eq!(v3, Vector3::new(1, 2, 3));
    /// ```
    pub fn from_v2_with_z<V>(v: V, z: T) -> Self
    where
        V: VectorLike<T, 2>,
    {
        Self {
            x: v.get(0),
            y: v.get(1),
            z,
        }
    }
}

impl<T: VectorElement> Vector3<T> {
    /// ```
    /// let zero = pyr::math::Vector3::<i32>::ZERO;
    /// assert_eq!((zero.x, zero.y, zero.z), (0, 0, 0));
    /// ```
    pub const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO);

    /// ```
    /// let one = pyr::math::Vector3::<i32>::ONE;
    /// assert_eq!((one.x, one.y, one.z), (1, 1, 1));
    /// ```
    pub const ONE: Self = Self::new(T::ONE, T::ONE, T::ONE);

    /// A unit vector pointing along the positive X axis.
    /// ```
    /// let unit_x = pyr::math::Vector3::<i32>::UNIT_X;
    /// assert_eq!((unit_x.x, unit_x.y, unit_x.z), (1, 0, 0));
    /// ```
    pub const UNIT_X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);

    /// A unit vector pointing along the positive Y axis.
    /// ```
    /// let unit_y = pyr::math::Vector3::<i32>::UNIT_Y;
    /// assert_eq!((unit_y.x, unit_y.y, unit_y.z), (0, 1, 0));
    /// ```
    pub const UNIT_Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);

    /// A unit vector pointing along the positive Z axis.
    /// ```
    /// let unit_z = pyr::math::Vector3::<i32>::UNIT_Z;
    /// assert_eq!((unit_z.x, unit_z.y, unit_z.z), (0, 0, 1));
    /// ```
    pub const UNIT_Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
}

impl<T: VectorElement> VectorLike<T, 3> for Vector3<T> {
    fn get(&self, index: usize) -> T {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("out of range"),
        }
    }

    fn set(&mut self, index: usize, value: T) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => panic!("out of range"),
        }
    }
}

impl<T: VectorElement + fmt::Display> fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl<T: VectorElement + std::ops::Neg<Output = T>> Vector3<T> {
    /// Get the cross product of two vectors.
    pub fn cross(&self, rhs: impl VectorLike<T, 3>) -> Self {
        Self {
            x: self.get(1) * rhs.get(2) - self.get(2) * rhs.get(1),
            y: -(self.get(0) * rhs.get(2) - self.get(2) * rhs.get(0)),
            z: self.get(0) * rhs.get(1) - self.get(1) * rhs.get(0),
        }
    }
}

impl<T: VectorElement + Float> Vector3<T> {
    /// Get the angle between two vectors.
    /// Returns the angle in radians.
    pub fn angle(&self, rhs: Vector3<T>) -> T {
        let dot = self.dot(rhs);
        let len = self.length() * rhs.length();
        (dot / len).acos()
    }

    /// Get the angle between two vectors.
    /// Returns the angle in radians.
    /// The sign of the angle is determined by the sign of the cross product.
    pub fn signed_angle(&self, rhs: Vector3<T>, normal: impl VectorLike<T, 3>) -> T {
        let angle = self.angle(rhs);
        let cross = self.cross(rhs);
        if cross.dot(normal) < T::zero() {
            -angle
        } else {
            angle
        }
    }

    pub fn reflect(&self, normal: Vector3<T>) -> Self {
        *self - normal * self.dot(normal) * T::from(2.0).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    fn gen_vec3<T: VectorElement + Arbitrary>() -> impl Strategy<Value = Vector3<T>> {
        (any::<T>(), any::<T>(), any::<T>()).prop_map(|(x, y, z)| Vector3::<T>::new(x, y, z))
    }

    fn gen_non_zero_normal_f32_vec3() -> impl Strategy<Value = Vector3<f32>> {
        // 要素が-1000.0から1000.0の間のベクトルを生成する
        (
            -1000.0..1000.0_f32,
            -1000.0..1000.0_f32,
            -1000.0..1000.0_f32,
        )
            .prop_map(|(x, y, z)| Vector3::<f32>::new(x, y, z))
            .prop_filter("non zero", |v| v.length_squared() > 0.0)
    }

    fn convert<T, U>(v: Vector3<T>) -> Vector3<U>
    where
        T: VectorElement + num::cast::AsPrimitive<U>,
        U: VectorElement + 'static,
    {
        Vector3::<U> {
            x: v.x.as_(),
            y: v.y.as_(),
            z: v.z.as_(),
        }
    }

    proptest! {
        #[test]
        fn new(x in any::<i32>(), y in any::<i32>(), z in any::<i32>()) {
            let v = Vector3::new(x, y, z);
            assert_eq!(v.x, x);
            assert_eq!(v.y, y);
            assert_eq!(v.z, z);
        }

        #[test]
        fn vector_like_get(v in gen_vec3::<i32>()) {
            assert_eq!((v.get(0), v.get(1), v.get(2)), (v.x, v.y, v.z));
        }

        #[test]
        fn fmt(v in gen_vec3::<i32>()) {
            let s = format!("{}", v);
            assert_eq!(s, format!("[{}, {}, {}]", v.x, v.y, v.z));
        }

        #[test]
        fn cross(v1 in gen_vec3::<i32>(), v2 in gen_vec3()) {
        {
            let v1 = Vector3::new(1, 0, 0_i64); // x axis
            let v2 = Vector3::new(0, 1, 0); // y axis
            let v3 = v1.cross(v2);
            assert_eq!(v3, Vector3::new(0, 0, 1))
        }
        {
            let v1 = Vector3::new(0, 1, 0); // y axis
            let v2 = Vector3::new(0, 0, 1_i64); // z axis
            let v3 = v1.cross(v2);
            assert_eq!(v3, Vector3::new(1, 0, 0))
        }
        {
            let v1 = Vector3::new(0, 0, 1); // z axis
            let v2 = Vector3::new(1, 0, 0_i64); // z axis
            let v3 = v1.cross(v2);
            assert_eq!(v3, Vector3::new(0, 1, 0))
        }
            let v1 = convert::<i32, i64>(v1);
            let v2 = convert::<i32, i64>(v2);
            let c1 = v1.cross(v2);
            let c2 = v2.cross(v1);
            assert_eq!(c1, -c2)
        }

        #[test]
        fn deserialize_i32(x in any::<i32>(), y in any::<i32>(), z in any::<i32>()) {
            let s = format!("{{ \"x\": {}, \"y\": {}, \"z\": {} }}", x, y, z);
            let v: Vector3::<i32> = serde_json::from_str(&s).unwrap();
            assert_eq!(v, Vector3::new(x, y, z));
        }

        #[test]
        fn deserialize_f32(x in any::<f32>(), y in any::<f32>(), z in any::<f32>()) {
            let s = format!("{{ \"x\": {}, \"y\": {}, \"z\": {} }}", x, y, z);
            let v: Vector3::<f32> = serde_json::from_str(&s).unwrap();
            assert_eq!(v, Vector3::new(x, y, z));
        }

        #[test]
        fn serialize(v in gen_vec3::<i32>()) {
            let json1 = serde_json::to_value(&v).unwrap();
            let json2 = serde_json::json!({
                "x": v.x,
                "y": v.y,
                "z": v.z,
            });
            assert_eq!(json1, json2);
        }

        #[test]
        fn angle(v1 in gen_non_zero_normal_f32_vec3(), v2 in gen_non_zero_normal_f32_vec3()) {
            let angle = v1.angle(v2);
            assert!(0.0 <= angle && angle <= std::f32::consts::PI);
        }

        #[test]
        fn signed_angle(v1 in gen_non_zero_normal_f32_vec3(), v2 in gen_non_zero_normal_f32_vec3()) {
            let angle = v1.angle(v2);
            assert!(-std::f32::consts::PI <= angle && angle <= std::f32::consts::PI);
        }
    }
}
