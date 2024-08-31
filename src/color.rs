mod rgb;
mod rgba;
mod traits;

pub use rgb::Rgb;
pub use rgba::Rgba;
pub use traits::*;

use crate::math::{vec::VectorElement, Vec3, Vec4};

impl<T: VectorElement + ColorElement> From<Vec3<T>> for Rgb<T> {
    fn from(v: Vec3<T>) -> Self {
        Self {
            r: v.x,
            g: v.y,
            b: v.z,
        }
    }
}

impl<T: VectorElement + ColorElement> From<Vec4<T>> for Rgba<T> {
    fn from(v: Vec4<T>) -> Self {
        Self {
            r: v.x,
            g: v.y,
            b: v.z,
            a: v.w,
        }
    }
}

impl<T: VectorElement + ColorElement> From<Rgb<T>> for Vec3<T> {
    fn from(c: Rgb<T>) -> Self {
        Vec3::new(c.r, c.g, c.b)
    }
}

impl<T: VectorElement + ColorElement> From<Rgba<T>> for Vec4<T> {
    fn from(c: Rgba<T>) -> Self {
        Vec4::new(c.r, c.g, c.b, c.a)
    }
}
