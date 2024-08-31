use crate::math::{Vec3, VectorElement};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray<T: VectorElement> {
    origin: Vec3<T>,
    direction: Vec3<T>
}

// impl Eq when if T implements Eq
impl <T: VectorElement + Eq> Eq for Ray<T> {}

impl <T: VectorElement> Ray<T> {
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Ray<T> {
        Ray { origin, direction }
    }

    pub fn point_at(&self, t: T) -> Vec3<T> {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Vec3<T> {
        self.origin
    }

    pub fn direction(&self) -> Vec3<T> {
        self.direction
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayCastHit<T: VectorElement> {
    point: Vec3<T>,
    normal: Vec3<T>,
    t: T,
}

impl<T: VectorElement> RayCastHit<T> {
    pub fn new(point: Vec3<T>, normal: Vec3<T>, t: T) -> RayCastHit<T> {
        RayCastHit { point, normal, t }
    }

    pub fn point(&self) -> Vec3<T> {
        self.point
    }
    pub fn normal(&self) -> Vec3<T> {
        self.normal
    }
    pub fn t(&self) -> T {
        self.t
    }
}

pub trait RayCast<T: VectorElement> {
    fn cast(&self, ray: &Ray<T>) -> Option<RayCastHit<T>>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ray() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin(), origin);
        assert_eq!(ray.direction(), direction);
        assert_eq!(ray.point_at(0.0), origin);
        assert_eq!(ray.point_at(1.0), origin + direction);
        assert_eq!(ray.point_at(2.0), origin + direction * 2.0);
    }
}
