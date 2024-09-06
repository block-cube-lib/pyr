use crate::color::traits::*;
use crate::color::Rgba;
use crate::math::{vec::VectorElement, Vec3};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::*;

#[repr(C)]
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

impl<T: ColorElement> Default for Rgb<T> {
    fn default() -> Self {
        Self {
            r: T::ZERO,
            g: T::ZERO,
            b: T::ZERO,
        }
    }
}

impl<T: ColorElement> Add for Rgb<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: <T as ColorElement>::add(self.r, rhs.r),
            g: <T as ColorElement>::add(self.g, rhs.g),
            b: <T as ColorElement>::add(self.b, rhs.b),
        }
    }
}

impl<T: ColorElement> Sub for Rgb<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: <T as ColorElement>::sub(self.r, rhs.r),
            g: <T as ColorElement>::sub(self.g, rhs.g),
            b: <T as ColorElement>::sub(self.b, rhs.b),
        }
    }
}

impl<T: ColorElement> Mul for Rgb<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: <T as ColorElement>::mul(self.r, rhs.r),
            g: <T as ColorElement>::mul(self.g, rhs.g),
            b: <T as ColorElement>::mul(self.b, rhs.b),
        }
    }
}

impl<T: ColorElement> Div for Rgb<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            r: <T as ColorElement>::div(self.r, rhs.r),
            g: <T as ColorElement>::div(self.g, rhs.g),
            b: <T as ColorElement>::div(self.b, rhs.b),
        }
    }
}

impl<T: ColorElement> AddAssign for Rgb<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.r = <T as ColorElement>::add(self.r, rhs.r);
        self.g = <T as ColorElement>::add(self.g, rhs.g);
        self.b = <T as ColorElement>::add(self.b, rhs.b);
    }
}

impl<T: ColorElement> SubAssign for Rgb<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.r = <T as ColorElement>::sub(self.r, rhs.r);
        self.g = <T as ColorElement>::sub(self.g, rhs.g);
        self.b = <T as ColorElement>::sub(self.b, rhs.b);
    }
}

impl<T: ColorElement> MulAssign for Rgb<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.r = <T as ColorElement>::mul(self.r, rhs.r);
        self.g = <T as ColorElement>::mul(self.g, rhs.g);
        self.b = <T as ColorElement>::mul(self.b, rhs.b);
    }
}

impl<T: ColorElement> DivAssign for Rgb<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.r = <T as ColorElement>::div(self.r, rhs.r);
        self.g = <T as ColorElement>::div(self.g, rhs.g);
        self.b = <T as ColorElement>::div(self.b, rhs.b);
    }
}

impl<T: ColorElement> Mul<T> for Rgb<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            r: <T as ColorElement>::mul(self.r, rhs),
            g: <T as ColorElement>::mul(self.g, rhs),
            b: <T as ColorElement>::mul(self.b, rhs),
        }
    }
}

impl<T: ColorElement> Div<T> for Rgb<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            r: <T as ColorElement>::div(self.r, rhs),
            g: <T as ColorElement>::div(self.g, rhs),
            b: <T as ColorElement>::div(self.b, rhs),
        }
    }
}

impl<T: ColorElement> MulAssign<T> for Rgb<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.r = <T as ColorElement>::mul(self.r, rhs);
        self.g = <T as ColorElement>::mul(self.g, rhs);
        self.b = <T as ColorElement>::mul(self.b, rhs);
    }
}

impl<T: ColorElement> DivAssign<T> for Rgb<T> {
    fn div_assign(&mut self, rhs: T) {
        self.r = <T as ColorElement>::div(self.r, rhs);
        self.g = <T as ColorElement>::div(self.g, rhs);
        self.b = <T as ColorElement>::div(self.b, rhs);
    }
}

macro_rules! impl_mul_t_rgb {
    ($($t:ty),*) => {
        $(
            impl Mul<Rgb<$t>> for $t {
                type Output = Rgb<$t>;

                fn mul(self, rhs: Rgb<$t>) -> Self::Output {
                    Rgb {
                        r: <$t as ColorElement>::mul(self, rhs.r),
                        g: <$t as ColorElement>::mul(self, rhs.g),
                        b: <$t as ColorElement>::mul(self, rhs.b),
                    }
                }
            }
        )*
    };
}
impl_mul_t_rgb!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl<T: ColorElement> Index<usize> for Rgb<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("out of range"),
        }
    }
}

impl<T: ColorElement> IndexMut<usize> for Rgb<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("out of range"),
        }
    }
}

impl<T: ColorElement> From<[T; 3]> for Rgb<T> {
    fn from(v: [T; 3]) -> Self {
        Self {
            r: v[0],
            g: v[1],
            b: v[2],
        }
    }
}

impl<T: ColorElement> From<Rgb<T>> for [T; 3] {
    fn from(c: Rgb<T>) -> Self {
        [c.r, c.g, c.b]
    }
}

impl<T: ColorElement> From<(T, T, T)> for Rgb<T> {
    fn from(v: (T, T, T)) -> Self {
        Self {
            r: v.0,
            g: v.1,
            b: v.2,
        }
    }
}

impl<T: ColorElement> From<Rgb<T>> for (T, T, T) {
    fn from(c: Rgb<T>) -> Self {
        (c.r, c.g, c.b)
    }
}

impl<T: ColorElement + VectorElement> From<Rgb<T>> for Vec3<T> {
    fn from(c: Rgb<T>) -> Self {
        Vec3::new(c.r, c.g, c.b)
    }
}

impl<T: ColorElement + VectorElement> From<Vec3<T>> for Rgb<T> {
    fn from(v: Vec3<T>) -> Self {
        Self {
            r: v.x,
            g: v.y,
            b: v.z,
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

impl<T: ColorElement> Rgb<T> {
    pub fn from_rgba(rgba: Rgba<T>) -> Self {
        Self {
            r: rgba.r,
            g: rgba.g,
            b: rgba.b,
        }
    }

    pub fn to_rgba(self) -> Rgba<T> {
        Rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: T::LDR_MAX,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let c1 = super::Rgb::<u8>::new(200, 200, 200);
        let c2 = super::Rgb::new(10, 10, 10);
        assert_eq!(c1 + c2, Rgb::<u8>::new(210, 210, 210));
        assert_eq!(c1 + c1, Rgb::<u8>::new(255, 255, 255));
    }
}
