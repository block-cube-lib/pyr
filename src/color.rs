mod rgb;
mod rgba;
mod traits;

pub use rgb::Rgb;
pub use rgba::Rgba;
pub use traits::*;

use crate::math::{vector::VectorElement, Vector3, Vector4};

impl<T: VectorElement + ColorElement> From<Vector3<T>> for Rgb<T> {
    fn from(v: Vector3<T>) -> Self {
        Self {
            r: v.x,
            g: v.y,
            b: v.z,
        }
    }
}

impl<T: VectorElement + ColorElement> From<Vector4<T>> for Rgba<T> {
    fn from(v: Vector4<T>) -> Self {
        Self {
            r: v.x,
            g: v.y,
            b: v.z,
            a: v.w,
        }
    }
}

impl<T: VectorElement + ColorElement> From<Rgb<T>> for Vector3<T> {
    fn from(c: Rgb<T>) -> Self {
        Vector3::new(c.r, c.g, c.b)
    }
}

impl<T: VectorElement + ColorElement> From<Rgba<T>> for Vector4<T> {
    fn from(c: Rgba<T>) -> Self {
        Vector4::new(c.r, c.g, c.b, c.a)
    }
}
