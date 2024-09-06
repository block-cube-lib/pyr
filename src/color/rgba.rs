use crate::color::traits::*;
use crate::color::Rgb;
use crate::math::{vec::VectorElement, Vec4};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rgba<T: ColorElement> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T: ColorElement> Rgba<T> {
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl<T: ColorElement + Eq> Eq for Rgba<T> where T: Eq {}

impl<T: ColorElement> Default for Rgba<T> {
    fn default() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::one())
    }
}

impl<T: ColorElement> Add for Rgba<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(
            <T as ColorElement>::add(self.r, rhs.r),
            <T as ColorElement>::add(self.g, rhs.g),
            <T as ColorElement>::add(self.b, rhs.b),
            <T as ColorElement>::add(self.a, rhs.a),
        )
    }
}

impl<T: ColorElement> Sub for Rgba<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(
            <T as ColorElement>::sub(self.r, rhs.r),
            <T as ColorElement>::sub(self.g, rhs.g),
            <T as ColorElement>::sub(self.b, rhs.b),
            <T as ColorElement>::sub(self.a, rhs.a),
        )
    }
}

impl<T: ColorElement> Mul for Rgba<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(
            <T as ColorElement>::mul(self.r, rhs.r),
            <T as ColorElement>::mul(self.g, rhs.g),
            <T as ColorElement>::mul(self.b, rhs.b),
            <T as ColorElement>::mul(self.a, rhs.a),
        )
    }
}

impl<T: ColorElement> Div for Rgba<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(
            <T as ColorElement>::div(self.r, rhs.r),
            <T as ColorElement>::div(self.g, rhs.g),
            <T as ColorElement>::div(self.b, rhs.b),
            <T as ColorElement>::div(self.a, rhs.a),
        )
    }
}

impl<T: ColorElement> AddAssign for Rgba<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.r = <T as ColorElement>::add(self.r, rhs.r);
        self.g = <T as ColorElement>::add(self.g, rhs.g);
        self.b = <T as ColorElement>::add(self.b, rhs.b);
        self.a = <T as ColorElement>::add(self.a, rhs.a);
    }
}

impl<T: ColorElement> SubAssign for Rgba<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.r = <T as ColorElement>::sub(self.r, rhs.r);
        self.g = <T as ColorElement>::sub(self.g, rhs.g);
        self.b = <T as ColorElement>::sub(self.b, rhs.b);
        self.a = <T as ColorElement>::sub(self.a, rhs.a);
    }
}

impl<T: ColorElement> MulAssign for Rgba<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.r = <T as ColorElement>::mul(self.r, rhs.r);
        self.g = <T as ColorElement>::mul(self.g, rhs.g);
        self.b = <T as ColorElement>::mul(self.b, rhs.b);
        self.a = <T as ColorElement>::mul(self.a, rhs.a);
    }
}

impl<T: ColorElement> DivAssign for Rgba<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.r = <T as ColorElement>::div(self.r, rhs.r);
        self.g = <T as ColorElement>::div(self.g, rhs.g);
        self.b = <T as ColorElement>::div(self.b, rhs.b);
        self.a = <T as ColorElement>::div(self.a, rhs.a);
    }
}

impl<T: ColorElement> Mul<T> for Rgba<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(
            <T as ColorElement>::mul(self.r, rhs),
            <T as ColorElement>::mul(self.g, rhs),
            <T as ColorElement>::mul(self.b, rhs),
            <T as ColorElement>::mul(self.a, rhs),
        )
    }
}

impl<T: ColorElement> Div<T> for Rgba<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(
            <T as ColorElement>::div(self.r, rhs),
            <T as ColorElement>::div(self.g, rhs),
            <T as ColorElement>::div(self.b, rhs),
            <T as ColorElement>::div(self.a, rhs),
        )
    }
}

impl<T: ColorElement> MulAssign<T> for Rgba<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.r = <T as ColorElement>::mul(self.r, rhs);
        self.g = <T as ColorElement>::mul(self.g, rhs);
        self.b = <T as ColorElement>::mul(self.b, rhs);
        self.a = <T as ColorElement>::mul(self.a, rhs);
    }
}

impl<T: ColorElement> DivAssign<T> for Rgba<T> {
    fn div_assign(&mut self, rhs: T) {
        self.r = <T as ColorElement>::div(self.r, rhs);
        self.g = <T as ColorElement>::div(self.g, rhs);
        self.b = <T as ColorElement>::div(self.b, rhs);
        self.a = <T as ColorElement>::div(self.a, rhs);
    }
}

macro_rules! impl_mul_t_rgba {
    ($($t:ty),*) => {
        $(
            impl Mul<Rgba<$t>> for $t {
                type Output = Rgba<$t>;

                fn mul(self, rhs: Rgba<$t>) -> Self::Output {
                    Rgba {
                        r: <$t as ColorElement>::mul(self, rhs.r),
                        g: <$t as ColorElement>::mul(self, rhs.g),
                        b: <$t as ColorElement>::mul(self, rhs.b),
                        a: <$t as ColorElement>::mul(self, rhs.a),
                    }
                }
            }
        )*
    };
}
impl_mul_t_rgba!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl<T: ColorElement> Index<usize> for Rgba<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!("out of range"),
        }
    }
}

impl<T: ColorElement> IndexMut<usize> for Rgba<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => panic!("out of range"),
        }
    }
}

impl<T: ColorElement> From<[T; 4]> for Rgba<T> {
    fn from(v: [T; 4]) -> Self {
        Self {
            r: v[0],
            g: v[1],
            b: v[2],
            a: v[3],
        }
    }
}

impl<T: ColorElement> From<Rgba<T>> for [T; 4] {
    fn from(c: Rgba<T>) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl<T: ColorElement> From<(T, T, T, T)> for Rgba<T> {
    fn from(v: (T, T, T, T)) -> Self {
        Self {
            r: v.0,
            g: v.1,
            b: v.2,
            a: v.3,
        }
    }
}

impl<T: ColorElement> From<Rgba<T>> for (T, T, T, T) {
    fn from(c: Rgba<T>) -> Self {
        (c.r, c.g, c.b, c.a)
    }
}

impl<T: ColorElement + VectorElement> From<Rgba<T>> for Vec4<T> {
    fn from(c: Rgba<T>) -> Self {
        Vec4::new(c.r, c.g, c.b, c.a)
    }
}

impl<T: ColorElement + VectorElement> From<Vec4<T>> for Rgba<T> {
    fn from(v: Vec4<T>) -> Self {
        Self {
            r: v.x,
            g: v.y,
            b: v.z,
            a: v.w,
        }
    }
}

impl<T: ColorElement> From<Rgb<T>> for Rgba<T> {
    fn from(rgb: Rgb<T>) -> Self {
        Rgba {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: T::LDR_MAX,
        }
    }
}

impl<T: ColorElement> Rgba<T> {
    pub fn from_rgb(rgb: Rgb<T>) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: T::LDR_MAX,
        }
    }

    pub fn to_rgb(self) -> Rgb<T> {
        Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}
