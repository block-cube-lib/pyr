use crate::geometry::ray::*;
use crate::math::*;
use crate::num::Zero;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sphere<T: VectorElement> {
    center: Vec3<T>,
    radius: T,
}

impl<T: VectorElement> Eq for Sphere<T> where T: Eq {}

impl<T: VectorElement> Sphere<T> {
    pub fn new(center: impl VectorLike<T, 3>, radius: T) -> Self {
        Self {
            center: Vec3::new(center.get(0), center.get(1), center.get(2)),
            radius,
        }
    }

    pub fn center(&self) -> Vec3<T> {
        self.center
    }

    pub fn center_mut(&mut self) -> &mut Vec3<T> {
        &mut self.center
    }

    pub fn radius(&self) -> T {
        self.radius
    }

    pub fn radius_mut(&mut self) -> &mut T {
        &mut self.radius
    }
}

impl<T: VectorElement + num::Float + num::FromPrimitive + Zero> RayCast<T> for Sphere<T> {
    fn cast(&self, ray: &Ray<T>) -> Option<RayCastHit<T>> {
        let oc = self.center() - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius().powi(2);
        let discriminant = h * h - a * c;
        if discriminant < T::ZERO {
            None
        } else {
            let t = (h - discriminant.sqrt()) / a;
            let p = ray.point_at(t);
            Some(RayCastHit::new(
                p,
                (p - self.center()) / self.radius(),
                t,
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type Vec3 = Vector<f64, 3>;

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
    fn center_mut() {
        let mut sphere = Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        *sphere.center_mut() = Vec3::new(1.0, 2.0, 3.5);
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
        *sphere.radius_mut() = 2.5;
        assert_eq!(sphere.radius(), 2.5);
    }

    #[test]
    fn new1() {
        let v = Vec3::new(1.0, 2.0, 3.5);
        let sphere = Sphere::new(v, 2.5);
        assert_eq!(sphere.center(), v);
        assert_eq!(sphere.radius(), 2.5);
    }

    #[test]
    fn new2() {
        let sphere = Sphere::new([1.0, 2.0, 3.5], 2.5);
        assert_eq!(sphere.center(), Vec3::new(1.0, 2.0, 3.5));
        assert_eq!(sphere.radius(), 2.5);
    }

    #[test]
    fn new3() {
        let sphere = Sphere::new((1.0, 2.0, 3.5), 2.5);
        assert_eq!(sphere.center(), Vec3::new(1.0, 2.0, 3.5));
        assert_eq!(sphere.radius(), 2.5);
    }
}
