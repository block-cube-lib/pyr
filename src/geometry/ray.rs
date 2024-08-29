use crate::math::{Vector3, VectorElement};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray<T: VectorElement> {
    origin: Vector3<T>,
    direction: Vector3<T>
}

// impl Eq when if T implements Eq
impl <T: VectorElement + Eq> Eq for Ray<T> {}

impl <T: VectorElement> Ray<T> {
    pub fn new(origin: Vector3<T>, direction: Vector3<T>) -> Ray<T> {
        Ray { origin, direction }
    }

    pub fn point_at(&self, t: T) -> Vector3<T> {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Vector3<T> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<T> {
        self.direction
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayCastResult<T: VectorElement> {
    point: Vector3<T>,
    normal: Vector3<T>,
    t: T,
}

impl<T: VectorElement> RayCastResult<T> {
    pub fn new(point: Vector3<T>, normal: Vector3<T>, t: T) -> RayCastResult<T> {
        RayCastResult { point, normal, t }
    }

    pub fn point(&self) -> &Vector3<T> {
        &self.point
    }
    pub fn normal(&self) -> &Vector3<T> {
        &self.normal
    }
    pub fn t(&self) -> &T {
        &self.t
    }
}

pub trait RayCast<T: VectorElement> {
    fn cast(&self, ray: &Ray<T>) -> Option<RayCastResult<T>>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ray() {
        let origin = Vector3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin(), origin);
        assert_eq!(ray.direction(), direction);
        assert_eq!(ray.point_at(0.0), origin);
        assert_eq!(ray.point_at(1.0), origin + direction);
        assert_eq!(ray.point_at(2.0), origin + direction * 2.0);
    }
}
