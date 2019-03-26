use num::Num;

pub trait VectorElement: Num + Clone + Copy {}
impl<T> VectorElement for T where T: Num + Clone + Copy {}

pub trait Vector: Clone + Copy {
    type ElementType: VectorElement;
    const DIMENSION: usize;
}

#[macro_use]
mod ops;

pub mod prelude;

pub mod vector1;
pub mod vector2;
pub mod vector3;
pub mod vector4;

pub use self::vector1::Vector1;
pub use self::vector2::Vector2;
pub use self::vector3::Vector3;
pub use self::vector4::Vector4;
