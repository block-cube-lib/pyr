#[macro_use]
mod ops;

pub mod vector;
pub mod prelude;

pub mod vector1;
pub mod vector2;
pub mod vector3;
pub mod vector4;

pub use self::vector::{Vector, VectorElement};

pub use self::vector1::Vector1;
pub use self::vector2::Vector2;
pub use self::vector3::Vector3;
pub use self::vector4::Vector4;
