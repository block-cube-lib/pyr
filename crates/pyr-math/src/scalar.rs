use crate::{ApproxEq, Lerp};

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

pub trait FloatScalar:
    Scalar + num_traits::Float + ApproxEq<Epsilon = Self> + Lerp<Factor = Self>
{
}

impl<T> FloatScalar for T where
    T: Scalar + num_traits::Float + ApproxEq<Epsilon = Self> + Lerp<Factor = Self>
{
}
