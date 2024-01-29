#[macro_export]
macro_rules! impl_vector_display_helper {
    ($element: ident, $($tail: ident),+) => {
        write!(f, "{}, ", self.$element)?;
        impl_vector_display!($($tail),+);
    };
    ($element: ident) => {
        write!(f, "{}", self.$element)?;
    };
}

#[macro_export]
macro_rules! impl_vector_display {
    ($type_name: tt, $($element: ident),+) => {
        impl<T: VectorElement> std::fmt::Display for $type_name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "(")?;
                impl_vector_display_helper!($($element),+);
                write!(f, ")")
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vector_ops {
    ($type_name: tt, $dimension: expr, $($element: ident, $index: expr),+) => {
        // Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign,
        impl_vector_ops!($type_name, $dimension, Add, add, +, $($element, $index),+);
        impl_vector_ops!($type_name, $dimension, Sub, sub, -, $($element, $index),+);
        impl_vector_ops!($type_name, $dimension, Mul, mul, *, $($element, $index),+);
        impl_vector_ops!($type_name, $dimension, Div, div, /, $($element, $index),+);

        impl_vector_ops!(vector_scalar; $type_name, Mul, mul, *, $($element, $index),+);
        impl_vector_ops!(vector_scalar; $type_name, Div, div, /, $($element, $index),+);

        impl_vector_ops!(mul_scalar_vector; i8, $type_name);
        impl_vector_ops!(mul_scalar_vector; i16, $type_name);
        impl_vector_ops!(mul_scalar_vector; i32, $type_name);
        impl_vector_ops!(mul_scalar_vector; i64, $type_name);
        impl_vector_ops!(mul_scalar_vector; i128, $type_name);
        impl_vector_ops!(mul_scalar_vector; u8, $type_name);
        impl_vector_ops!(mul_scalar_vector; u16, $type_name);
        impl_vector_ops!(mul_scalar_vector; u32, $type_name);
        impl_vector_ops!(mul_scalar_vector; u64, $type_name);
        impl_vector_ops!(mul_scalar_vector; u128, $type_name);
        impl_vector_ops!(mul_scalar_vector; isize, $type_name);
        impl_vector_ops!(mul_scalar_vector; usize, $type_name);
        impl_vector_ops!(mul_scalar_vector; f32, $type_name);
        impl_vector_ops!(mul_scalar_vector; f64, $type_name);

        impl<T: VectorElement> std::ops::Index<usize> for $type_name<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    $(
                        $index => &self.$element,
                    )+
                    _ => panic!("Out of range. index = {index}"),
                }
            }
        }

        impl<T: VectorElement> std::ops::IndexMut<usize> for $type_name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    $(
                        $index => &mut self.$element,
                    )+
                    _ => panic!("Out of range. index = {index}"),
                }
            }
        }

        impl<T> std::ops::Neg for $type_name<T>
        where
            T: VectorElement + std::ops::Neg<Output = T>,
        {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self {
                    $(
                        $element: -self.$element,
                    )+
                }
            }
        }
    };
    ($type_name: tt, $dimension: expr, $ops_trait_name: tt, $ops_function_name: ident, $op: tt, $($element: ident, $index: expr),+) => {
        impl<T: VectorElement, V> std::ops::$ops_trait_name<V> for $type_name<T>
        where
            V: VectorLike<T, $dimension>,
        {
            type Output = Self;

            fn $ops_function_name(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $element: self.$element $op rhs.get($index),
                    )+
                }
            }
        }

        paste::paste!{
            impl<T: VectorElement, V> std::ops::[<$ops_trait_name Assign>]<V> for $type_name<T>
            where
                V: VectorLike<T, 1>,
            {
                fn [<$ops_function_name _assign>](&mut self, rhs: Self) -> <Self as std::ops::$ops_trait_name>::Output {
                    $(
                        self.$element = self.$element $op rhs.get($index);
                    )+
                }
            }
        }
    };
    (vector_scalar; $type_name: tt, $ops_trait_name: tt, $ops_function_name: ident, $op: tt, $($element: ident, $index: expr),+) => {
        impl<T: VectorElement> std::ops::$ops_trait_name<T> for $type_name<T> {
            type Output = Self;

            fn $ops_function_name(self, scalar: T) -> Self::Output {
                Self {
                    $(
                        $element: self.$element $op scalar,
                    )+
                }
            }
        }

        paste::paste!{
            impl<T: VectorElement> std::ops::[<$ops_trait_name Assign>]<T> for $type_name<T> {
                fn [<$ops_function_name _assign>](&mut self, scalar: T) {
                    $(
                        self.$element *= scalar;
                    )+
                }
            }
        }
    };
    (mul_scalar_vector; $scalar_type: ty, $vector_type: tt) => {
        impl std::ops::Mul<$vector_type<$scalar_type>> for $scalar_type {
            type Output = $vector_type<$scalar_type>;

            fn mul(self, rhs: $vector_type<$scalar_type>) -> Self::Output {
                rhs * self
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vector {
    ($type_name: tt, $dimension: expr, $($element: ident, $index: expr),+) => {
        impl<T: VectorElement> $type_name<T> {
            pub fn new($($element: T),+) -> Self {
                Self { $($element),+ }
            }
        }

        impl<T: VectorElement + Eq> Eq for $type_name<T> {}

        impl<T: VectorElement> VectorLike<T, $dimension> for $type_name<T> {
            fn get(&self, index: usize) -> T {
                match index {
                    $(
                        $index => self.$element,
                    )+
                    _ => panic!("Out of range vector elements index. index = {index}."),
                }
            }

            fn set(&self, index: usize, value: T) {
                match index {
                    $(
                        $index => self.$element = value,
                    )+
                    _ => panic!("Out of range vector elements index. index = {index}."),
                }
            }
        }

        impl<T: VectorElement> $type_name<T> {
            pub fn length_squared(&self) -> T {
                $(
                    self.$element * self.$element +
                )+ T::zero()
            }

            pub fn dot(&self, rhs: impl VectorLike<T, 2>) -> T {
                [$(
                    self.$element * rhs.$element,
                )+].sum()
            }
        }

        impl<T> $type_name<T>
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

            pub fn distance(&self, other: impl VectorLike<T, $dimension>) -> T {
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
    }
}

/*
#[cfg(test)]
mod ops_test {
    use super::*;

    type Vector3<T> = Vector<T, 3>;

    #[test]
    fn add() {
        let v1 = Vector3::new(1, 2, 4);
        let v2 = Vector3::new(5, 3, 7);
        assert_eq!(v1 + v2, [6, 5, 11].to_vector());
    }

    #[test]
    fn sub() {
        let v1 = Vector3::new(1, 2, 4);
        let v2 = Vector3::new(5, 3, 7);
        assert_eq!(v1 - v2, [-4, -1, -3].to_vector());
    }

    #[test]
    fn mul() {
        let v1 = Vector3::new(1, 2, 4);
        let v2 = Vector3::new(5, 3, 7);
        assert_eq!(v1 * v2, [5, 6, 28].to_vector());
    }

    #[test]
    fn div() {
        let v1 = Vector3::new(9.0, 10.0, 3.0);
        let v2 = Vector3::new(3.0, 2.0, 1.5);
        assert_eq!(v1 / v2, [3.0, 5.0, 2.0].to_vector());
    }

    #[test]
    fn add_assign() {
        let v1 = Vector3::new(1, 2, 4);
        let v2 = Vector3::new(5, 3, 7);
        let mut v = v1;
        v += v2;
        assert_eq!(v, v1 + v2);
    }

    #[test]
    fn sub_assign() {
        let v1 = Vector3::new(1, 2, 4);
        let v2 = Vector3::new(5, 3, 7);
        let mut v = v1;
        v -= v2;
        assert_eq!(v, v1 - v2);
    }

    #[test]
    fn mul_assign() {
        let v1 = Vector3::new(1, 2, 4);
        let v2 = Vector3::new(5, 3, 7);
        let mut v = v1;
        v *= v2;
        assert_eq!(v, v1 * v2);
    }

    #[test]
    fn div_assign() {
        let v1 = Vector3::new(9.0, 10.0, 3.0);
        let v2 = Vector3::new(3.0, 2.0, 1.5);
        let mut v = v1;
        v /= v2;
        assert_eq!(v, v1 / v2);
    }

    #[test]
    fn mul_scalar() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 2.0, [2.0, 4.0, 6.0].to_vector());
    }

    #[test]
    fn div_scalar() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v / 2.0, [0.5, 1.0, 1.5].to_vector());
    }

    #[test]
    fn mul_assgin_scalar() {
        let mut v = Vector3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v, [2.0, 4.0, 6.0].to_vector());
    }

    #[test]
    fn div_assgin_scalar() {
        let mut v = Vector3::new(1.0, 2.0, 3.0);
        v /= 2.0;
        assert_eq!(v, [0.5, 1.0, 1.5].to_vector());
    }

    #[test]
    fn mul_scalar_vector() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(2.0 * v, [2.0, 4.0, 6.0].to_vector());
    }

    #[test]
    fn index() {
        let v = Vector3::new(2, 3, 4);
        assert_eq!(v[0], 2);
        assert_eq!(v[1], 3);
        assert_eq!(v[2], 4);
    }

    #[test]
    #[should_panic]
    fn index_out_of_range() {
        let v = Vector3::new(2, 3, 4);
        let _ = v[3];
    }

    #[test]
    fn index_mut() {
        let mut v = Vector3::zero();
        v[0] = 2;
        v[1] = 3;
        v[2] = 4;
        assert_eq!(v[0], 2);
        assert_eq!(v[1], 3);
        assert_eq!(v[2], 4);
    }

    #[test]
    #[should_panic]
    fn index_mut_out_of_range() {
        let mut v = Vector3::zero();
        v[3] = 42;
    }
}
*/
