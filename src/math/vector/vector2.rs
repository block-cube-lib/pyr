use std;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
include!("vector_ops.rs");
vector_operators!(Vector2; x, 0; y, 1);
include!("vector_functions.rs");
vector_functions!(Vector2; x, 0; y, 1);

// static
impl Vector2 {
    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0)
    }

    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0)
    }

    pub fn yx(&self) -> Self {
        Vector2::new(self.y, self.x)
    }
    pub fn cross(&self, other: Vector2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

#[cfg(test)]
mod test {
    use super::Vector2;
    vector_operators_test!(Vector2; x, 0; y, 1);
    vector_functions_test!(Vector2; x, 0; y, 1);

    #[test]
    fn unit_x() {
        assert_eq!(Vector2::unit_x(), Vector2::new(1.0, 0.0));
    }

    #[test]
    fn unit_y() {
        assert_eq!(Vector2::unit_y(), Vector2::new(0.0, 1.0));
    }

    #[test]
    fn yx() {
        let v = Vector2::new(12.3, 4.5);
        assert_eq!(v.yx(), Vector2::new(v.y, v.x));
    }

    #[test]
    fn cross() {
        let a = Vector2::new(2.0, 3.0);
        let b = Vector2::new(4.0, 5.0);
        assert_eq!(a.cross(b), -2.0);
    }
}
