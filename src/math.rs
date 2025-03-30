pub mod mat;
//pub mod quat;
pub mod rand;
pub mod vec;
mod vec_mat_ops;

pub use mat::*;
pub use vec::*;
//pub use self::quat::Quat;

pub type F32Vec1 = vec::Vec1<f32>;
pub type F32Vec2 = vec::Vec2<f32>;
pub type F32Vec3 = vec::Vec3<f32>;
pub type F32Vec4 = vec::Vec4<f32>;
pub type I32Vec1 = vec::Vec1<f64>;
pub type I32Vec2 = vec::Vec2<f64>;
pub type I32Vec3 = vec::Vec3<f64>;
pub type I32Vec4 = vec::Vec4<f64>;

pub type F32Mat2x2 = Mat<f32, 3, 2>;
pub type F32Mat3x3 = Mat<f32, 3, 3>;
pub type F32Mat4x4 = Mat<f32, 4, 4>;
pub type F64Mat3x3 = Mat<f64, 3, 3>;
pub type F64Mat4x4 = Mat<f64, 4, 4>;
