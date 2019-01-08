use std::ops;
#[cfg(feature = "serde")]
use Serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

macro_rules! impl_vector2 {
    () => {
        impl_vector2!(f32);
        impl_vector2!(f64);
        impl_vector2!(i8);
        impl_vector2!(i16);
        impl_vector2!(i32);
        impl_vector2!(i64);
        impl_vector2!(i128);
        impl_vector2!(isize);
        impl_vector2!(u8);
        impl_vector2!(u16);
        impl_vector2!(u32);
        impl_vector2!(u64);
        impl_vector2!(u128);
        impl_vector2!(usize);
    };

    ($element_type: ty) => {
        impl Vector2<$element_type> {
            pub const ZERO: Vector2<$element_type> = Vector2 {
                x: 0 as $element_type,
                y: 0 as $element_type,
            };
            pub const ONE: Vector2<$element_type> = Vector2 {
                x: 1 as $element_type,
                y: 1 as $element_type,
            };
            pub const UNIT_X: Vector2<$element_type> = Vector2 {
                x: 1 as $element_type,
                y: 0 as $element_type,
            };
            pub const UNIT_Y: Vector2<$element_type> = Vector2 {
                x: 0 as $element_type,
                y: 1 as $element_type,
            };
        }
    };
}

impl_vector2!();

impl_ops!(Vector2; x, y);

#[cfg(test)]
mod test {
    use super::*;
    use proptest::*;
    ops_test!(Vector2; x, y);
}
