use crate::color::traits::*;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rgb<T: ColorElement> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: ColorElement> Rgb<T> {
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl<T: ColorElement + Eq> Eq for Rgb<T> where T: Eq {}

impl<T: ColorElement> Color<T> for Rgb<T> {
    fn r(&self) -> T {
        self.r
    }

    fn g(&self) -> T {
        self.g
    }

    fn b(&self) -> T {
        self.b
    }

    fn a(&self) -> T {
        T::ONE
    }
}


// impl ops for integer types
macro_rules! impl_ops {
    () => {
        impl_ops!(u8, integer);
        impl_ops!(u16, integer);
        impl_ops!(u32, integer);
        impl_ops!(u64, integer);
        impl_ops!(u128, integer);
        impl_ops!(usize, integer);
        impl_ops!(i8, integer);
        impl_ops!(i16, integer);
        impl_ops!(i32, integer);
        impl_ops!(i64, integer);
        impl_ops!(i128, integer);
        impl_ops!(isize, integer);

        impl_ops!(f32, float);
        impl_ops!(f64, float);
    };
    ($t: ty, integer) => {
        impl Add for Rgb<$t> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self {
                    r: self.r.saturating_add(rhs.r),
                    g: self.g.saturating_add(rhs.g),
                    b: self.b.saturating_add(rhs.b),
                }
            }
        }
        impl Sub for Rgb<$t> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self {
                    r: self.r.saturating_sub(rhs.r),
                    g: self.g.saturating_sub(rhs.g),
                    b: self.b.saturating_sub(rhs.b),
                }
            }
        }
        impl Mul for Rgb<$t> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                Self {
                    r: self.r.saturating_mul(rhs.r),
                    g: self.g.saturating_mul(rhs.g),
                    b: self.b.saturating_mul(rhs.b),
                }
            }
        }
        impl Div for Rgb<$t> {
            type Output = Self;
            fn div(self, rhs: Self) -> Self {
                Self {
                    r: self.r.saturating_div(rhs.r),
                    g: self.g.saturating_div(rhs.g),
                    b: self.b.saturating_div(rhs.b),
                }
            }
        }

        impl AddAssign for Rgb<$t> {
            fn add_assign(&mut self, rhs: Self) {
                self.r = self.r.saturating_add(rhs.r);
                self.g = self.g.saturating_add(rhs.g);
                self.b = self.b.saturating_add(rhs.b);
            }
        }
        impl SubAssign for Rgb<$t> {
            fn sub_assign(&mut self, rhs: Self) {
                self.r = self.r.saturating_sub(rhs.r);
                self.g = self.g.saturating_sub(rhs.g);
                self.b = self.b.saturating_sub(rhs.b);
            }
        }
        impl MulAssign for Rgb<$t> {
            fn mul_assign(&mut self, rhs: Self) {
                self.r = self.r.saturating_mul(rhs.r);
                self.g = self.g.saturating_mul(rhs.g);
                self.b = self.b.saturating_mul(rhs.b);
            }
        }
        impl DivAssign for Rgb<$t> {
            fn div_assign(&mut self, rhs: Self) {
                self.r = self.r.saturating_div(rhs.r);
                self.g = self.g.saturating_div(rhs.g);
                self.b = self.b.saturating_div(rhs.b);
            }
        }

        impl MulAssign<$t> for Rgb<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                self.r = self.r.saturating_mul(rhs);
                self.g = self.g.saturating_mul(rhs);
                self.b = self.b.saturating_mul(rhs);
            }
        }
        impl DivAssign<$t> for Rgb<$t> {
            fn div_assign(&mut self, rhs: $t) {
                self.r = self.r.saturating_div(rhs);
                self.g = self.g.saturating_div(rhs);
                self.b = self.b.saturating_div(rhs);
            }
        }

        impl Mul<Rgb<$t>> for $t {
            type Output = Rgb<$t>;
            fn mul(self, rhs: Rgb<$t>) -> Self::Output {
                Rgb {
                    r: self * rhs.r,
                    g: self * rhs.g,
                    b: self * rhs.b,
                }
            }
        }
    };
    ($t: ty, float) => {
        impl_ops!($t, Add, add, +, ops, float);
        impl_ops!($t, Sub, sub, -, ops, float);
        impl_ops!($t, Mul, mul, *, ops, float);
        impl_ops!($t, Div, div, /, ops, float);

        impl_ops!($t, AddAssign, add_assign, +=, ops_assign, float);
        impl_ops!($t, SubAssign, sub_assign, -=, ops_assign, float);
        impl_ops!($t, MulAssign, mul_assign, *=, ops_assign, float);
        impl_ops!($t, DivAssign, div_assign, /=, ops_assign, float);

        impl Mul<$t> for Rgb<$t> {
            type Output = Self;
            fn mul(self, rhs: $t) -> Self {
                Self {
                    r: self.r * rhs,
                    g: self.g * rhs,
                    b: self.b * rhs,
                }
            }
        }

        impl Div<$t> for Rgb<$t> {
            type Output = Self;
            fn div(self, rhs: $t) -> Self {
                Self {
                    r: self.r / rhs,
                    g: self.g / rhs,
                    b: self.b / rhs,
                }
            }
        }

        impl MulAssign<$t> for Rgb<$t> {
            fn mul_assign(&mut self, rhs: $t) {
                self.r *= rhs;
                self.g *= rhs;
                self.b *= rhs;
            }
        }

        impl DivAssign<$t> for Rgb<$t> {
            fn div_assign(&mut self, rhs: $t) {
                self.r /= rhs;
                self.g /= rhs;
                self.b /= rhs;
            }
        }

        impl Mul<Rgb<$t>> for $t {
            type Output = Rgb<$t>;
            fn mul(self, rhs: Rgb<$t>) -> Rgb<$t> {
                Rgb {
                    r: self * rhs.r,
                    g: self * rhs.g,
                    b: self * rhs.b,
                }
            }
        }
    };
    ($t:ty, $ops_name:ident, $func_name:ident, $op:tt, ops, float) => {
        impl $ops_name for Rgb<$t> {
            type Output = Self;
            fn $func_name(self, rhs: Self) -> Self {
                Self {
                    r: self.r $op rhs.r,
                    g: self.g $op rhs.g,
                    b: self.b $op rhs.b,
                }
            }
        }
    };
    ($t:ty, $ops_name:ident, $func_name:ident, $op:tt, ops_assign, float) => {
        impl $ops_name for Rgb<$t> {
            fn $func_name(&mut self, rhs: Self) {
                self.r $op rhs.r;
                self.g $op rhs.g;
                self.b $op rhs.b;
            }
        }
    };
}

impl_ops!();
