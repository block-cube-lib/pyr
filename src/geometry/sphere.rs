use crate::math::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sphere<T: VectorElement> {
    center: Vector3<T>,
    radius: T,
}

impl<T: VectorElement> Eq for Sphere<T> where T: Eq {}

impl<T: VectorElement> Sphere<T> {
    pub fn new(center: impl VectorLike<T, 3>, radius: T) -> Self {
        Self {
            center: Vector3::new(center.get(0), center.get(1), center.get(2)),
            radius,
        }
    }

    pub fn center(&self) -> Vector3<T> {
        self.center
    }

    pub fn center_mut(&mut self) -> &mut Vector3<T> {
        &mut self.center
    }

    pub fn radius(&self) -> T {
        self.radius
    }

    pub fn radius_mut(&mut self) -> &mut T {
        &mut self.radius
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type Vector3 = Vector<f64, 3>;

    #[test]
    fn center() {
        let sphere = Sphere {
            center: Vector3::new(1.0, 2.0, 3.5),
            radius: 1.0,
        };
        assert_eq!(sphere.center(), Vector3::new(1.0, 2.0, 3.5));

        let sphere = Sphere {
            center: Vector3::new(2.0, 4.0, -6.0),
            radius: 1.0,
        };
        assert_eq!(sphere.center(), Vector3::new(2.0, 4.0, -6.0));
    }

    #[test]
    fn center_mut() {
        let mut sphere = Sphere {
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        *sphere.center_mut() = Vector3::new(1.0, 2.0, 3.5);
        assert_eq!(sphere.center(), Vector3::new(1.0, 2.0, 3.5));
    }

    #[test]
    fn radius() {
        let sphere = Sphere {
            center: Vector3::new(1.0, 2.0, 3.0),
            radius: 1.0,
        };
        assert_eq!(sphere.radius(), 1.0);

        let sphere = Sphere {
            center: Vector3::new(1.0, 2.0, 3.0),
            radius: 2.5,
        };
        assert_eq!(sphere.radius(), 2.5);
    }

    #[test]
    fn radius_mut() {
        let mut sphere = Sphere {
            center: Vector3::new(1.0, 2.0, 3.0),
            radius: 1.0,
        };
        *sphere.radius_mut() = 2.5;
        assert_eq!(sphere.radius(), 2.5);
    }

    #[test]
    fn new1() {
        let v = Vector3::new(1.0, 2.0, 3.5);
        let sphere = Sphere::new(v, 2.5);
        assert_eq!(sphere.center(), v);
        assert_eq!(sphere.radius(), 2.5);
    }

    #[test]
    fn new2() {
        let sphere = Sphere::new([1.0, 2.0, 3.5], 2.5);
        assert_eq!(sphere.center(), Vector3::new(1.0, 2.0, 3.5));
        assert_eq!(sphere.radius(), 2.5);
    }

    #[test]
    fn new3() {
        let sphere = Sphere::new((1.0, 2.0, 3.5), 2.5);
        assert_eq!(sphere.center(), Vector3::new(1.0, 2.0, 3.5));
        assert_eq!(sphere.radius(), 2.5);
    }
}
