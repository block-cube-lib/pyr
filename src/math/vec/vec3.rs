use super::traits::{FloatVectorElement, VectorElement, VectorLike};
use crate::num::AsFloatingPoint;
use pyr_math_derive::Vector;
use serde::{Deserialize, Serialize};

/// A 3-dimensional vector.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Vector, Serialize, Deserialize)]
pub struct Vec3<T: VectorElement> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: VectorElement> Vec3<T> {
    /// Create a new vector.
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Create a new vector from a 1-dimensional vector.
    /// ```
    /// use pyr::math::vec::{Vec1, Vec3};
    /// let v1 = Vec1::new(1);
    /// let v3 = Vec3::from_v1_with_yz(v1, 2, 3);
    /// assert_eq!(v3, Vec3::new(1, 2, 3));
    /// ```
    pub fn from_v1_with_yz<V>(v: V, y: T, z: T) -> Self
    where
        V: VectorLike<1, ElementType = T>,
    {
        Self { x: *v.get(0), y, z }
    }

    /// Create a new vector from a 2-dimensional vector.
    /// ```
    /// use pyr::math::vec::{Vec2, Vec3};
    /// let v2 = Vec2::new(1, 2);
    /// let v3 = Vec3::from_v2_with_z(v2, 3);
    /// assert_eq!(v3, Vec3::new(1, 2, 3));
    /// ```
    pub fn from_v2_with_z<V>(v: V, z: T) -> Self
    where
        V: VectorLike<2, ElementType = T>,
    {
        Self {
            x: *v.get(0),
            y: *v.get(1),
            z,
        }
    }
}

impl<T: VectorElement> Vec3<T> {
    /// A unit vector pointing along the positive X axis.
    /// ```
    /// let unit_x = pyr::math::Vec3::<i32>::UNIT_X;
    /// assert_eq!((unit_x.x, unit_x.y, unit_x.z), (1, 0, 0));
    /// ```
    pub const UNIT_X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);

    /// A unit vector pointing along the positive Y axis.
    /// ```
    /// let unit_y = pyr::math::Vec3::<i32>::UNIT_Y;
    /// assert_eq!((unit_y.x, unit_y.y, unit_y.z), (0, 1, 0));
    /// ```
    pub const UNIT_Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);

    /// A unit vector pointing along the positive Z axis.
    /// ```
    /// let unit_z = pyr::math::Vec3::<i32>::UNIT_Z;
    /// assert_eq!((unit_z.x, unit_z.y, unit_z.z), (0, 0, 1));
    /// ```
    pub const UNIT_Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
}

impl<T: VectorElement> VectorLike<3> for Vec3<T> {
    type ElementType = T;

    fn get(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of range"),
        }
    }

    fn get_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("out of range"),
        }
    }
}

impl<T: VectorElement + std::ops::Neg<Output = T>> Vec3<T> {
    /// Get the cross product of two vectors.
    pub fn cross(&self, rhs: impl VectorLike<3, ElementType = T>) -> Self {
        #[cfg(feature = "right_handed_coordinates")]
        {
            Self {
                x: *self.get(1) * *rhs.get(2) - *self.get(2) * *rhs.get(1),
                y: *self.get(2) * *rhs.get(0) - *self.get(0) * *rhs.get(2),
                z: *self.get(0) * *rhs.get(1) - *self.get(1) * *rhs.get(0),
            }
        }
        #[cfg(feature = "left_handed_coordinates")]
        {
            Self {
                x: *self.get(2) * *rhs.get(1) - *self.get(1) * *rhs.get(2),
                y: *self.get(0) * *rhs.get(2) - *self.get(2) * *rhs.get(0),
                z: *self.get(1) * *rhs.get(0) - *self.get(0) * *rhs.get(1),
            }
        }
    }
}

impl<T: VectorElement> Vec3<T> {
    /// Get the angle between two vectors.
    /// Returns the angle in radians.
    pub fn angle(&self, rhs: impl VectorLike<3, ElementType = T>) -> <T as AsFloatingPoint>::Output
    where
        <T as AsFloatingPoint>::Output: FloatVectorElement,
    {
        use super::ops::vector_cast;
        use ::num::Float as _;
        let rhs: Self = vector_cast(rhs);
        let rhs = rhs.as_float_vec();
        let dot = self.as_float_vec().dot(rhs);
        let len = self.length() * rhs.length();
        (dot / len).acos()
    }

    /// Get the angle between two vectors.
    /// Returns the angle in radians.
    /// The sign of the angle is determined by the sign of the cross product.
    pub fn signed_angle(
        &self,
        rhs: impl VectorLike<3, ElementType = T>,
        normal: impl VectorLike<3, ElementType = T>,
    ) -> <T as AsFloatingPoint>::Output
    where
        <T as AsFloatingPoint>::Output: FloatVectorElement,
    {
        use crate::num::Zero as _;
        let rhs = Vec3::<T>::new(*rhs.get(0), *rhs.get(1), *rhs.get(2)).as_float_vec();
        let lhs = self.as_float_vec();
        let angle = lhs.angle(rhs);
        let cross = lhs.cross(rhs);
        let normal = Vec3::<T>::new(*normal.get(0), *normal.get(1), *normal.get(2)).as_float_vec();
        if cross.dot(normal) < <T as AsFloatingPoint>::Output::ZERO {
            -angle
        } else {
            angle
        }
    }

    pub fn reflect(&self, normal: Vec3<T>) -> Vec3<<T as AsFloatingPoint>::Output>
    where
        <T as AsFloatingPoint>::Output: FloatVectorElement,
    {
        use crate::num::One as _;
        let two: <T as AsFloatingPoint>::Output =
            <T as AsFloatingPoint>::Output::ONE + <T as AsFloatingPoint>::Output::ONE;
        let v = self.as_float_vec();
        let normal = normal.normalized();
        v - normal * v.dot(normal) * two
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    fn gen_vec3<T: VectorElement + Arbitrary>() -> impl Strategy<Value = Vec3<T>> {
        (any::<T>(), any::<T>(), any::<T>()).prop_map(|(x, y, z)| Vec3::<T>::new(x, y, z))
    }

    fn gen_non_zero_normal_f32_vec3() -> impl Strategy<Value = Vec3<f32>> {
        // 要素が-1000.0から1000.0の間のベクトルを生成する
        (
            -1000.0..1000.0_f32,
            -1000.0..1000.0_f32,
            -1000.0..1000.0_f32,
        )
            .prop_map(|(x, y, z)| Vec3::<f32>::new(x, y, z))
            .prop_filter("non zero", |v| v.length_squared() > 0.0)
    }

    fn convert<T, U>(v: Vec3<T>) -> Vec3<U>
    where
        T: VectorElement + num::cast::AsPrimitive<U>,
        U: VectorElement + 'static,
    {
        Vec3::<U> {
            x: v.x.as_(),
            y: v.y.as_(),
            z: v.z.as_(),
        }
    }

    proptest! {
        #[test]
        fn new(x in any::<i32>(), y in any::<i32>(), z in any::<i32>()) {
            let v = Vec3::new(x, y, z);
            assert_eq!(v.x, x);
            assert_eq!(v.y, y);
            assert_eq!(v.z, z);
        }

        #[test]
        fn vector_like_get(v in gen_vec3::<i32>()) {
            assert_eq!((*v.get(0), *v.get(1), *v.get(2)), (v.x, v.y, v.z));
        }

        #[test]
        fn fmt(v in gen_vec3::<i32>()) {
            let s = format!("{}", v);
            assert_eq!(s, format!("[{}, {}, {}]", v.x, v.y, v.z));
        }

        #[cfg(feature = "right_handed_coordinates")]
        #[test]
        fn cross(v1 in gen_vec3::<i32>(), v2 in gen_vec3()) {
        {
            let v1 = Vec3::new(1, 0, 0_i64); // x axis
            let v2 = Vec3::new(0, 1, 0); // y axis
            let v3 = v1.cross(v2);
            assert_eq!(v3, Vec3::new(0, 0, 1))
        }
        {
            let v1 = Vec3::new(0, 1, 0); // y axis
            let v2 = Vec3::new(0, 0, 1_i64); // z axis
            let v3 = v1.cross(v2);
            assert_eq!(v3, Vec3::new(1, 0, 0))
        }
        {
            let v1 = Vec3::new(0, 0, 1); // z axis
            let v2 = Vec3::new(1, 0, 0_i64); // z axis
            let v3 = v1.cross(v2);
            assert_eq!(v3, Vec3::new(0, 1, 0))
        }
            let v1 = convert::<i32, i64>(v1);
            let v2 = convert::<i32, i64>(v2);
            let c1 = v1.cross(v2);
            let c2 = v2.cross(v1);
            assert_eq!(c1, -c2)
        }

        #[cfg(feature = "left_handed_coordinates")]
        #[test]
        fn cross(v1 in gen_vec3::<i32>(), v2 in gen_vec3()) {
        {
            let v1 = Vec3::new(1, 0, 0_i64); // x axis
            let v2 = Vec3::new(0, 1, 0); // y axis
            let v3 = v1.cross(v2);
            assert_eq!(v3, Vec3::new(0, 0, -1))
        }
        {
            let v1 = Vec3::new(0, 1, 0); // y axis
            let v2 = Vec3::new(0, 0, 1_i64); // z axis
            let v3 = v1.cross(v2);
            assert_eq!(v3, Vec3::new(-1, 0, 0))
        }
        {
            let v1 = Vec3::new(0, 0, 1); // z axis
            let v2 = Vec3::new(1, 0, 0_i64); // z axis
            let v3 = v1.cross(v2);
            assert_eq!(v3, Vec3::new(0, -1, 0))
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
            let v: Vec3::<i32> = serde_json::from_str(&s).unwrap();
            assert_eq!(v, Vec3::new(x, y, z));
        }

        #[test]
        fn deserialize_f32(x in any::<f32>(), y in any::<f32>(), z in any::<f32>()) {
            let s = format!("{{ \"x\": {}, \"y\": {}, \"z\": {} }}", x, y, z);
            let v: Vec3::<f32> = serde_json::from_str(&s).unwrap();
            assert_eq!(v, Vec3::new(x, y, z));
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

    #[test]
    fn distance() {
        let v1 = Vec3::new(1.0_f64, 2.0, 3.0);
        let v2 = Vec3::new(4.0_f64, 6.0, 8.0);
        let f_distance = v1.distance(v2);
        assert_eq!(f_distance, (v1 - v2).length());
        let v1 = Vec3::new(1_u32, 2, 3);
        let v2 = Vec3::new(4_u32, 6, 8);
        assert_eq!(v1.distance(v2), f_distance);
    }

    #[test]
    fn as_float_vec() {
        let v = Vec3::new(1, 2, 3);
        let fv = v.as_float_vec();
        assert_eq!(fv.x, 1.0);
        assert_eq!(fv.y, 2.0);
        assert_eq!(fv.z, 3.0);
    }

    #[test]
    fn normalized() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let n = v.normalized();
        assert_eq!(n.length(), 1.0);
    }

    #[test]
    fn normalize() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v.normalize();
        assert_eq!(v.length(), 1.0);
    }
}
