use crate::geometry::ray::*;
use crate::math::vec::ops;
use crate::math::*;
use crate::num::Zero;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sphere<V: VectorLike<3>> {
    center: V,
    radius: V::ElementType,
}

impl<V: VectorLike<3> + Copy> Eq for Sphere<V> where V: Eq {}

impl<V: VectorLike<3> + Copy> Sphere<V> {
    pub fn new(
        center: impl VectorLike<3, ElementType = V::ElementType>,
        radius: V::ElementType,
    ) -> Self {
        Self {
            center: center.into_other_vector(),
            radius,
        }
    }

    pub fn center(&self) -> V {
        self.center
    }

    pub fn set_center(&mut self, center: impl VectorLike<3, ElementType = V::ElementType>) {
        self.center = center.into_other_vector();
    }

    pub fn radius(&self) -> V::ElementType {
        self.radius
    }

    pub fn set_radius(&mut self, radius: V::ElementType) {
        self.radius = radius;
    }
}

impl<V: FloatVectorLike<3>> RayCast<V> for Sphere<V>
where
    V::ElementType: FloatVectorElement,
{
    fn cast(&self, ray: &Ray<V>) -> Option<RayCastHit<V>> {
        use num::Float as _;
        let oc: V = ops::sub(self.center(), ray.origin());
        let a = ops::length_squared(ray.direction());
        let h = ops::dot(ray.direction(), oc);
        let c = ops::length_squared(oc) - self.radius().powi(2);
        let discriminant = h * h - a * c;
        if discriminant < V::ElementType::ZERO {
            None
        } else {
            let t = (h - discriminant.sqrt()) / a;
            let p = ray.point_at(t);
            let pc: V = ops::sub(p, self.center());
            let normal: V = ops::div_scalar(pc, self.radius());
            Some(RayCastHit::new(p, normal.into_other_vector(), t))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type Vec3 = Vec<f64, 3>;

    #[test]
    fn center() {
        let sphere = Sphere {
            center: Vec3::new(1.0, 2.0, 3.5),
            radius: 1.0,
        };
        assert_eq!(sphere.center(), Vec3::new(1.0, 2.0, 3.5));

        let sphere = Sphere {
            center: Vec3::new(2.0, 4.0, -6.0),
            radius: 1.0,
        };
        assert_eq!(sphere.center(), Vec3::new(2.0, 4.0, -6.0));
    }

    #[test]
    fn set_center() {
        let mut sphere = Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        sphere.set_center(Vec3::new(1.0, 2.0, 3.5));
        assert_eq!(sphere.center(), Vec3::new(1.0, 2.0, 3.5));
    }

    #[test]
    fn radius() {
        let sphere = Sphere {
            center: Vec3::new(1.0, 2.0, 3.0),
            radius: 1.0,
        };
        assert_eq!(sphere.radius(), 1.0);

        let sphere = Sphere {
            center: Vec3::new(1.0, 2.0, 3.0),
            radius: 2.5,
        };
        assert_eq!(sphere.radius(), 2.5);
    }

    #[test]
    fn radius_mut() {
        let mut sphere = Sphere {
            center: Vec3::new(1.0, 2.0, 3.0),
            radius: 1.0,
        };
        sphere.set_radius(2.5);
        assert_eq!(sphere.radius(), 2.5);
    }

    #[test]
    fn new1() {
        let v = Vec3::new(1.0, 2.0, 3.5);
        let sphere = Sphere::<Vec3>::new(v, 2.5);
        assert_eq!(sphere.center(), v);
        assert_eq!(sphere.radius(), 2.5);
    }

    #[test]
    fn new2() {
        let sphere = Sphere::<Vec3>::new([1.0, 2.0, 3.5], 2.5);
        assert_eq!(sphere.center(), Vec3::new(1.0, 2.0, 3.5));
        assert_eq!(sphere.radius(), 2.5);
    }

    #[test]
    fn new3() {
        let sphere = Sphere::<Vec3>::new((1.0, 2.0, 3.5), 2.5);
        assert_eq!(sphere.center(), Vec3::new(1.0, 2.0, 3.5));
        assert_eq!(sphere.radius(), 2.5);
    }
}
