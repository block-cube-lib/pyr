use super::traits::{VectorElement, VectorLike};
use paste::paste;
use seq_macro::seq;
use std::ops::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VecWrapper<T: VectorElement, const DIMENSION: usize> {
    pub elements: [T; DIMENSION],
}

impl<T: VectorElement, const N: usize> VecWrapper<T, N> {
    pub const ZERO: Self = Self {
        elements: [T::ZERO; N],
    };
    pub const ONE: Self = Self {
        elements: [T::ONE; N],
    };
}

impl<T: Eq + VectorElement, const N: usize> Eq for VecWrapper<T, N> {}

impl<T: VectorElement, const DIMENSTION: usize> VectorLike<DIMENSTION>
    for VecWrapper<T, DIMENSTION>
{
    type ElementType = T;

    fn get(&self, index: usize) -> T {
        self.elements[index]
    }

    fn set(&mut self, index: usize, value: T) {
        self.elements[index] = value;
    }

    fn from_array(array: [T; DIMENSTION]) -> Self {
        Self { elements: array }
    }
    fn into_array(self) -> [T; DIMENSTION] {
        self.elements
    }

    fn into_float_array(self) -> [T::FloatCalcType; DIMENSTION] {
        use crate::num::Zero;
        let mut result = [T::FloatCalcType::ZERO; DIMENSTION];
        for i in 0..DIMENSTION {
            result[i] = self.elements[i].as_float_type();
        }
        result
    }
}

impl<T: VectorElement, const N: usize> std::convert::From<[T; N]> for VecWrapper<T, N> {
    fn from(value: [T; N]) -> Self {
        Self { elements: value }
    }
}

impl<T, const N: usize> Index<usize> for VecWrapper<T, N>
where
    T: VectorElement,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for VecWrapper<T, N>
where
    T: VectorElement,
{
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.elements[index]
    }
}

macro_rules! impl_from_tuple {
    ($dim: expr) => {
        seq!(N in 0..$dim {
            impl<T: VectorElement> std::convert::From<(#(T,)*)> for VecWrapper<T, $dim> {
                fn from(value: (#(T,)*)) -> Self {
                    Self { elements: [#(value.N,)*] }
                }
            }
        });
    }
}

macro_rules! impl_new {
    ($dim: expr) => {
        impl<T: VectorElement> VecWrapper<T, $dim> {
            seq!(n in 0..$dim {
                paste! {
                    #[allow(clippy::too_many_arguments)]
                    pub const fn new( #( [<element_ n>]: T,)*) -> Self {
                        Self {
                            elements: [ #( [<element_ n>],)* ],
                        }
                    }
                }
            });
        }
    }
}

seq!(N in 1..=32 {
    impl_from_tuple!(N);
    impl_new!(N);
});

impl<T: VectorElement, const N: usize> VecWrapper<T, N> {
    pub fn length_squared(&self) -> T {
        let mut result = T::ZERO;
        for v in self.elements {
            result += v * v;
        }
        result
    }

    pub fn dot(&self, rhs: impl VectorLike<N, ElementType = T>) -> T {
        let mut result = T::ZERO;
        for i in 0..N {
            result += self.elements[i] * rhs.get(i);
        }
        result
    }
}

impl<T: VectorElement + num::Float, const N: usize> VecWrapper<T, N> {
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len != T::ZERO {
            let one_over_len = T::ONE / self.length();
            let mut elements = [one_over_len; N];
            for i in 0..self.elements.len() {
                elements[i] *= self.elements[i];
            }
            Self { elements }
        } else {
            Self::ZERO
        }
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

macro_rules! impl_ops_vector_trait_for_vec_wrapper {
    ($trait_name: ident, $func_name: ident, $op: tt) => {
        impl<T: VectorElement, const N: usize> $trait_name<Self> for VecWrapper<T, N> {
            type Output = Self;

            fn $func_name(self, rhs: Self) -> Self::Output {
                let mut elements = [T::ZERO; N];
                for i in 0..N {
                    elements[i] = self.elements[i] $op rhs.elements[i];
                }
                Self {
                    elements
                }
            }
        }

        paste! {
            impl<T: VectorElement, const N: usize> [<$trait_name Assign>]<Self> for VecWrapper<T, N>
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
        impl<T: VectorElement, const N: usize> $trait_name<T> for VecWrapper<T, N> {
            type Output = Self;

            fn $func_name(self, scalar: T) -> Self::Output {
                let mut elements = [T::ZERO; N];
                for i in 0..N {
                    elements[i] = self.elements[i] $op scalar;
                }
                Self {
                    elements
                }
            }
        }

        paste! {
            impl<T: VectorElement, const N: usize> [<$trait_name Assign>]<T> for VecWrapper<T, N>
            {
                fn [<$func_name _assign>](&mut self, scalar: T) {
                    *self = *self $op scalar;
                }
            }
        }
    };
}

impl_ops_scalar_trait_for_vec_wrapper!(Mul, mul, *);
impl_ops_scalar_trait_for_vec_wrapper!(Div, div, /);

impl<T: VectorElement + Neg<Output = T>, const N: usize> Neg for VecWrapper<T, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut elements = [T::ZERO; N];
        for i in 0..N {
            elements[i] = -self.elements[i];
        }
        Self { elements }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        const V: VecWrapper<i32, 5> = VecWrapper::<i32, 5>::new(1, 2, 3, 4, 5);
        assert_eq!(V.elements[0], 1);
        assert_eq!(V.elements[1], 2);
        assert_eq!(V.elements[2], 3);
        assert_eq!(V.elements[3], 4);
        assert_eq!(V.elements[4], 5);
    }

    #[test]
    fn vector_like_get() {
        let v = VecWrapper::<i32, 5>::new(1, 2, 3, 4, 5);
        assert_eq!(v.get(0), 1);
        assert_eq!(v.get(1), 2);
        assert_eq!(v.get(2), 3);
        assert_eq!(v.get(3), 4);
        assert_eq!(v.get(4), 5);
    }

    #[test]
    #[should_panic]
    fn vector_like_get_out_of_range() {
        let v = VecWrapper::<i32, 5>::ZERO;
        let _ = v.get(5);
    }

    #[test]
    fn vector_like_set() {
        let mut v = VecWrapper::<i32, 5>::ZERO;
        v.set(0, 1);
        v.set(1, 2);
        v.set(2, 3);
        v.set(3, 4);
        v.set(4, 5);
        assert_eq!(v.get(0), 1);
        assert_eq!(v.get(1), 2);
        assert_eq!(v.get(2), 3);
        assert_eq!(v.get(3), 4);
        assert_eq!(v.get(4), 5);
    }

    #[test]
    #[should_panic]
    fn vector_like_set_out_of_range() {
        let mut v = VecWrapper::<i32, 5>::ZERO;
        v.set(5, 0);
    }

    #[test]
    fn from_array() {
        let v = VecWrapper::<i32, 5>::from([1, 2, 3, 4, 5]);
        assert_eq!(v.elements, [1, 2, 3, 4, 5])
    }

    #[test]
    fn from_tuple() {
        let v = VecWrapper::<i32, 5>::from((1, 2, 3, 4, 5));
        assert_eq!(v.elements, [1, 2, 3, 4, 5])
    }

    #[test]
    fn length_squared() {
        let v = VecWrapper::<i32, 5>::new(1, 2, 3, 4, 5);
        assert_eq!(v.length_squared(), 1 * 1 + 2 * 2 + 3 * 3 + 4 * 4 + 5 * 5);
    }

    #[test]
    fn length() {
        let v = VecWrapper::<f32, 5>::new(1.0, 2.0, 3.0, 4.0, 5.0);
        assert_eq!(
            v.length(),
            (1.0_f32 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0).sqrt()
        );
    }

    #[test]
    fn normalized() {
        let v = VecWrapper::<f32, 5>::new(1.0, 2.0, 3.0, 4.0, 5.0);
        let n = v.normalized();
        let one_over_len = 1.0 / v.length();
        assert_ne!((n.length_squared() - 1.0).abs(), 0.00001);
        assert_ne!((v[0] - 1.0 * one_over_len).abs(), 0.00001);
        assert_ne!((v[1] - 2.0 * one_over_len).abs(), 0.00001);
        assert_ne!((v[2] - 3.0 * one_over_len).abs(), 0.00001);
        assert_ne!((v[3] - 4.0 * one_over_len).abs(), 0.00001);
        assert_ne!((v[4] - 5.0 * one_over_len).abs(), 0.00001);
    }

    #[test]
    fn dot() {
        let v1 = VecWrapper::<i32, 5>::new(1, 2, 3, 4, 5);
        let d = v1.dot([11, 12, 13, 14, 15]);
        assert_eq!(d, 1 * 11 + 2 * 12 + 3 * 13 + 4 * 14 + 5 * 15);
    }
}
