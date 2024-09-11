use crate::math::vec::ops;
use crate::math::{FloatVectorElement, FloatVectorLike, VectorElement, VectorLike};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray<V: FloatVectorLike<3>> {
    origin: V,
    direction: V,
}

impl<V: FloatVectorLike<3>> Ray<V>
where
    V::ElementType: FloatVectorElement,
{
    pub fn new(
        origin: impl VectorLike<3, ElementType = V::ElementType>,
        direction: impl VectorLike<3, ElementType = V::ElementType>,
    ) -> Self {
        Ray {
            origin: origin.into_other_vector(),
            direction: ops::normalized(direction),
        }
    }

    pub fn point_at(&self, t: V::ElementType) -> V {
        let dir_mul_t: V = ops::mul_scalar(self.direction, t);
        ops::add(self.origin, dir_mul_t)
    }

    pub fn origin(&self) -> V {
        self.origin
    }

    pub fn direction(&self) -> V {
        self.direction
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RayCastHit<V: FloatVectorLike<3>> {
    point: V,
    normal: V,
    t: V::ElementType,
}

impl<V: FloatVectorLike<3>> RayCastHit<V> {
    pub fn new(point: V, normal: V, t: V::ElementType) -> RayCastHit<V> {
        Self { point, normal, t }
    }

    pub fn point(&self) -> V {
        self.point
    }
    pub fn normal(&self) -> V {
        self.normal
    }
    pub fn t(&self) -> V::ElementType {
        self.t
    }
}

pub trait RayCast<V: FloatVectorLike<3>> {
    fn cast(&self, ray: &Ray<V>) -> Option<RayCastHit<V>>;
}

#[cfg(test)]
mod test {
    use crate::math::vec::VectorLike;
    type Vec3 = crate::math::vec::Vec3<f64>;
    type Ray = super::Ray<[f64; 3]>;

    #[test]
    fn test_ray() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        let nd = direction.normalized();

        assert_eq!(ray.origin(), origin.into_other_vector::<[f64; 3]>());
        assert_eq!(ray.direction(), nd.into_other_vector::<[f64; 3]>());
        assert_eq!(ray.point_at(0.0), origin.into_other_vector::<[f64; 3]>());
        assert_eq!(ray.point_at(1.0), (origin + nd).into_other_vector::<[f64; 3]>());
        assert_eq!(ray.point_at(2.0), (origin + nd * 2.0).into_other_vector::<[f64; 3]>());
    }
}
