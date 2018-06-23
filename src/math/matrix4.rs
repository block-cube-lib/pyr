use super::vector4::Vector4;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Matrix4 {
    pub m: [Vector4; 4],
}

impl Matrix4 {
    pub fn zero() -> Matrix4 {
        Matrix4 {
            m: [
                Vector4::zero(),
                Vector4::zero(),
                Vector4::zero(),
                Vector4::zero(),
            ],
        }
    }

    pub fn identity() -> Matrix4 {
        Matrix4 {
            m: [
                Vector4::new(1.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 1.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 1.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 1.0),
            ],
        }
    }
}

impl ops::Index<usize> for Matrix4 {
    type Output = Vector4;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.m[index]
    }
}

impl ops::IndexMut<usize> for Matrix4 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Vector4 {
        &mut self.m[index]
    }
}
