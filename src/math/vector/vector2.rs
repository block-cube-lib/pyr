pub use super::traits::{VectorElement, VectorLike};
pub use super::vector1::Vector1;
use num::{Float, One, Zero};
use pyr_math_derive::Vector;
use serde::{Deserialize, Serialize};
use std::fmt;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Vector, Serialize, Deserialize)]
pub struct Vector2<T: VectorElement> {
    pub x: T,
    pub y: T,
}

impl<T: VectorElement> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn unit_x() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
        }
    }

    pub fn unit_y() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
        }
    }
}

impl<T: VectorElement> Zero for Vector2<T> {
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl<T: VectorElement> One for Vector2<T> {
    fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
        }
    }
}

impl<T: VectorElement> VectorLike<T, 2> for Vector2<T> {
    fn get(&self, index: usize) -> T {
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("out of range"),
        }
    }

    fn set(&mut self, index: usize, value: T) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            _ => panic!("out of range"),
        }
    }
}

impl<T: VectorElement + fmt::Display> fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T: VectorElement> Vector2<T> {
    pub fn cross(&self, rhs: impl VectorLike<T, 2>) -> T {
        self.x * rhs.get(1) - self.y * rhs.get(0)
    }
}

impl<T: VectorElement + Float> Vector2<T> {
    pub fn angle(&self, rhs: Vector2<T>) -> T {
        let dot = self.dot(rhs);
        let len = self.length() * rhs.length();
        (dot / len).acos()
    }

    pub fn signed_angle(&self, rhs: Vector2<T>) -> T {
        let angle = self.angle(rhs);
        let cross = self.cross(rhs);
        if cross < T::zero() {
            -angle
        } else {
            angle
        }
    }

    pub fn reflect(&self, normal: Vector2<T>) -> Self {
        *self - normal * self.dot(normal) * T::from(2.0).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn new() {
        let v = Vector2::<i32>::new(1, 2);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
    }

    #[test]
    fn vector_element_get() {
        let v = Vector2::<i32> { x: 1, y: 2 };
        assert_eq!(v.get(0), 1);
        assert_eq!(v.get(1), 2);
    }

    #[test]
    fn add() {
        let v1 = Vector2::<i32> { x: 1, y: 2 };
        let v2 = Vector2::<i32> { x: 11, y: 23 };
        let v = v1 + v2;
        assert_eq!(v.x, 12);
        assert_eq!(v.y, 25);
    }

    #[test]
    fn sub() {
        let v1 = Vector2::<i32> { x: 10, y: 11 };
        let v2 = Vector2::<i32> { x: 2, y: 9 };
        let v = v1 - v2;
        assert_eq!(v.x, 8);
        assert_eq!(v.y, 2);
    }

    #[test]
    fn mul() {
        let v1 = Vector2::<i32> { x: 10, y: 11 };
        let v2 = Vector2::<i32> { x: 2, y: 9 };
        let v = v1 * v2;
        assert_eq!(v.x, 20);
        assert_eq!(v.y, 99);
    }

    #[test]
    fn div() {
        let v1 = Vector2::<i32> { x: 10, y: 12 };
        let v2 = Vector2::<i32> { x: 2, y: 3 };
        let v = v1 / v2;
        assert_eq!(v.x, 5);
        assert_eq!(v.y, 4);
    }

    #[test]
    fn add_assign() {
        let mut v1 = Vector2::<i32> { x: 1, y: 2 };
        let v2 = Vector2::<i32> { x: 11, y: 23 };
        v1 += v2;
        assert_eq!(v1.x, 12);
        assert_eq!(v1.y, 25);
    }

    #[test]
    fn sub_assign() {
        let mut v1 = Vector2::<i32> { x: 10, y: 11 };
        let v2 = Vector2::<i32> { x: 2, y: 9 };
        v1 -= v2;
        assert_eq!(v1.x, 8);
        assert_eq!(v1.y, 2);
    }

    #[test]
    fn mul_assign() {
        let mut v1 = Vector2::<i32> { x: 10, y: 11 };
        let v2 = Vector2::<i32> { x: 2, y: 9 };
        v1 *= v2;
        assert_eq!(v1.x, 20);
        assert_eq!(v1.y, 99);
    }

    #[test]
    fn div_assign() {
        let mut v1 = Vector2::<i32> { x: 10, y: 12 };
        let v2 = Vector2::<i32> { x: 2, y: 3 };
        v1 /= v2;
        assert_eq!(v1.x, 5);
        assert_eq!(v1.y, 4);
    }

    #[test]
    fn index() {
        let v = Vector2 { x: 10, y: 20 };
        assert_eq!(v[0], 10);
        assert_eq!(v[1], 20);
    }

    #[test]
    #[should_panic]
    fn index_out_of_range() {
        let v = Vector2 { x: 10, y: 20 };
        let _ = v[2];
    }

    #[test]
    fn index_mut() {
        let mut v = Vector2 { x: 10, y: 20 };
        v[0] = 60;
        v[1] = 70;
        assert_eq!(v[0], 60);
        assert_eq!(v[1], 70);
    }

    #[test]
    #[should_panic]
    fn index_mut_out_of_range() {
        let mut v = Vector2 { x: 10, y: 20 };
        v[2] = 21;
    }

    #[test]
    fn neg() {
        let v = Vector2 { x: 10, y: 20 };
        let v = -v;
        assert_eq!(v[0], -10);
        assert_eq!(v[1], -20);
    }

    #[test]
    fn length_squared() {
        let v = Vector2 { x: 10, y: 20 };
        assert_eq!(v.length_squared(), 10 * 10 + 20 * 20);
    }

    #[test]
    fn length() {
        let v = Vector2 { x: 10.0, y: 20.0 };
        assert_eq!(v.length(), (10.0_f64 * 10.0 + 20.0 * 20.0).sqrt());
    }

    #[test]
    fn dot() {
        let v = Vector2 { x: 10, y: 20 };
        assert_eq!(v.dot([2_i64, 3]), 10 * 2 + 20 * 3);
    }
}
