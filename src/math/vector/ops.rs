///
/// impl vector operators.
///
/// Add(Assign), Sub(Assign), Mut(Assign), Div(Assign)
/// Index, IndexMut
/// Neg
///
#[macro_export]
macro_rules! impl_ops {
    ($type_name: tt; $( $element: tt ),+) => {
        impl_ops_helper!($type_name; $($element),+; Add, add, +);
        impl_ops_helper!($type_name; $($element),+; Sub, sub, -);
        impl_ops_helper!($type_name; $($element),+; Mul, mul, *);
        impl_ops_helper!($type_name; $($element),+; Div, div, /);
        impl_ops_helper!($type_name; $($element),+; assign, AddAssign, add_assign, +);
        impl_ops_helper!($type_name; $($element),+; assign, SubAssign, sub_assign, -);
        impl_ops_helper!($type_name; $($element),+; assign, MulAssign, mul_assign, *);
        impl_ops_helper!($type_name; $($element),+; assign, DivAssign, div_assign, /);

        impl<T: super::VectorElement> std::ops::Mul<T> for $type_name<T> {
            type Output = $type_name<T>;

            fn mul(self, rhs: T) -> Self::Output {
                $type_name::<T> {
                    $(
                        $element: self.$element * rhs,
                    )+
                }
            }
        }
        impl<T: super::VectorElement> std::ops::Div<T> for $type_name<T> {
            type Output = $type_name<T>;

            fn div(self, rhs: T) -> Self::Output {
                $type_name::<T> {
                    $(
                        $element: self.$element / rhs,
                    )+
                }
            }
        }

        impl<T: super::VectorElement> std::ops::MulAssign<T> for $type_name<T> {
            fn mul_assign(&mut self, rhs: T) {
                *self = *self * rhs;
            }
        }
        impl<T: super::VectorElement> std::ops::DivAssign<T> for $type_name<T> {
            fn div_assign(&mut self, rhs: T) {
                *self = *self / rhs;
            }
        }

        impl_mul_scaler_vector!($type_name; i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 isize usize);
        impl_index!($type_name; $($element),+);

        impl<T: super::VectorElement> std::ops::Neg for $type_name<T>
        where T: std::ops::Neg<Output = T> {
            type Output = $type_name<T>;

            fn neg(self) -> Self::Output {
                $type_name {
                    $(
                        $element: -self.$element,
                    )*
                }
            }
        }
    };
}
macro_rules! impl_ops_helper {
    ($type_name: tt; $($element: tt),+; $trait_name: tt, $func_name: tt, $op: tt) => {
        impl<T: super::VectorElement> std::ops::$trait_name for $type_name<T> {
            type Output = Self;

            fn $func_name(self, rhs: Self) -> Self::Output {
                $type_name::<T> {
                    $(
                        $element: self.$element $op rhs.$element,
                    )+
                }
            }
        }
    };
    ($type_name: tt; $($element: tt),+; assign, $trait_name: tt, $func_name: tt, $op: tt) => {
        impl<T: super::VectorElement> std::ops::$trait_name for $type_name<T> {
            fn $func_name(&mut self, rhs: $type_name<T>) {
                *self = *self $op rhs;
            }
        }
    };
}

macro_rules! impl_mul_scaler_vector {
    ($type_name: tt; $($type: ty)*) => {
        $(
            impl std::ops::Mul<$type_name<$type>> for $type
            {
                type Output = $type_name<$type>;

                fn mul(self, rhs: $type_name<$type>) -> Self::Output {
                    rhs * self
                }
            }
        )*
    }
}

macro_rules! index_match {
    (x) => {
        0
    };
    (y) => {
        1
    };
    (z) => {
        2
    };
    (w) => {
        3
    };
}
macro_rules! impl_index {
    ($vector_type_name: tt; $($element: tt),+) => {
        impl_index!($vector_type_name; $($element),+; i8);
        impl_index!($vector_type_name; $($element),+; i16);
        impl_index!($vector_type_name; $($element),+; i32);
        impl_index!($vector_type_name; $($element),+; i64);
        impl_index!($vector_type_name; $($element),+; u8);
        impl_index!($vector_type_name; $($element),+; u16);
        impl_index!($vector_type_name; $($element),+; u32);
        impl_index!($vector_type_name; $($element),+; u64);
        impl_index!($vector_type_name; $($element),+; isize);
        impl_index!($vector_type_name; $($element),+; usize);
    };
    ($vector_type_name: tt; $($element: tt),+; $index_type: ty) => {
        impl<T: super::VectorElement> std::ops::Index<$index_type> for $vector_type_name<T> {
            type Output = T;

            fn index(&'_ self, index: $index_type) -> &'_ T {
                match index {
                    $(
                        index_match!($element) => &self.$element,
                    )+
                    _ => panic!("Out of range"),
                }
            }
        }

        impl<T: super::VectorElement> std::ops::IndexMut<$index_type> for $vector_type_name<T> {
            fn index_mut(&'_ mut self, index: $index_type) -> &'_ mut T {
                match index {
                    $(
                        index_match!($element) => &mut self.$element,
                    )+
                    _ => panic!("Out of range"),
                }
            }
        }
    };
}

//
// test
//
#[cfg(test)]
#[macro_export]
macro_rules! ops_test {
    ($type: tt; $($field: tt),*) => {
        ops_test_helper!($type, $($field),*; add, +, f32);
        ops_test_helper!($type, $($field),*; sub, -, f32);
        ops_test_helper!($type, $($field),*; mul, *, f32);
        ops_test_helper!($type, $($field),*; div, /, f32);
        ops_test_helper!($type, $($field),*; add_assign, +, +=, f32);
        ops_test_helper!($type, $($field),*; sub_assign, -, -=, f32);
        ops_test_helper!($type, $($field),*; mul_assign, *, *=, f32);
        ops_test_helper!($type, $($field),*; div_assign, /, /=, f32);
        ops_test_helper!(vector_op_scalar =>, $type, $($field),*; mul_scalar, *, f32);
        ops_test_helper!(vector_op_scalar =>, $type, $($field),*; div_scalar, /, f32);
    };
}

#[cfg(test)]
macro_rules! ops_test_helper {
    ($type: tt, $($field: tt),*; $test_name: ident, $op: tt, $element_type: ty) => {
        proptest! {
            #[test]
            fn $test_name( $($field in -100.0..100.0),* ) {
                $(
                    let $field = $field as $element_type;
                )*
                let v1 = $type {
                    $(
                        $field: $field
                    ),*
                };
                let v2 = $type {
                    $(
                        $field: $field * 2.0
                    ),*
                };
                let v3 = v1 $op v2;
                $(
                    assert_eq!(v3.$field, $field $op ($field * 2.0));
                )*
            }
        }
    };
    ($type: tt, $($field: tt),*; $test_name: ident, $op: tt, $assign_op: tt, $element_type: ty) => {
        proptest! {
            #[test]
            fn $test_name( $($field in -100.0..100.0),* ) {
                $(
                    let $field = $field as $element_type;
                )*
                let mut v1 = $type {
                    $(
                        $field: $field
                    ),*
                };
                let v2 = $type {
                    $(
                        $field: $field * 2.0
                    ),*
                };

                let v3 = v1 $op v2;
                v1 $assign_op v2;
                assert_eq!(v1, v3);
            }
        }
    };
    (vector_op_scalar =>, $type: tt, $($field: tt),*; $test_name: ident, $op: tt, $element_type: ty) => {
        proptest! {
            #[test]
            fn $test_name( $($field in -100.0..100.0),* ,scalar in -100.0..100.0) {
                let scalar = scalar as $element_type;
                $(
                    let $field = $field as $element_type;
                )*
                let mut v = $type {
                    $(
                        $field: $field
                    ),*
                };

                v = v $op scalar;
                $(
                    assert_eq!(v.$field, $field $op scalar);
                )*
            }
        }
    };
}
