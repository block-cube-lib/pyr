use std::ops;
#[cfg(feature = "serde")]
use Serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector1<T> {
    pub x: T,
}

macro_rules! impl_vector1 {
    () => {
        impl_vector1!(f32);
        impl_vector1!(f64);
        impl_vector1!(i8);
        impl_vector1!(i16);
        impl_vector1!(i32);
        impl_vector1!(i64);
        impl_vector1!(i128);
        impl_vector1!(isize);
        impl_vector1!(u8);
        impl_vector1!(u16);
        impl_vector1!(u32);
        impl_vector1!(u64);
        impl_vector1!(u128);
        impl_vector1!(usize);
    };

    ($element_type: ty) => {
        impl Vector1<$element_type> {
            pub const ZERO: Vector1<$element_type> = Vector1 {
                x: 0 as $element_type,
            };
            pub const ONE: Vector1<$element_type> = Vector1 {
                x: 1 as $element_type,
            };
            pub const UNIT_X: Vector1<$element_type> = Vector1 {
                x: 1 as $element_type,
            };
        }
    };
}

impl_vector1!();

impl_ops!(Vector1; x);

#[cfg(test)]
mod test {
    use super::*;
    use proptest::*;
    ops_test!(Vector1; x);
    //#[test]
    //#[should_panic]
    //fn index_out_of_range(i in index_max!(x)..100_usize) {
    //    let v = Vector1 { x: 0.0 };
    //    let _ = v[i];
    //}
}
