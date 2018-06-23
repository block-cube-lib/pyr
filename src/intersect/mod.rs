pub trait Intersect<RHS = Self> {
    type Output;

    fn intersect(self, rhs: RHS) -> Self::Output;
}

use math::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}
