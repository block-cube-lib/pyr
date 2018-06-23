use std;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
include!("vector_ops.rs");
vector_operators!(Vector3; x, 0; y, 1; z, 2);
include!("vector_functions.rs");
vector_functions!(Vector3; x, 0; y, 1; z, 2);

impl From<[f32; 3]> for Vector3 {
    fn from(v: [f32; 3]) -> Vector3 {
        Vector3::new(v[0], v[1], v[2])
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(v: (f32, f32, f32)) -> Vector3 {
        Vector3::new(v.0, v.1, v.2)
    }
}

#[cfg(test)]
mod test {
    use super::Vector3;
    vector_operators_test!(Vector3; x, 0; y, 1; z, 2);
    vector_functions_test!(Vector3; x, 0; y, 1; z, 2);
}
