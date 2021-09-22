use super::prelude::*;
use super::{Vector, VectorElement};
use num::{pow, One, Zero};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector3<T: VectorElement> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: VectorElement> Vector for Vector3<T> {
    type ElementType = T;
    const DIMENSION: usize = 3;
}

impl<T: VectorElement + Eq> Eq for Vector3<T> {}

//============================================================
// operator
//============================================================
impl_vector_ops!(Vector3; x, y, z);

//============================================================
// impl num
//============================================================
/// Make a zero vector.
/// Same to Vector3 { x: 0, y: 0, z: 0 }.
impl<T: VectorElement> Zero for Vector3<T> {
    fn zero() -> Self {
        (T::zero(), T::zero(), T::zero()).into()
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

/// Make a Vector3 { x: 1, y: 1, z: 1 }.
impl<T: VectorElement> One for Vector3<T> {
    fn one() -> Self {
        (T::one(), T::one(), T::one()).into()
    }
}

//============================================================
// prelude
//============================================================
impl<T: VectorElement> Vector3<T> {
    /// Make a Vector3 from x and y, z
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3::<T> { x, y, z }
    }

    /// Make a unit vector.
    pub fn unit_x() -> Self {
        Vector3::new(T::one(), T::zero(), T::zero())
    }

    /// ```
    /// use pyrite::math::Vector3;
    /// let v = Vector3::unit_y();
    /// assert_eq!(v, Vector3::new(0, 1, 0));
    /// ```
    pub fn unit_y() -> Self {
        Vector3::new(T::zero(), T::one(), T::zero())
    }

    /// ```
    /// use pyrite::math::Vector3;
    /// let v = Vector3::unit_z();
    /// assert_eq!(v, Vector3::new(0, 0, 1));
    /// ```
    pub fn unit_z() -> Self {
        Vector3::new(T::zero(), T::zero(), T::one())
    }
}

impl<T: VectorElement> LengthSquared for Vector3<T> {
    /// Return the square of the length.
    fn length_squared(&self) -> T {
        pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2)
    }
}

impl<T: VectorElement> Dot for Vector3<T> {
    /// Dot product.
    fn dot(&self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: VectorElement> Cross for Vector3<T> {
    type Output = Self;

    /// Cross product.
    fn cross(&self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T> Reflect for Vector3<T>
where
    T: VectorElement + std::ops::Mul<Self, Output = Self>,
    Vector3<T>: Vector<ElementType = T>
        + Dot
        + std::ops::Mul<T, Output = Self>
        + std::ops::Sub<Self, Output = Self>
        + std::ops::Mul<Self, Output = Self>,
{
    fn reflect(&self, normal: Self) -> Self {
        let two: T = T::one() + T::one();
        *self - two * self.dot(normal) * normal
    }
}

//============================================================
// from
//============================================================
impl<T: VectorElement> From<[T; 3]> for Vector3<T> {
    fn from(v: [T; 3]) -> Vector3<T> {
        Vector3::<T> {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

impl<T: VectorElement> From<(T, T, T)> for Vector3<T> {
    fn from(v: (T, T, T)) -> Vector3<T> {
        Vector3::<T> {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}

//============================================================
// fmt
//============================================================
impl<T: std::fmt::Display + VectorElement> std::fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

//============================================================
// test
//============================================================
#[cfg(test)]
mod test {
    use super::super::prelude::*;
    use super::super::VectorElement;
    use super::Vector3;
    use num::Zero;

    #[test]
    fn new() {
        assert_eq!(Vector3::new(1, 2, 3), Vector3 { x: 1, y: 2, z: 3 });
    }

    #[test]
    fn length_squared() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(
            v.length_squared(),
            ::num::pow(1.0, 2) + ::num::pow(2.0, 2) + ::num::pow(3.0, 2)
        );
    }

    #[test]
    fn length() {
        use num::Float;
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length_squared().sqrt(), v.length());
    }

    #[test]
    fn from_tuple() {
        let t = (1, 2, 3);
        let v: Vector3<_> = t.into();
        assert_eq!(t.0, v.x);
        assert_eq!(t.1, v.y);
        assert_eq!(t.2, v.z);
    }

    #[test]
    fn from_array() {
        let a = [1, 2, 3];
        let v: Vector3<_> = a.into();
        assert_eq!(a[0], v.x);
        assert_eq!(a[1], v.y);
        assert_eq!(a[2], v.z);
    }

    #[test]
    fn iter() {
        let v = Vector3::new(1, 2, 3);
        for (i, e) in v.iter().enumerate() {
            assert_eq!(i + 1, e);
        }
    }

    #[test]
    fn normalized() {
        use proptest::*;

        // zero vector test
        assert_eq!(Vector3::<f32>::zero().normalized(), Vector3::zero());

        fn test<T: VectorElement + std::fmt::Debug + ::num::Float>(x: T, y: T, z: T) {
            let v = Vector3::new(x, y, z);
            let len = v.length();
            assert_eq!(v.normalized(), v / len);
        }

        // -100.0 <= e < 0.0
        proptest! (|(x in -100.0..0.0, y in -100.0..0.0, z in -100.0..0.0)| {
            test(x, y, z);
        });
        // 1.0 <= e < 100.0
        proptest! (|(x in 1.0..100.0, y in 1.0..100.0, z in 1.0..100.0)| {
            test(x, y, z);
        });
    }

    #[test]
    fn reflect() {
        for x in (1..=180).map(|v| v as f64) {
            let (sin, cos) = x.to_radians().sin_cos();
            let v = Vector3::new(cos, -sin, 0.0);
            let n = Vector3::unit_y();
            let reflect_vector = v.reflect(n);
            assert_eq!(reflect_vector, Vector3::new(cos, sin, 0.0));
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        use proptest::*;

        proptest! (|(x in -100..100, y in -100..100, z in -100..100)| {
            let a = Vector3 { x, y, z };
            let json = serde_json::to_string(&a).unwrap();
            let b = serde_json::from_str(&json).unwrap();
            assert_eq!(a, b);
        });
    }
}

#[cfg(test)]
mod test_ops {
    use super::Vector3;
    use num::Zero;

    #[test]
    fn mul_scalar() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let a = v * 3.2;
        let b = 3.2 * v;
        assert_eq!(a, b);
    }

    #[test]
    fn index() {
        let v = Vector3::new(1, 2, 3);
        assert_eq!(v.x, v[0]);
        assert_eq!(v.y, v[1]);
        assert_eq!(v.z, v[2]);
    }

    #[test]
    #[should_panic]
    fn index_out_of_range() {
        let v = Vector3::new(1, 2, 3);
        let _ = v[3];
    }

    #[test]
    fn index_mut() {
        let mut v = Vector3::zero();
        v[0] = 1;
        v[1] = 42;
        v[2] = 55;
        assert_eq!(1, v[0]);
        assert_eq!(42, v[1]);
        assert_eq!(55, v[2]);
    }

    #[test]
    #[should_panic]
    fn index_mut_out_of_range() {
        let mut v = Vector3::zero();
        v[3] = 1;
    }
}

#[cfg(test)]
mod test_ops_prop {
    use super::Vector3;

    mod ops {
        use super::Vector3;
        use proptest::*;
        ops_test!(Vector3; x, y, z);
    }
}
