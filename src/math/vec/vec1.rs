use crate::math::traits::{VectorElement, VectorLike};
use pyr_math_derive::Vector;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Vector, PartialEq, Serialize, Deserialize)]
pub struct Vec1<T: VectorElement> {
    pub x: T,
}

impl<T: VectorElement> Vec1<T> {
    pub const fn new(x: T) -> Self {
        Self { x }
    }
}

impl<T: VectorElement> VectorLike<1> for Vec1<T> {
    type ElementType = T;

    fn get(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            _ => panic!("out of range"),
        }
    }

    fn get_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            _ => panic!("out of range"),
        }
    }
}

impl<T: VectorElement> Vec1<T> {
    pub const UNIT_X: Self = Self::new(T::ONE);
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn new(x in any::<i32>()) {
            let v = Vec1::new(x);
            assert_eq!(v.x, x);
        }

        #[test]
        fn vector_like_get(x in any::<i32>()) {
            let v = Vec1::new(x);
            assert_eq!(*v.get(0), x);
        }
    }

    #[test]
    #[should_panic]
    fn vector_like_get_out_of_range() {
        let v = Vec1::new(0);
        let _ = v.get(1);
    }

    proptest! {
        #[test]
        fn vector_like_set(x in any::<i32>()) {
            let mut v = Vec1::new(0);
            *v.get_mut(0) = x;
            assert_eq!(v.x, x);
        }
    }

    #[test]
    #[should_panic]
    fn vector_like_set_out_of_range() {
        let mut v = Vec1::new(1);
        *v.get_mut(1) = 1;
    }

    #[test]
    fn add() {
        let v1 = Vec1::new(1_i32);
        let v2 = Vec1::new(2_i32);
        let result = v1 + v2;
        assert_eq!(result.x, 3);
    }

    #[test]
    fn add_assign() {
        let mut v = Vec1::new(2_i32);
        v += Vec1::new(3_i32);
        assert_eq!(v.x, 5);
    }

    #[test]
    fn sub() {
        let v1 = Vec1::new(3_i32);
        let v2 = Vec1::new(1_i32);
        let result = v1 - v2;
        assert_eq!(result.x, 2);
    }

    #[test]
    fn sub_assign() {
        let mut v = Vec1::new(5_i32);
        v -= Vec1::new(3_i32);
        assert_eq!(v.x, 2);
    }

    #[test]
    fn mul() {
        let v1 = Vec1::new(2_i32);
        let v2 = Vec1::new(3_i32);
        let result = v1 * v2;
        assert_eq!(result.x, 6);
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec1::new(3_i32);
        v *= Vec1::new(4_i32);
        assert_eq!(v.x, 12);
    }

    #[test]
    fn div() {
        let v1 = Vec1::new(8_i32);
        let v2 = Vec1::new(4_i32);
        let result = v1 / v2;
        assert_eq!(result.x, 2);
    }

    #[test]
    fn div_assign() {
        let mut v = Vec1::new(10_i32);
        v /= Vec1::new(2_i32);
        assert_eq!(v.x, 5);
    }

    #[test]
    fn mul_scalar() {
        let v = Vec1::new(2_i32) * 3;
        assert_eq!(v.x, 6);
    }

    #[test]
    fn mul_assign_scalar() {
        let mut v = Vec1::new(2_i32);
        v *= 4;
        assert_eq!(v.x, 8);
    }

    #[test]
    fn div_scalar() {
        let v = Vec1::new(6_i32) / 3;
        assert_eq!(v.x, 2);
    }

    #[test]
    fn div_assign_scalar() {
        let mut v = Vec1::new(10_i32);
        v /= 2;
        assert_eq!(v.x, 5);
    }

    #[test]
    fn mul_scalar_vec() {
        let v = 2 * Vec1::new(3_i32);
        assert_eq!(v.x, 6);
    }

    #[test]
    fn index() {
        let v = Vec1::new(1_i32);
        assert_eq!(v[0], 1);
    }

    #[test]
    fn index_mut() {
        let mut v = Vec1::new(0_i32);
        v[0] = 10;
        assert_eq!(v.x, 10);
    }

    #[test]
    fn neg() {
        let v = Vec1::new(1_i32);
        assert_eq!((-v).x, -1);
    }

    #[test]
    fn as_float_vec() {
        let v = Vec1::new(1_i32);
        let fv = v.as_float_vec();
        assert_eq!(fv.x, 1.0);
    }

    #[test]
    fn length_squared() {
        assert_eq!(Vec1::new(2_i32).length_squared(), 4);
    }

    #[test]
    fn length() {
        assert_eq!(Vec1::new(2_i32).length(), 2.0);
    }

    #[test]
    fn dot() {
        let v = Vec1::new(2);
        let d = v.dot([3]);
        assert_eq!(d, 6);
    }

    #[test]
    fn from_array() {
        let v = Vec1::from([1]);
        assert_eq!(v.x, 1);
    }

    #[test]
    fn into_array() {
        let v = Vec1::new(1_i32);
        let a: [i32; 1] = v.into();
        assert_eq!(a, [1]);
    }

    #[test]
    fn from_tuple() {
        let v = Vec1::from((1,));
        assert_eq!(v.x, 1);
    }

    #[test]
    fn into_tuple() {
        let v = Vec1::new(1_i32);
        let t: (i32,) = v.into();
        assert_eq!(t, (1,));
    }
}
