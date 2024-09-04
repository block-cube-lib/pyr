pub trait One {
    const ONE: Self;
}

pub trait Zero {
    const ZERO: Self;

    fn is_near_zero(&self) -> bool;
}

// macro: impl One and Zero for primitive
#[doc(hidden)]
macro_rules! impl_one {
    ($type: ty) => {
        impl One for $type {
            const ONE: Self = 1u8 as $type;
        }
    };
}

#[doc(hidden)]
macro_rules! impl_zero {
    ($type: ty, integer) => {
        impl Zero for $type {
            const ZERO: Self = 0;

            fn is_near_zero(&self) -> bool {
                *self == 0
            }
        }
    };
    ($type: ty, floating_point) => {
        impl Zero for $type {
            const ZERO: Self = 0.0;

            fn is_near_zero(&self) -> bool {
                self.abs() < 1e-8
            }
        }
    };
}

impl_one!(i8);
impl_one!(i16);
impl_one!(i32);
impl_one!(i64);
impl_one!(i128);
impl_one!(isize);
impl_one!(u8);
impl_one!(u16);
impl_one!(u32);
impl_one!(u64);
impl_one!(u128);
impl_one!(usize);
impl_one!(f32);
impl_one!(f64);

impl_zero!(i8, integer);
impl_zero!(i16, integer);
impl_zero!(i32, integer);
impl_zero!(i64, integer);
impl_zero!(i128, integer);
impl_zero!(isize, integer);
impl_zero!(u8, integer);
impl_zero!(u16, integer);
impl_zero!(u32, integer);
impl_zero!(u64, integer);
impl_zero!(u128, integer);
impl_zero!(usize, integer);
impl_zero!(f32, floating_point);
impl_zero!(f64, floating_point);
