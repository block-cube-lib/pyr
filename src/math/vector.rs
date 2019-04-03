use num::Num;

pub trait VectorElement: Num + Clone + Copy + std::fmt::Debug + Default {}
impl<T> VectorElement for T where T: Num + Clone + Copy + std::fmt::Debug + Default {}

pub trait Vector: Clone + Copy + Default {
    type ElementType: VectorElement;
    const DIMENSION: usize;
}

#[macro_use]
mod ops;

pub mod prelude;

mod vector1;
mod vector2;
mod vector3;
mod vector4;

pub use self::vector1::Vector1;
pub use self::vector2::Vector2;
pub use self::vector3::Vector3;
pub use self::vector4::Vector4;
