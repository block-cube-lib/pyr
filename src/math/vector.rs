mod array_wrapper;
mod traits;
mod vector1;
mod vector2;
mod vector3;
mod vector4;

pub use array_wrapper::ArrayWrapper;
pub use traits::{VectorElement, VectorLike};
pub use vector1::Vector1;
pub use vector2::Vector2;
pub use vector3::Vector3;
pub use vector4::Vector4;

#[doc(hidden)]
pub trait VectorTypeHolder<T: VectorElement, const D: usize> {
    type Vector;
}

#[doc(hidden)]
pub struct VectorTypeResolver<T: VectorElement, const D: usize> {
    _marker: std::marker::PhantomData<fn() -> [T; D]>,
}

impl<T: VectorElement> VectorTypeHolder<T, 1> for VectorTypeResolver<T, 1> {
    type Vector = Vector1<T>;
}
impl<T: VectorElement> VectorTypeHolder<T, 2> for VectorTypeResolver<T, 2> {
    type Vector = Vector2<T>;
}
impl<T: VectorElement> VectorTypeHolder<T, 3> for VectorTypeResolver<T, 3> {
    type Vector = Vector3<T>;
}
impl<T: VectorElement> VectorTypeHolder<T, 4> for VectorTypeResolver<T, 4> {
    type Vector = Vector4<T>;
}

seq_macro::seq!(N in 5..32 {
    impl<T: VectorElement> VectorTypeHolder<T, N> for VectorTypeResolver<T, N> {
        type Vector = ArrayWrapper<T, N>;
    }
});

/// Vector type. T: Type of the element. D: Dimension. D must be in the range of 1 to 32.
pub type Vector<T, const D: usize> = <VectorTypeResolver<T, D> as VectorTypeHolder<T, D>>::Vector;

/*
pub(crate) mod macros;
mod traits;
mod vector1;
mod vector2;

use crate::{impl_vector, impl_vector_ops};
use num::{Float, One, Zero};
use seq_macro::seq;
use std::marker::PhantomData;

pub use std::fmt::{self, Display, Formatter};
pub use traits::*;
pub use vector1::Vector1;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3<T: VectorElement> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl_vector_ops!(Vector3, 1, x, 0, y, 1, z, 2);
impl_vector!(Vector3, 1, x, 0, y, 1, z, 2);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector4<T: VectorElement> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
impl<T: VectorElement + Eq> Eq for Vector4<T> {}
impl<T: VectorElement> VectorLike<T, 4> for Vector4<T> {
    fn get(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.z,
            _ => panic!("Out of range vector elements index. index = {index}."),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrayWrapper<T: VectorElement, const DIMENSION: usize> {
    elements: [T; DIMENSION],
}
impl<T, const DIMENSION: usize> Eq for ArrayWrapper<T, DIMENSION> where T: VectorElement + Eq {}
impl<T, const DIMENSION: usize> Default for ArrayWrapper<T, DIMENSION>
where
    T: VectorElement,
{
    fn default() -> Self {
        Self {
            elements: [T::default(); DIMENSION],
        }
    }
}
impl<T: VectorElement, const DIMENSION: usize> VectorLike<T, DIMENSION>
    for ArrayWrapper<T, DIMENSION>
{
    fn get(&self, index: usize) -> &T {
        &self.elements[index]
    }
}

trait VectorTypeHolder<T, const D: usize> {
    type Vector;
}
struct VectorTypeResolver<T, const D: usize> {
    _marker: PhantomData<fn() -> [T; D]>,
}

impl<T> VectorTypeHolder<T, 1> for VectorTypeResolver<T, 2> {
    type Vector = Vector2<T>;
}
impl<T> VectorTypeHolder<T, 2> for VectorTypeResolver<T, 2> {
    type Vector = Vector2<T>;
}
impl<T> VectorTypeHolder<T, 3> for VectorTypeResolver<T, 3> {
    type Vector = Vector3<T>;
}
impl<T> VectorTypeHolder<T, 4> for VectorTypeResolver<T, 4> {
    type Vector = Vector4<T>;
}
seq!(N in 5..256 {
    impl<T> VectorTypeHolder<T, N> for VectorTypeResolver<T, N> {
        type Vector = ArrayWrapper<T, N>;
    }
});

pub type Vector<T, const D: usize> = <VectorTypeResolver<T, D> as VectorTypeHolder<T, D>>::Vector;

impl<T, const DIMENSION: usize> VectorLike<T, DIMENSION> for Vector<T, DIMENSION>
where
    T: VectorElement,
{
    fn get(&self, index: usize) -> &T {
        &self.elements[index]
    }
}

impl<T: VectorElement, const DIMENSION: usize> From<[T; DIMENSION]> for Vector<T, DIMENSION> {
    fn from(v: [T; DIMENSION]) -> Self {
        Self { elements: v }
    }
}

//include!("./vector/xyzw_accessor.rs");

impl<T: VectorElement> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }

    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }

    pub fn unit_z() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }

    pub fn corss(&self, other: impl VectorLike<T, 3>) -> Self {
        Self {
            x: *self.get(1) * *other.get(2) - *self.get(2) * *other.get(1),
            y: *self.get(2) * *other.get(1) - *self.get(1) * *other.get(2),
            z: *self.get(0) * *other.get(1) - *self.get(1) * *other.get(0),
        }
    }

    pub fn reflect(&self, normal: impl VectorLike<T, 3>) -> Self {
        let normal = normal.to_vector();
        let two = T::one() + T::one();
        *self - normal * two * self.dot(normal)
    }
}

impl<T: VectorElement> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero(), T::zero(), T::zero())
    }

    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one(), T::zero(), T::zero())
    }

    pub fn unit_z() -> Self {
        Self::new(T::zero(), T::zero(), T::one(), T::zero())
    }

    pub fn unit_w() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::one())
    }
}

impl<T: VectorElement, const DIMENSION: usize> Zero for Vector<T, DIMENSION> {
    fn zero() -> Self {
        Self {
            elements: [T::zero(); DIMENSION],
        }
    }

    fn is_zero(&self) -> bool {
        self.elements.iter().all(|e| e.is_zero())
    }
}

//include!("./vector/ops.rs");

impl<T: VectorElement, const DIMENSION: usize> One for Vector<T, DIMENSION> {
    fn one() -> Self {
        Self {
            elements: [T::one(); DIMENSION],
        }
    }
}

/*
impl<V, T: VectorElement, const DIMENSION: usize> V
where
    V: VectorLike<T, DIMENSION>,
{
    pub fn length_squared(&self) -> T {
        let mut result = T::zero();
        for i in 0..DIMENSION {
            result += self.elements[i] * self.elements[i];
        }
        result
    }

    pub fn dot(&self, rhs: impl VectorLike<T, DIMENSION>) -> T {
        let mut result = T::zero();
        for i in 0..DIMENSION {
            result += *self.get(i) * *rhs.get(i);
        }
        result
    }
}

impl<T, const DIMENSION: usize> Vector<T, DIMENSION>
where
    T: VectorElement + Float,
{
    pub fn length(&self) -> T {
        let length_squared = self.length_squared();
        if length_squared.is_zero() {
            T::zero()
        } else {
            length_squared.sqrt()
        }
    }

    pub fn distance(&self, other: impl VectorLike<T, DIMENSION>) -> T {
        let v = *self - other.to_vector();
        v.length()
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len.is_zero() {
            Self::zero()
        } else {
            *self / len
        }
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}
*/

pub trait ToVector<T: VectorElement, const DIMENSION: usize> {
    fn to_vector(self) -> Vector<T, DIMENSION>;
}

impl<V, T, const DIMENSION: usize> ToVector<T, DIMENSION> for V
where
    V: VectorLike<T, DIMENSION>,
    T: VectorElement,
{
    fn to_vector(self) -> Vector<T, DIMENSION> {
        let mut elements = [T::default(); DIMENSION];
        for (i, element) in elements.iter_mut().enumerate() {
            *element = *self.get(i);
        }
        Vector::<T, DIMENSION> { elements }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_vec1() {
        let v = Vector1::new(0);
        assert_eq!(v.elements[0], 0);
    }

    #[test]
    fn new_vec2() {
        let v = Vector2::new(0, 1);
        assert_eq!(v.elements[0], 0);
        assert_eq!(v.elements[1], 1);
    }

    #[test]
    fn new_vec3() {
        let v = Vector3::new(0, 1, 2);
        assert_eq!(v.elements[0], 0);
        assert_eq!(v.elements[1], 1);
        assert_eq!(v.elements[2], 2);
    }

    #[test]
    fn new_vec4() {
        let v = Vector4::new(0, 1, 2, 3);
        assert_eq!(v.elements[0], 0);
        assert_eq!(v.elements[1], 1);
        assert_eq!(v.elements[2], 2);
        assert_eq!(v.elements[3], 3);
    }

    #[test]
    fn to_vector() {
        let v = [0, 1, 2].to_vector();
        assert_eq!(v.elements[0], 0);
        assert_eq!(v.elements[1], 1);
        assert_eq!(v.elements[2], 2);

        let v = [1, 3, 5, 7, 9].to_vector();
        assert_eq!(v.elements[0], 1);
        assert_eq!(v.elements[1], 3);
        assert_eq!(v.elements[2], 5);
        assert_eq!(v.elements[3], 7);
        assert_eq!(v.elements[4], 9);
    }

    #[test]
    fn from_array() {
        let v = Vector::from([0, 1, 2]);
        assert_eq!(v.elements[0], 0);
        assert_eq!(v.elements[1], 1);
        assert_eq!(v.elements[2], 2);
    }

    #[test]
    fn array_into_vector() {
        let v: Vector3<_> = [0, 1, 2].into();
        assert_eq!(v.elements[0], 0);
        assert_eq!(v.elements[1], 1);
        assert_eq!(v.elements[2], 2);
    }

    #[test]
    fn length_squared() {
        let v = Vector2::new(2, 3);
        assert_eq!(v.length_squared(), 2 * 2 + 3 * 3);

        let v = Vector3::new(1, 2, 3);
        assert_eq!(v.length_squared(), 1 * 1 + 2 * 2 + 3 * 3);
    }

    #[test]
    fn length() {
        let v = Vector2::new(2.0, 3.0);
        assert_eq!(v.length(), (2.0 * 2.0 + 3.0 * 3.0).sqrt());

        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length(), (1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0).sqrt());
    }

    #[test]
    fn dot() {
        let v1 = Vector2::new(2, 3);
        let v2 = Vector2::new(3, 9);
        assert_eq!(v1.dot(v2), 33);

        let v1 = Vector3::new(2, 3, 4);
        let v2 = Vector3::new(3, 9, 10);
        assert_eq!(v1.dot(v2), 73);
    }

    #[test]
    fn reflect() {
        let l = Vector3::new(1.0, -1.0, 0.0);
        let n = Vector3::unit_y();
        assert_eq!(l.reflect(n), Vector3::new(1.0, 1.0, 0.0));

        let l = Vector3::new(2.0, -1.0, 4.0);
        assert_eq!(l.reflect(n), Vector3::new(2.0, 1.0, 4.0));

        let l = Vector3::new(1.0, -1.0, 5.0);
        let n = Vector3::unit_x();
        assert_eq!(l.reflect(n), Vector3::new(-1.0, -1.0, 5.0_f64));
    }
}
*/
