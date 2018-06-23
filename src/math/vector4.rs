use super::Vector3;
use std;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
include!("vector_ops.rs");
vector_operators!(Vector4; x, 0; y, 1; z, 2; w, 3);
include!("vector_functions.rs");
vector_functions!(Vector4; x, 0; y, 1; z, 2; w, 3);

impl Vector4 {
    pub fn xyz(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Vector3;
    use super::Vector4;
    vector_operators_test!(Vector4; x, 0; y, 1; z, 2; w, 3);
    vector_functions_test!(Vector4; x, 0; y, 1; z, 2; w, 3);

    #[test]
    fn xyz() {
        let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let xyz = v.xyz();
        assert_eq!(xyz, Vector3::new(1.0, 2.0, 3.0));
    }
}
