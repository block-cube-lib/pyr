use super::traits::*;
use crate::math::ops;
use crate::num::{AsFloatingPoint, One, Zero};
use paste::paste;
use seq_macro::seq;
use std::ops::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrayWrapper<T: VectorElement, const N: usize> {
    inner: [T; N],
}

impl<T: VectorElement + Eq, const N: usize> Eq for ArrayWrapper<T, N> {}

impl<T: VectorElement, const N: usize> One for ArrayWrapper<T, N> {
    const ONE: Self = Self { inner: [T::ONE; N] };
}

impl<T: VectorElement, const N: usize> Zero for ArrayWrapper<T, N> {
    const ZERO: Self = Self {
        inner: [T::ZERO; N],
    };

    fn is_near_zero(&self) -> bool {
        self.inner.iter().all(|&x| x.is_near_zero())
    }
}

impl<T: VectorElement, const N: usize> Into<[T; N]> for ArrayWrapper<T, N> {
    fn into(self) -> [T; N] {
        self.inner
    }
}

impl<T, const N: usize> VectorLike<N> for ArrayWrapper<T, N>
where
    T: VectorElement,
{
    type ElementType = T;

    fn get(&self, index: usize) -> &Self::ElementType {
        self.inner.get(index)
    }

    fn get_mut(&mut self, index: usize) -> &mut Self::ElementType {
        self.inner.get_mut(index)
    }
}

impl<T: VectorElement, const N: usize> std::convert::From<[T; N]> for ArrayWrapper<T, N>
where
    [T; N]: VectorLike<N>,
{
    fn from(array: [T; N]) -> ArrayWrapper<T, N> {
        ArrayWrapper { inner: array }
    }
}

macro_rules! impl_from_tuple {
    ($dim: expr) => {
        seq!(N in 0..$dim {
            impl<T: VectorElement> std::convert::From<(#(T,)*)> for ArrayWrapper<T, $dim> {
                fn from(value: (#(T,)*)) -> Self {
                    Self { inner: value.into() }
                }
            }
        });
    }
}

macro_rules! impl_new {
    ($dim: expr) => {
        impl<T: VectorElement> ArrayWrapper<T, $dim> {
            seq!(n in 0..$dim {
                paste! {
                    #[allow(clippy::too_many_arguments)]
                    pub const fn new( #( [<element_ n>]: T,)*) -> Self {
                        Self {
                            inner: [ #( [<element_ n>],)* ],
                        }
                    }
                }
            });
        }
    }
}

seq!(N in 1..=12 {
    impl_from_tuple!(N);
    impl_new!(N);
});

impl<T: VectorElement, const N: usize> ArrayWrapper<T, N> {
    pub fn length_squared(&self) -> T {
        ops::length_squared(self.inner)
    }

    pub fn dot(&self, rhs: impl VectorLike<N, ElementType = T>) -> T {
        ops::dot(self.inner, rhs)
    }

    pub fn length(&self) -> <T as AsFloatingPoint>::Output
    where
        <T as AsFloatingPoint>::Output: FloatVectorElement,
    {
        ops::length(self.inner)
    }

    pub fn normalized(&self) -> ArrayWrapper<<T as AsFloatingPoint>::Output, N>
    where
        <T as AsFloatingPoint>::Output: FloatVectorElement,
    {
        let a = ops::normalized(self.inner);
        a
    }
}

impl<T: FloatVectorElement, const N: usize> ArrayWrapper<T, N> {
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

macro_rules! impl_ops_vector_trait_for_vec_wrapper {
    ($trait_name: ident, $func_name: ident, $op: tt) => {
        impl<T: VectorElement, const N: usize> $trait_name<Self> for ArrayWrapper<T, N> {
            type Output = Self;

            fn $func_name(self, rhs: Self) -> Self::Output {
                let mut elements = [T::ZERO; N];
                for i in 0..N {
                    elements[i] = self.inner[i] $op rhs.inner[i];
                }
                Self {
                    inner: elements
                }
            }
        }

        paste! {
            impl<T: VectorElement, const N: usize> [<$trait_name Assign>]<Self> for ArrayWrapper<T, N>
            {
                fn [<$func_name _assign>](&mut self, rhs: Self) {
                    *self = *self $op rhs;
                }
            }
        }
    };
}

impl_ops_vector_trait_for_vec_wrapper!(Add, add, +);
impl_ops_vector_trait_for_vec_wrapper!(Sub, sub, -);
impl_ops_vector_trait_for_vec_wrapper!(Mul, mul, *);
impl_ops_vector_trait_for_vec_wrapper!(Div, div, /);

macro_rules! impl_ops_scalar_trait_for_vec_wrapper {
    ($trait_name: ident, $func_name: ident, $op: tt) => {
        impl<T: VectorElement, const N: usize> $trait_name<T> for ArrayWrapper<T, N> {
            type Output = Self;

            fn $func_name(self, scalar: T) -> Self::Output {
                let mut elements = [T::ZERO; N];
                for i in 0..N {
                    elements[i] = self.inner[i] $op scalar;
                }
                Self {
                    inner: elements
                }
            }
        }

        paste! {
            impl<T: VectorElement, const N: usize> [<$trait_name Assign>]<T> for ArrayWrapper<T, N>
            {
                #[inline]
                fn [<$func_name _assign>](&mut self, scalar: T) {
                    for i in 0..N {
                        self.inner[i] = self.inner[i] $op scalar;
                    }
                }
            }
        }
    };
}

impl_ops_scalar_trait_for_vec_wrapper!(Mul, mul, *);
impl_ops_scalar_trait_for_vec_wrapper!(Div, div, /);

/// ```
/// use pyr::math::ArrayWrapper;
/// let v = ArrayWrapper::<i32, 2>::new(1, 2);
/// let v = -v;
/// assert_eq!(v[0], -1);
/// assert_eq!(v[1], -2);
/// ```
impl<T: VectorElement + Neg<Output = T>, const N: usize> Neg for ArrayWrapper<T, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut elements = [T::ZERO; N];
        for i in 0..N {
            elements[i] = -self.inner[i];
        }
        Self { inner: elements }
    }
}

/// ```
/// use pyr::math::ArrayWrapper;
/// let v = ArrayWrapper::<i32, 2>::new(1, 2);
/// assert_eq!(v[0], 1);
/// assert_eq!(v[1], 2);
/// ```
impl<T: VectorElement + Neg<Output = T>, const N: usize> Index<usize> for ArrayWrapper<T, N> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

/// ```
/// use pyr::math::ArrayWrapper;
/// let mut v = ArrayWrapper::<i32, 2>::new(1, 2);
/// v[0] = 3;
/// assert_eq!(v[0], 3);
/// ```
impl<T: VectorElement + Neg<Output = T>, const N: usize> IndexMut<usize> for ArrayWrapper<T, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        const V: ArrayWrapper<i32, 5> = ArrayWrapper::<i32, 5>::new(1, 2, 3, 4, 5);
        assert_eq!(V.inner[0], 1);
        assert_eq!(V.inner[1], 2);
        assert_eq!(V.inner[2], 3);
        assert_eq!(V.inner[3], 4);
        assert_eq!(V.inner[4], 5);
    }

    #[test]
    fn vector_like_get() {
        let v: ArrayWrapper<i32, 5> = ArrayWrapper::<i32, 5>::new(1, 2, 3, 4, 5);
        assert_eq!(*v.get(0), 1);
        assert_eq!(*v.get(1), 2);
        assert_eq!(*v.get(2), 3);
        assert_eq!(*v.get(3), 4);
        assert_eq!(*v.get(4), 5);
    }

    #[test]
    #[should_panic]
    fn vector_like_get_out_of_range() {
        let v = ArrayWrapper::<i32, 5>::ZERO;
        let _ = v.get(5);
    }

    #[test]
    fn vector_like_get_mut() {
        let mut v = ArrayWrapper::<i32, 5>::ZERO;
        *v.get_mut(0) = 1;
        *v.get_mut(1) = 2;
        *v.get_mut(2) = 3;
        *v.get_mut(3) = 4;
        *v.get_mut(4) = 5;
        assert_eq!(*v.get(0), 1);
        assert_eq!(*v.get(1), 2);
        assert_eq!(*v.get(2), 3);
        assert_eq!(*v.get(3), 4);
        assert_eq!(*v.get(4), 5);
    }

    #[test]
    #[should_panic]
    fn vector_like_get_mut_out_of_range() {
        let mut v = ArrayWrapper::<i32, 5>::ZERO;
        *v.get_mut(5) = 0;
    }

    #[test]
    fn from_array() {
        let v = ArrayWrapper::<i32, 5>::from([1, 2, 3, 4, 5]);
        assert_eq!(v.inner, [1, 2, 3, 4, 5])
    }

    #[test]
    fn from_tuple() {
        let v = ArrayWrapper::<i32, 5>::from((1, 2, 3, 4, 5));
        assert_eq!(v.inner, [1, 2, 3, 4, 5])
    }

    #[test]
    fn length_squared() {
        let v = ArrayWrapper::<i32, 5>::new(1, 2, 3, 4, 5);
        assert_eq!(v.length_squared(), 1 * 1 + 2 * 2 + 3 * 3 + 4 * 4 + 5 * 5);
    }

    #[test]
    fn length() {
        let v = ArrayWrapper::<f32, 5>::new(1.0, 2.0, 3.0, 4.0, 5.0);
        assert_eq!(
            v.length(),
            (1.0_f32 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt()
        );
    }

    #[test]
    fn normalized() {
        let v = ArrayWrapper::<i32, 5>::new(1, 2, 3, 4, 5);
        let n = v.normalized();
        let one_over_len = 1.0 / v.length();
        assert!((n.length() - 1.0).is_near_zero(), "{}", n.length_squared());
        assert!(
            (n[0] - 1.0 * one_over_len).is_near_zero(),
            "{}",
            n[0] - 1.0 * one_over_len
        );
        assert!(
            (n[1] - 2.0 * one_over_len).is_near_zero(),
            "{}",
            n[1] - 2.0 * one_over_len
        );
        assert!(
            (n[2] - 3.0 * one_over_len).is_near_zero(),
            "{}",
            n[2] - 3.0 * one_over_len
        );
        assert!(
            (n[3] - 4.0 * one_over_len).is_near_zero(),
            "{}",
            n[3] - 4.0 * one_over_len
        );
        assert!(
            (n[4] - 5.0 * one_over_len).is_near_zero(),
            "{}",
            n[4] - 5.0 * one_over_len
        );
    }

    #[test]
    fn dot() {
        let v1 = ArrayWrapper::<i32, 5>::new(1, 2, 3, 4, 5);
        let d = v1.dot([11, 12, 13, 14, 15]);
        assert_eq!(d, 1 * 11 + 2 * 12 + 3 * 13 + 4 * 14 + 5 * 15);
    }

    #[test]
    fn mul_vector_scalar() {
        let v = ArrayWrapper::<i32, 5>::new(1, 2, 3, 4, 5);
        let v = v * 2;
        assert_eq!(v.inner, [2, 4, 6, 8, 10]);
    }

    //#[test]
    //fn mul_scalar_vector() {
    //    let v = ArrayWrapper::<i32, 5>::new(1, 2, 3, 4, 5);
    //    let v = 2 * v;
    //    assert_eq!(v.inner, [2, 4, 6, 8, 10]);
    //}
}
