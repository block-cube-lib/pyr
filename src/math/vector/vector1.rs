use crate::math::vector::array_wrapper::ArrayWrapper;
use crate::math::vector::{VectorElement, VectorLike};
use num::{One, Zero};
use pyr_math_derive::Vector;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Vector, Serialize, Deserialize)]
pub struct Vector1<T: VectorElement> {
    pub x: T,
}

impl<T: VectorElement> Vector1<T> {
    pub const fn new(x: T) -> Self {
        Self { x }
    }
}

impl<T: VectorElement> Vector1<T> {
    pub const ZERO: Self = Self::new(T::ZERO);
    pub const ONE: Self = Self::new(T::ONE);
    pub const UNIT_X: Self = Self::new(T::ONE);
}

impl<T: VectorElement> VectorLike<T, 1> for Vector1<T> {
    fn get(&self, index: usize) -> T {
        match index {
            0 => self.x,
            _ => panic!("out of range"),
        }
    }

    fn set(&mut self, index: usize, value: T) {
        match index {
            0 => self.x = value,
            _ => panic!("out of range"),
        }
    }
}

impl<T: VectorElement> std::convert::From<ArrayWrapper<T, 1>> for Vector1<T> {
    fn from(value: ArrayWrapper<T, 1>) -> Self {
        Self {
            x: value.elements[0],
        }
    }
}

impl<T: VectorElement> std::convert::From<(T,)> for Vector1<T> {
    fn from(value: (T,)) -> Self {
        Self { x: value.0 }
    }
}

impl<T: VectorElement> std::convert::From<[T; 1]> for Vector1<T> {
    fn from(value: [T; 1]) -> Self {
        Self { x: value[0] }
    }
}

impl<T: VectorElement> Zero for Vector1<T> {
    fn zero() -> Self {
        Self { x: T::zero() }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl<T: VectorElement> One for Vector1<T> {
    fn one() -> Self {
        Self { x: T::one() }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn new(x in any::<i32>()) {
            let v = Vector1::new(x);
            assert_eq!(v.x, x);
        }

        #[test]
        fn vector_like_get(x in any::<i32>()) {
            let v = Vector1::new(x);
            assert_eq!(v.get(0), x);
        }
    }

    #[test]
    #[should_panic]
    fn vector_like_get_out_of_range() {
        let v = Vector1::new(0);
        let _ = v.get(1);
    }

    proptest! {
        #[test]
        fn vector_like_set(x in any::<i32>()) {
            let mut v = Vector1::new(0);
            v.set(0, x);
            assert_eq!(v.x, x);
        }
    }

    #[test]
    #[should_panic]
    fn vector_like_set_out_of_range() {
        let mut v = Vector1::new(1);
        v.set(1, 1);
    }

    proptest! {
        #[test]
        fn from_tuple(x in any::<i32>()) {
            let v = Vector1::from((x,));
            assert_eq!(v.x, x);
        }

        #[test]
        fn from_array(x in any::<i32>()) {
            let v = Vector1::from([x]);
            assert_eq!(v.x, x);
        }

        #[test]
        fn from_array_wrapper(x in any::<i32>()) {
            let v = Vector1::from(ArrayWrapper::<i32, 1>::new(x));
            assert_eq!(v.x, x);
        }

        #[test]
        fn length_squared(x in any::<i32>()) {
            let x = x as i64;
            let v = Vector1::new(x);
            let ls = v.length_squared();
            assert_eq!(ls, x * x);
        }

        #[test]
        fn length(x in any::<f32>()) {
            let v = Vector1::new(x);
            let l = v.length();
            assert_ne!((x - l).abs(), 0.000001);
        }

        #[test]
        fn dot(x1 in any::<i32>(), x2 in any::<i32>()) {
            let (x1, x2) = (x1 as i64, x2 as i64);
            let v = Vector1::new(x1);
            let d = v.dot([x2]);
            assert_eq!(d, x1 * x2);
        }
    }
}
