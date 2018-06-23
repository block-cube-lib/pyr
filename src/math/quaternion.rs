use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl ops::Index<usize> for Quaternion {
    type Output = f32;

    fn index<'a>(&'a self, index: usize) -> &'a f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("out of range"),
        }
    }
}

impl ops::IndexMut<usize> for Quaternion {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("out of range"),
        }
    }
}
