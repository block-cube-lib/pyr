pub trait One {
    const ONE: Self;
}

pub trait Zero {
    const ZERO: Self;
}

// macro: impl One and Zero for primitive
#[doc(hidden)]
macro_rules! impl_one_zero {
    ($type: ty) => {
        impl One for $type {
            const ONE: Self = 1u8 as $type;
        }
        impl Zero for $type {
            const ZERO: Self = 0u8 as $type;
        }
    };
}

impl_one_zero!(i8);
impl_one_zero!(i16);
impl_one_zero!(i32);
impl_one_zero!(i64);
impl_one_zero!(i128);
impl_one_zero!(isize);
impl_one_zero!(u8);
impl_one_zero!(u16);
impl_one_zero!(u32);
impl_one_zero!(u64);
impl_one_zero!(u128);
impl_one_zero!(usize);
impl_one_zero!(f32);
impl_one_zero!(f64);
