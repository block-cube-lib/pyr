///
/// impl vector operators.
///
/// Add(Assign), Sub(Assign), Mut(Assign), Div(Assign)
/// Index, IndexMut
/// Neg
///
#[macro_export]
macro_rules! impl_ops {
    ($type: tt; $($field: tt),*) => {
        impl_ops_helper!($type, $( $field ),*; Add, add, +);
        impl_ops_helper!($type, $( $field ),*; Sub, sub, -);
        impl_ops_helper!($type, $( $field ),*; Mul, mul, *);
        impl_ops_helper!($type, $( $field ),*; Div, div, /);
        impl_ops_helper!($type, $( $field ),*; AddAssign, Add, add_assign, +);
        impl_ops_helper!($type, $( $field ),*; SubAssign, Sub, sub_assign, -);
        impl_ops_helper!($type, $( $field ),*; MulAssign, Mul, mul_assign, *);
        impl_ops_helper!($type, $( $field ),*; DivAssign, Div, div_assign, /);

        impl_ops_scalar!($type, $( $field ),*; Mul, mul, *);
        impl_ops_scalar!($type, $( $field ),*; Div, div, /);
        impl_ops_scalar!($type, $( $field ),*; MulAssign, Mul, mul_assign, *);
        impl_ops_scalar!($type, $( $field ),*; DivAssign, Div, div_assign, /);

        impl_ops_mul_scalar!($type, $( $field ),*; f32);
        impl_ops_mul_scalar!($type, $( $field ),*; f64);
        impl_ops_mul_scalar!($type, $( $field ),*; i8);
        impl_ops_mul_scalar!($type, $( $field ),*; i16);
        impl_ops_mul_scalar!($type, $( $field ),*; i32);
        impl_ops_mul_scalar!($type, $( $field ),*; i64);
        impl_ops_mul_scalar!($type, $( $field ),*; i128);
        impl_ops_mul_scalar!($type, $( $field ),*; isize);
        impl_ops_mul_scalar!($type, $( $field ),*; u8);
        impl_ops_mul_scalar!($type, $( $field ),*; u16);
        impl_ops_mul_scalar!($type, $( $field ),*; u32);
        impl_ops_mul_scalar!($type, $( $field ),*; u64);
        impl_ops_mul_scalar!($type, $( $field ),*; u128);
        impl_ops_mul_scalar!($type, $( $field ),*; usize);

        impl_index_ops!($type; $( $field ),*);

        impl<T> ops::Neg for $type<T>
        where T: ops::Neg {
            type Output = $type<T>;

            fn neg(self) -> Self::Output {
                $type {
                    $(
                        $field: self.$field,
                    )*
                }
            }
        }
    }
}

macro_rules! impl_ops_helper {
    // +, -, *, /
    ($type: tt, $($field:tt),*; $trait: tt, $func: ident, $op: tt) => {
        impl<T> ops::$trait for $type<T>
        where
            T: ops::$trait<T, Output = T>,
        {
            type Output = $type<T>;

            fn $func(self, rhs: $type<T>) -> $type<T> {
                $type {
                    $(
                        $field: self.$field $op rhs.$field
                    ),*
                }
            }
        }
    };

    // +=, -=, *=, /=
    ($type: tt, $($field:tt),*; $trait: tt, $operator_trait: tt, $func: ident, $op: tt) => {
        impl<T> ops::$trait for $type<T>
        where
            T: ops::$operator_trait<T, Output = T> + Copy,
        {
            fn $func(&mut self, rhs: $type<T>) {
                *self = $type {
                            $(
                                $field: self.$field $op rhs.$field
                            ),*
                        };
            }
        }
    }
}

macro_rules! impl_ops_scalar {
    // T: *, /
    ($type: tt, $($field:tt),*; $trait: tt, $func: ident, $op: tt) => {
        impl<T> ops::$trait<T> for $type<T>
        where
            T: ops::$trait<T, Output = T> + Copy,
        {
            type Output = $type<T>;

            fn $func(self, rhs: T) -> $type<T> {
                $type {
                    $(
                        $field: self.$field $op rhs
                    ),*
                }
            }
        }
    };

    // T: *=, /=
    ($type: tt, $($field:tt),*; $trait: tt, $operator_trait: tt, $func: ident, $op: tt) => {
        impl<T> ops::$trait<T> for $type<T>
        where
            T: ops::$operator_trait<T, Output = T> + Copy,
        {
            fn $func(&mut self, rhs: T) {
                *self = $type {
                            $(
                                $field: self.$field $op rhs
                            ),*
                        };
            }
        }
    }
}

// Mul<VectorX<Scalar>> for Scalar
macro_rules! impl_ops_mul_scalar {
    ($type: tt, $($field:tt),*; $element_type: ty) => {
        impl ops::Mul<$type<$element_type>> for $element_type {
            type Output = $type<$element_type>;

            fn mul(self, rhs: $type<$element_type>) -> Self::Output {
                $type {
                    $(
                        $field: self * rhs.$field
                    ),*
                }
            }
        }
    }
}

// Index, IndexMut
macro_rules! impl_index_ops {
    ($type: tt; $($field: tt),*) => {
        impl<T> ops::Index<usize> for $type<T> {
            type Output = T;

            fn index(&self, index: usize) -> &T {
                match index {
                    $(
                        index!($field) => &self.$field,
                    )*
                    _ => panic!("Out of range"),
                }
            }
        }

        impl<T> ops::IndexMut<usize> for $type<T> {
            fn index_mut(&'_ mut self, index: usize) -> &'_ mut T {
                match index {
                    $(
                        index!($field) => &mut self.$field,
                    )*
                    _ => panic!("Out of range"),
                }
            }
        }
    };
}

macro_rules! index {
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

//
// test
//
#[cfg(test)]
#[macro_export]
macro_rules! index_max {
    ($index: expr) => { 0 };
    ($index: expr, $field_head: tt, $($field_tail: tt),*) => { 1 + index_max($($field_tail),*) };
}

#[cfg(test)]
#[macro_export]
macro_rules! ops_test {
    ($type: tt; $($field: tt),*) => {
        ops_test_helper!($type, $($field),*; add, +, f32);
        ops_test_helper!($type, $($field),*; sub, -, f32);
        //#[test]
        //fn index() {
        //    let v = Vector1 { x: 100.0 };
        //    assert_eq!(v.x, v[0]);
        //}
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
                    assert_eq!(v3.$field, $field $op $field * 2.0);
                )*
            }
        }
    };
}
