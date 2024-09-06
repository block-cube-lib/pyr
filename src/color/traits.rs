use crate::num::*;
use num::traits::NumAssign;
use std::fmt::Debug;

pub trait ColorElement: NumAssign + Clone + Copy + Debug + Default + Zero + One {
    const LDR_MAX: Self;

    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;

    fn into_signed_integer<T: SignedIntegerColorElement>(self) -> T;
    fn into_unsigned_integer<T: UnsignedIntegerColorElement>(self) -> T;
    fn into_floating_point<T: FloatingPointColorElement>(self) -> T;

    fn as_f64(self) -> f64;

    fn as_self(v: f64) -> Self;
}
pub trait SignedIntegerColorElement: ColorElement + num::Signed + num::Integer {}
pub trait UnsignedIntegerColorElement: ColorElement + num::Unsigned + num::Integer {}
pub trait FloatingPointColorElement: ColorElement + num::Float {}
impl<T> SignedIntegerColorElement for T where T: ColorElement + num::Signed + num::Integer {}
impl<T> UnsignedIntegerColorElement for T where T: ColorElement + num::Unsigned + num::Integer {}
impl<T> FloatingPointColorElement for T where T: ColorElement + num::Float {}

macro_rules! impl_color_element {
    (unsigned_integer; $($t:ty),*) => {
        $(
            impl ColorElement for $t {
                const LDR_MAX: Self = Self::MAX;

                fn add(self, rhs: Self) -> Self {
                    self.saturating_add(rhs)
                }
                fn sub(self, rhs: Self) -> Self {
                    self.saturating_sub(rhs)
                }
                fn mul(self, rhs: Self) -> Self {
                    self.saturating_mul(rhs)
                }
                fn div(self, rhs: Self) -> Self {
                    self.saturating_div(rhs)
                }

                fn into_signed_integer<T: SignedIntegerColorElement>(self) -> T {
                    T::as_self(self.as_f64() / Self::MAX.as_f64() * T::LDR_MAX.as_f64())
                }

                fn into_unsigned_integer<T: UnsignedIntegerColorElement>(self) -> T {
                    T::as_self(self.as_f64() / Self::MAX.as_f64() * T::LDR_MAX.as_f64())
                }

                fn into_floating_point<T: FloatingPointColorElement>(self) -> T {
                    T::as_self(self.as_f64() / Self::MAX.as_f64())
                }

                fn as_f64(self) -> f64 {
                    self as f64
                }

                fn as_self(v: f64) -> Self {
                    v as Self
                }
            }
        )*
    };
    (signed_integer; $($t:ty),*) => {
        $(
            impl ColorElement for $t {
                const LDR_MAX: Self = Self::MAX;

                fn add(self, rhs: Self) -> Self {
                    self.saturating_add(rhs)
                }
                fn sub(self, rhs: Self) -> Self {
                    self.saturating_sub(rhs)
                }
                fn mul(self, rhs: Self) -> Self {
                    self.saturating_mul(rhs)
                }
                fn div(self, rhs: Self) -> Self {
                    self.saturating_div(rhs)
                }

                fn into_signed_integer<T: SignedIntegerColorElement>(self) -> T {
                    T::as_self(self.as_f64() / Self::MAX.as_f64() * T::LDR_MAX.as_f64())
                }

                fn into_unsigned_integer<T: UnsignedIntegerColorElement>(self) -> T {
                    T::as_self(self.max(0).as_f64() / Self::MAX.as_f64() * T::LDR_MAX.as_f64())
                }

                fn into_floating_point<T: FloatingPointColorElement>(self) -> T {
                    T::as_self(self.as_f64() / Self::MAX.as_f64())
                }

                fn as_f64(self) -> f64 {
                    self as f64
                }

                fn as_self(v: f64) -> Self {
                    v as Self
                }
            }
        )*
    };
    (floating_point; $($t:ty),*) => {
        $(
            impl ColorElement for $t {
                const LDR_MAX: Self = 1.0;

                fn add(self, rhs: Self) -> Self {
                    self + rhs
                }
                fn sub(self, rhs: Self) -> Self {
                    self - rhs
                }
                fn mul(self, rhs: Self) -> Self {
                    self * rhs
                }
                fn div(self, rhs: Self) -> Self {
                    self / rhs
                }

                fn into_signed_integer<T: SignedIntegerColorElement>(self) -> T {
                    let v = self.clamp(-1.0, 1.0).as_f64();
                    if v.is_sign_positive() {
                        T::as_self(v * T::LDR_MAX.as_f64())
                    }
                    else {
                        T::as_self(v * (T::LDR_MAX.as_f64() + 1.0))
                    }
                }

                fn into_unsigned_integer<T: UnsignedIntegerColorElement>(self) -> T {
                    T::as_self(self.clamp(0.0, 1.0).as_f64() * T::LDR_MAX.as_f64())
                }

                fn into_floating_point<T: FloatingPointColorElement>(self) -> T {
                    T::as_self(self as f64)
                }

                fn as_f64(self) -> f64 {
                    self as f64
                }

                fn as_self(v: f64) -> Self {
                    v as Self
                }
            }
        )*
    };
}

impl_color_element!(unsigned_integer; u8, u16, u32, u64, u128, usize);
impl_color_element!(signed_integer; i8, i16, i32, i64, i128, isize);
impl_color_element!(floating_point; f32, f64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn into_signed_integer_from_signed_integer() {
        assert_eq!(0_i8.into_signed_integer::<i16>(), 0_i16);
        assert_eq!(i8::MAX.into_signed_integer::<i16>(), i16::MAX);
        assert_eq!(i8::MIN.into_signed_integer::<i16>(), i16::MIN);
    }

    #[test]
    fn into_signed_integer_from_unsigned_integer() {
        assert_eq!(0_u8.into_signed_integer::<i16>(), 0_i16);
        assert_eq!(u8::MAX.into_signed_integer::<i16>(), i16::MAX);
    }

    #[test]
    fn into_signed_integer_from_floating_point() {
        assert_eq!(0.0_f32.into_signed_integer::<i16>(), 0_i16);
        assert_eq!(1.0_f32.into_signed_integer::<i16>(), i16::MAX);
        assert_eq!((-1.0_f32).into_signed_integer::<i16>(), i16::MIN);
        assert_eq!(2.0_f32.into_signed_integer::<i16>(), i16::MAX);
        assert_eq!((-2.0_f32).into_signed_integer::<i16>(), i16::MIN);
    }

    #[test]
    fn into_unsigned_integer_from_signed_integer() {
        assert_eq!(0_i8.into_unsigned_integer::<u16>(), 0_u16);
        assert_eq!(i8::MAX.into_unsigned_integer::<u16>(), u16::MAX);
        assert_eq!(i8::MIN.into_unsigned_integer::<u16>(), 0_u16);
    }

    #[test]
    fn into_unsigned_integer_from_unsigned_integer() {
        assert_eq!(0_i8.into_unsigned_integer::<u16>(), 0_u16);
        assert_eq!(u8::MAX.into_unsigned_integer::<u16>(), u16::MAX);
        assert_eq!(u8::MIN.into_unsigned_integer::<u16>(), u16::MIN);
    }

    #[test]
    fn into_unsigned_integer_from_floating_point() {
        assert_eq!(0.0_f32.into_unsigned_integer::<u16>(), 0_u16);
        assert_eq!(1.0_f32.into_unsigned_integer::<u16>(), u16::MAX);
        assert_eq!((-1.0_f32).into_unsigned_integer::<u16>(), 0_u16);
        assert_eq!(2.0_f32.into_unsigned_integer::<u16>(), u16::MAX);
        assert_eq!((-2.0_f32).into_unsigned_integer::<u16>(), 0_u16);
    }

    #[test]
    fn into_floating_point_from_signed_integer() {
        assert_eq!(0_i8.into_floating_point::<f32>(), 0.0_f32);
        assert_eq!(i8::MAX.into_floating_point::<f32>(), 1.0_f32);
        assert_eq!(i8::MIN.into_floating_point::<f32>(), -1.0_f32);
    }

    #[test]
    fn into_floating_point_from_unsigned_integer() {
        assert_eq!(0_u8.into_floating_point::<f32>(), 0.0_f32);
        assert_eq!(u8::MAX.into_floating_point::<f32>(), 1.0_f32);
    }

    #[test]
    fn into_floating_point_from_floating_point() {
        assert_eq!(0.0_f32.into_floating_point::<f64>(), 0.0_f64);
        assert_eq!(1.0_f32.into_floating_point::<f64>(), 1.0_f64);
        assert_eq!((-1.0_f32).into_floating_point::<f64>(), -1.0_f64);
    }
}

