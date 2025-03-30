use num::{traits::NumAssignOps, Num};
use seq_macro::seq;

/// A trait that defines the requirements for the type of vector elements.
pub trait VectorElement:
    Num
    + NumAssignOps<Self>
    + Clone
    + Copy
    + std::fmt::Debug
    + Default
    + crate::num::One
    + crate::num::Zero
    + crate::num::AsFloatingPoint
{
}

pub trait FloatVectorElement: VectorElement + num::Float + crate::num::AsFloatingPoint<Output = Self> {}

/// A trait for types that can act like a vector.
pub trait VectorLike<const DIMENSION: usize>:
    Clone
    + Copy
    + PartialEq
    + From<[Self::ElementType; DIMENSION]>
    + Into<[Self::ElementType; DIMENSION]>
{
    type ElementType: VectorElement;

    fn get(&self, index: usize) -> &Self::ElementType;
    fn get_mut(&mut self, index: usize) -> &mut Self::ElementType;
}

impl<T: VectorElement, const DIMENSION: usize> VectorLike<DIMENSION> for [T; DIMENSION] {
    type ElementType = T;

    fn get(&self, index: usize) -> &Self::ElementType {
        &self[index]
    }
    fn get_mut(&mut self, index: usize) -> &mut Self::ElementType {
        &mut self[index]
    }
}

pub trait FloatVectorLike<const DIMENSION: usize>: VectorLike<DIMENSION> {}
impl<V, const DIMENSION: usize> FloatVectorLike<DIMENSION> for V
where
    V: VectorLike<DIMENSION>,
    V::ElementType: FloatVectorElement,
{
}

#[doc(hidden)]
macro_rules! impl_vector_element {
    ($type: ty, $f: ty) => {
        impl VectorElement for $type {}
    };
}
impl_vector_element!(f32, f32);
impl_vector_element!(f64, f64);
impl_vector_element!(i8, f64);
impl_vector_element!(i16, f64);
impl_vector_element!(i32, f64);
impl_vector_element!(i64, f64);
impl_vector_element!(i128, f64);
impl_vector_element!(u8, f64);
impl_vector_element!(u16, f64);
impl_vector_element!(u32, f64);
impl_vector_element!(u64, f64);
impl_vector_element!(u128, f64);
impl_vector_element!(isize, f64);
impl_vector_element!(usize, f64);
impl FloatVectorElement for f32 {}
impl FloatVectorElement for f64 {}

#[doc(hidden)]
macro_rules! impl_vector_like_for_tuple {
    ($n: expr) => {
        seq!(N in 0..$n {
            impl<T: VectorElement> VectorLike<$n> for (#(T,)*) {
                type ElementType = T;

                fn get(&self, index: usize) -> &T {
                    match index {
                        #( N => &self.N, )*
                        _ => panic!("out of range"),
                    }
                }

                fn get_mut(&mut self, index: usize) -> &mut T {
                        match index {
                            #( N => &mut self.N, )*
                            _ => panic!("out of range"),
                        }
                }
            }
        });
    };
}
seq!(D in 1..=12 {
    impl_vector_like_for_tuple!(D);
});

#[cfg(test)]
mod test {
    use super::VectorLike as _;

    #[test]
    fn vector_like_get_tuple_1() {
        let a = (1,);
        assert_eq!(*a.get(0), 1);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn vector_like_get_tuple_1_out_of_range() {
        let a = (1,);
        let _ = a.get(1);
    }

    #[test]
    fn vector_like_get_mut_tuple_1() {
        let mut a = (0,);
        assert_eq!(0, 0);
        *a.get_mut(0) = 1;
        assert_eq!(*a.get(0), 1);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn vector_like_get_mut_tuple_1_out_of_range() {
        let mut a = (0,);
        *a.get_mut(1) = 1;
    }

    #[test]
    fn vector_like_get_tuple_2() {
        let a = (1, 2);
        assert_eq!(*a.get(0), 1);
        assert_eq!(*a.get(1), 2);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn vector_like_get_tuple_2_out_of_range() {
        let a = (1, 2);
        let _ = *a.get(2);
    }

    #[test]
    fn vector_like_get_mut_tuple_2() {
        let mut a = (0, 0);
        assert_eq!(*a.get(0), 0);
        assert_eq!(*a.get(1), 0);
        *a.get_mut(0) = 10;
        *a.get_mut(1) = 20;
        assert_eq!(a.0, 10);
        assert_eq!(a.1, 20);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn vector_like_get_mut_tuple_2_out_of_range() {
        let mut a = (1, 2);
        *a.get_mut(2) = 1;
    }

    #[test]
    fn vector_like_get_tuple_3() {
        let a = (1, 2, 3);
        assert_eq!(*a.get(0), 1);
        assert_eq!(*a.get(1), 2);
        assert_eq!(*a.get(2), 3);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn vector_like_get_tuple_3_out_of_range() {
        let a = (1, 2, 3);
        let _ = a.get(3);
    }

    #[test]
    fn vector_like_set_tuple_3() {
        let mut a = (0, 0, 0);
        assert_eq!(*a.get(0), 0);
        assert_eq!(*a.get(1), 0);
        assert_eq!(*a.get(2), 0);
        *a.get_mut(0) = 10;
        *a.get_mut(1) = 20;
        *a.get_mut(2) = 30;
        assert_eq!(a.0, 10);
        assert_eq!(a.1, 20);
        assert_eq!(a.2, 30);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn vector_like_get_mut_tuple_3_out_of_range() {
        let mut a = (1, 2, 3);
        *a.get_mut(3) = 1;
    }

    #[test]
    fn vector_like_get_array_1() {
        let a = [10];
        assert_eq!(*a.get(0), 10);
    }

    #[test]
    #[should_panic]
    fn vector_like_get_array_1_out_of_range() {
        let a = [10];
        let _ = a.get(1);
    }

    #[test]
    fn vector_like_get_mut_array_1() {
        let mut a = [0];
        assert_eq!(a[0], 0);
        *a.get_mut(0) = 10;
        assert_eq!(a[0], 10);
    }

    #[test]
    #[should_panic]
    fn vector_like_set_array_1_out_of_range() {
        let mut a = [0];
        *a.get_mut(1) = 1;
    }

    #[test]
    fn vector_like_get_array_2() {
        let a = [10, 11];
        assert_eq!(*a.get(0), 10);
        assert_eq!(*a.get(1), 11);
    }

    #[test]
    #[should_panic]
    fn vector_like_get_array_2_out_of_range() {
        let a = [10, 11];
        let _ = a.get(2);
    }

    #[test]
    fn vector_like_set_array_2() {
        let mut a = [0, 0];
        assert_eq!(*a.get(0), 0);
        assert_eq!(*a.get(1), 0);
        *a.get_mut(0) = 10;
        *a.get_mut(1) = 20;
        assert_eq!(*a.get(0), 10);
        assert_eq!(*a.get(1), 20);
    }

    #[test]
    #[should_panic]
    fn vector_like_set_array_2_out_of_range() {
        let mut a = [0, 0];
        *a.get_mut(2) = 2;
    }

    #[test]
    fn vector_like_get_array_3() {
        let a = [10, 11, 12];
        assert_eq!(*a.get(0), 10);
        assert_eq!(*a.get(1), 11);
        assert_eq!(*a.get(2), 12);
    }

    #[test]
    #[should_panic]
    fn vector_like_get_array_3_out_of_range() {
        let a = [10, 11, 12];
        let _ = *a.get(3);
    }

    #[test]
    fn vector_like_set_array_3() {
        let mut a = [0, 0, 0];
        assert_eq!(*a.get(0), 0);
        assert_eq!(*a.get(1), 0);
        assert_eq!(*a.get(2), 0);
        *a.get_mut(0) = 10;
        *a.get_mut(1) = 20;
        *a.get_mut(2) = 30;
        assert_eq!(*a.get(0), 10);
        assert_eq!(*a.get(1), 20);
        assert_eq!(*a.get(2), 30);
    }

    #[test]
    #[should_panic]
    fn vector_like_set_array_3_out_of_range() {
        let mut a = [0, 0, 0];
        *a.get_mut(3) = 3;
    }

    #[test]
    fn vector_like_get_array_4() {
        let a = [10, 11, 12, 13];
        assert_eq!(*a.get(0), 10);
        assert_eq!(*a.get(1), 11);
        assert_eq!(*a.get(2), 12);
        assert_eq!(*a.get(3), 13);
    }

    #[test]
    #[should_panic]
    fn vector_like_get_array_4_out_of_range() {
        let a = [10, 11, 12, 13];
        let _ = *a.get(4);
    }

    #[test]
    fn vector_like_set_array_4() {
        let mut a = [0, 0, 0, 0];
        assert_eq!(*a.get(0), 0);
        assert_eq!(*a.get(1), 0);
        assert_eq!(*a.get(2), 0);
        assert_eq!(*a.get(3), 0);
        *a.get_mut(0) = 10;
        *a.get_mut(1) = 20;
        *a.get_mut(2) = 30;
        *a.get_mut(3) = 40;
        assert_eq!(*a.get(0), 10);
        assert_eq!(*a.get(1), 20);
        assert_eq!(*a.get(2), 30);
        assert_eq!(*a.get(3), 40);
    }

    #[test]
    #[should_panic]
    fn vector_like_set_array_4_out_of_range() {
        let mut a = [0, 0, 0, 0];
        *a.get_mut(4) = 4;
    }

    #[test]
    fn vector_like_get_array_5() {
        let a = [10, 11, 12, 13, 14];
        assert_eq!(*a.get(0), 10);
        assert_eq!(*a.get(1), 11);
        assert_eq!(*a.get(2), 12);
        assert_eq!(*a.get(3), 13);
        assert_eq!(*a.get(4), 14);
    }

    #[test]
    #[should_panic]
    fn vector_like_get_array_5_out_of_range() {
        let a = [10, 11, 12, 13, 14];
        let _ = *a.get(5);
    }

    #[test]
    fn vector_like_set_array_5() {
        let mut a = [0, 0, 0, 0, 0];
        assert_eq!(*a.get(0), 0);
        assert_eq!(*a.get(1), 0);
        assert_eq!(*a.get(2), 0);
        assert_eq!(*a.get(3), 0);
        assert_eq!(*a.get(4), 0);
        *a.get_mut(0) = 10;
        *a.get_mut(1) = 20;
        *a.get_mut(2) = 30;
        *a.get_mut(3) = 40;
        *a.get_mut(4) = 50;
        assert_eq!(*a.get(0), 10);
        assert_eq!(*a.get(1), 20);
        assert_eq!(*a.get(2), 30);
        assert_eq!(*a.get(3), 40);
        assert_eq!(*a.get(4), 50);
    }

    #[test]
    #[should_panic]
    fn vector_like_set_array_5_out_of_range() {
        let mut a = [0, 0, 0, 0, 0];
        *a.get_mut(5) = 5;
    }
}
