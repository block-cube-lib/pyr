use crate::math::{FloatVectorElement, Vec3, VectorLike};
use serde::{Deserialize, Serialize};
use std::ops;

pub trait QuatElement: FloatVectorElement
where
    Self: std::ops::Mul<Vec3<Self>, Output = Vec3<Self>>,
{
}
impl<T> QuatElement for T where
    Self: FloatVectorElement + std::ops::Mul<Vec3<Self>, Output = Vec3<Self>>
{
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Quat<T: QuatElement> {
    pub w: T,
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: QuatElement> Quat<T> {
    pub const IDENTITY: Self = Self {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
        w: T::ONE,
    };

    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_vec3_with_w(v: impl VectorLike<3, ElementType = T>, w: T) -> Self {
        Self {
            w,
            x: *v.get(0),
            y: *v.get(1),
            z: *v.get(2),
        }
    }

    pub fn rotate_axis_angle(axis: Vec3<T>, radian: T) -> Self {
        let one_half: T = T::ONE / (T::ONE + T::ONE);
        let half_radian = radian * one_half;
        let sin_half_angle = half_radian.sin();
        let cos_half_angle = half_radian.cos();
        let n = axis.normalized();
        Self {
            w: cos_half_angle,
            x: n.x * sin_half_angle,
            y: n.y * sin_half_angle,
            z: n.z * sin_half_angle,
        }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn norm(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn normalized(&self) -> Self {
        let n = self.norm();
        if n == T::ZERO {
            return Self::default();
        }
        let inv_sqrt_n = T::ONE / n.sqrt();
        Self {
            w: self.w * inv_sqrt_n,
            x: self.x * inv_sqrt_n,
            y: self.y * inv_sqrt_n,
            z: self.z * inv_sqrt_n,
        }
    }
}

impl<T: QuatElement> ops::Index<usize> for Quat<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.w,
            1 => &self.x,
            2 => &self.y,
            3 => &self.z,
            _ => panic!("out of range"),
        }
    }
}

impl<T: QuatElement> ops::IndexMut<usize> for Quat<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("out of range"),
        }
    }
}

impl<T: QuatElement> ops::Mul for Quat<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.z * rhs.y - self.y * rhs.z,
            y: self.w * rhs.y + self.y * rhs.w + self.x * rhs.z - self.z * rhs.x,
            z: self.w * rhs.z + self.z * rhs.w + self.y * rhs.x - self.x * rhs.y,
        }
    }
}

impl<T: QuatElement> ops::Mul<Vec3<T>> for Quat<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        let q = self.normalized();
        let p = Quat::from_vec3_with_w(rhs, T::ZERO);
        let conjugate = q.conjugate();
        let v = q * p * conjugate;
        Vec3::new(v.x, v.y, v.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_near_eq {
        ($a:expr, $b:expr) => {
            for i in 0..3 {
                assert!(($a[i] - $b[i]).abs() < 0.00001, "{} != {}", $a, $b);
            }
        };
    }

    #[test]
    fn rotate_vec() {
        let q = Quat::<f64>::rotate_axis_angle(Vec3::UNIT_Y, std::f64::consts::FRAC_PI_2);
        let v = Vec3::new(1.0, 0.0, 0.0);
        let rotated_v = q * v;
        assert_near_eq!(rotated_v, Vec3::<f64>::UNIT_Z);
    }
}
