use super::Vector;
use num::{Float, Zero};
use std::ops::{self, Index, IndexMut};

//============================================================
// iter
//============================================================
pub trait IndexAccessableVector:
    Vector + Index<usize> + IndexMut<usize, Output = <Self as Vector>::ElementType>
{
}

pub struct VectorIterator<'a, V: IndexAccessableVector> {
    index: usize,
    v: &'a V,
}

pub trait IteratableVector: IndexAccessableVector
where
    Self: Sized,
{
    fn iter(&self) -> VectorIterator<Self>;
}

//============================================================
// construct traits
//============================================================
pub trait FromScalar: Vector {
    fn from_scalar(scalar: Self::ElementType) -> Self;
}

//============================================================
// function traits
//============================================================
pub trait LengthSquared: Vector {
    fn length_squared(&self) -> Self::ElementType;
}

pub trait Length: Vector {
    type Output;

    fn length(&self) -> Self::Output;
}

pub trait Dot: Vector {
    fn dot(&self, rhs: Self) -> Self::ElementType;
}

pub trait Cross: Vector {
    type Output;

    fn cross(&self, rhs: Self) -> Self::Output;
}

pub trait Distance: Vector {
    type Output;

    fn distance(&self, rhs: Self) -> Self::Output;
}

pub trait Normalized: Vector {
    fn normalized(&self) -> Self;
}

pub trait Normalize: Normalized + Sized {
    fn normlaize(&mut self) -> &mut Self {
        *self = self.normalized();
        self
    }
}

pub trait Reflect: Vector {
    fn reflect(&self, normal: Self) -> Self;
}

//============================================================
// impl iter traits
//============================================================
impl<V> IndexAccessableVector for V where
    V: Vector + Index<usize, Output = <V as Vector>::ElementType> + IndexMut<usize>
{
}

impl<V: IndexAccessableVector> Iterator for VectorIterator<'_, V> {
    type Item = V::ElementType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < V::DIMENSION {
            let result = self.v[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl<V: IndexAccessableVector> IteratableVector for V {
    fn iter(&self) -> VectorIterator<Self> {
        VectorIterator { index: 0, v: &self }
    }
}

//============================================================
// impl function traits
//============================================================
/// Return the length of vector.
impl<V> Length for V
where
    V: Vector + LengthSquared,
    V::ElementType: Float,
{
    type Output = Self::ElementType;

    fn length(&self) -> Self::Output {
        self.length_squared().sqrt()
    }
}

impl<V: Vector> Distance for V
where
    V: ops::Sub<V, Output = V> + Length<Output = <V as Vector>::ElementType>,
{
    type Output = V::ElementType;

    fn distance(&self, rhs: V) -> Self::Output {
        let v = *self - rhs;
        v.length()
    }
}

impl<V: Vector> Normalized for V
where
    V: ops::Div<Self::ElementType, Output = Self> + FromScalar + LengthSquared,
    V::ElementType: Float + Sized,
{
    fn normalized(&self) -> Self {
        let length_squared = self.length_squared();
        let zero = V::ElementType::zero();
        if length_squared != zero {
            *self / length_squared.sqrt()
        } else {
            Self::from_scalar(zero)
        }
    }
}

impl<V: Normalized> Normalize for V {}

//============================================================
// functions
//============================================================
pub fn dot<V: Dot>(a: V, b: V) -> V::ElementType {
    a.dot(b)
}

pub fn distance<V: Distance>(a: V, b: V) -> V::Output {
    a.distance(b)
}
