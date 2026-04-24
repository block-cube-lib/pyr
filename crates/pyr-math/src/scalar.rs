#[cfg(feature = "serde")]
use serde::{Serialize, de::DeserializeOwned};

#[cfg(not(feature = "serde"))]
pub trait Scalar:
    num_traits::Num
    + num_traits::NumAssign
    + num_traits::ConstZero
    + num_traits::ConstOne
    + Copy
    + std::fmt::Debug
    + PartialEq
    + PartialOrd
    + Send
    + Sync
    + 'static
{
}

#[cfg(feature = "serde")]
pub trait Scalar:
    num_traits::Num
    + num_traits::NumAssign
    + Copy
    + std::fmt::Debug
    + PartialEq
    + PartialOrd
    + Send
    + Sync
    + 'static
    + Serialize
    + DeserializeOwned
{
}

#[cfg(not(feature = "serde"))]
impl<T> Scalar for T where
    T: num_traits::Num
        + num_traits::NumAssign
        + num_traits::ConstZero
        + num_traits::ConstOne
        + Copy
        + std::fmt::Debug
        + PartialEq
        + PartialOrd
        + Send
        + Sync
        + 'static
{
}

#[cfg(feature = "serde")]
impl<T> Scalar for T where
    T: num_traits::Num
        + num_traits::NumAssign
        + Copy
        + std::fmt::Debug
        + PartialEq
        + PartialOrd
        + Send
        + Sync
        + 'static
        + Serialize
        + DeserializeOwned
{
}

pub trait ApproxEq<T = Self> {
    type Epsilon;

    fn approx_eq(&self, other: &T) -> bool;

    fn approx_eq_eps(&self, other: &T, epsilon: Self::Epsilon) -> bool;
}

impl ApproxEq for f32 {
    type Epsilon = f32;

    #[inline]
    fn approx_eq(&self, other: &f32) -> bool {
        self.approx_eq_eps(other, 1e-5)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &f32, epsilon: f32) -> bool {
        (*self - *other).abs() <= epsilon
    }
}

impl ApproxEq for f64 {
    type Epsilon = f64;

    #[inline]
    fn approx_eq(&self, other: &f64) -> bool {
        self.approx_eq_eps(other, 1e-10)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &f64, epsilon: f64) -> bool {
        (*self - *other).abs() <= epsilon
    }
}

pub trait Lerp {
    type Factor;

    fn lerp(self, other: Self, t: Self::Factor) -> Self;
}
impl Lerp for f32 {
    type Factor = f32;

    #[inline]
    fn lerp(self, other: Self, t: Self) -> Self {
        self * (1.0 - t) + other * t
    }
}
impl Lerp for f64 {
    type Factor = f64;

    #[inline]
    fn lerp(self, other: Self, t: Self) -> Self {
        self * (1.0 - t) + other * t
    }
}

pub trait FloatScalar:
    Scalar + num_traits::Float + ApproxEq<Epsilon = Self> + Lerp<Factor = Self>
{
}

impl<T> FloatScalar for T where
    T: Scalar + num_traits::Float + ApproxEq<Epsilon = Self> + Lerp<Factor = Self>
{
}
