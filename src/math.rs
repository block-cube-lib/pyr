pub mod mat;
pub mod quat;
pub mod rand;
pub mod vec;

pub use self::mat::*;
pub use self::quat::Quat;
pub use self::vec::*;

pub type F32Vec1 = vec::Vec<f32, 1>;
pub type F32Vec2 = vec::Vec<f32, 2>;
pub type F32Vec3 = vec::Vec<f32, 3>;
pub type F32Vec4 = vec::Vec<f32, 4>;
pub type I32Vec1 = vec::Vec<f64, 1>;
pub type I32Vec2 = vec::Vec<f64, 2>;
pub type I32Vec3 = vec::Vec<f64, 3>;
pub type I32Vec4 = vec::Vec<f64, 4>;

//pub type F32Matrix3 = Matrix<f32, 3, 3>;
//pub type F32Matrix4 = Matrix<f32, 4, 4>;
//pub type F64Matrix3 = Matrix<f64, 3, 3>;
//pub type F64Matrix4 = Matrix<f64, 4, 4>;
