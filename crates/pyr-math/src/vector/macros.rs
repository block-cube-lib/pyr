// impl basic vector operations
macro_rules! impl_vec_ops {
    ($VecN:ident, $($field:ident),+) => {
            impl_vec_ops!($VecN, Add, add, +, AddAssign, add_assign, +=, $($field),+);
            impl_vec_ops!($VecN, Sub, sub, -, SubAssign, sub_assign, -=, $($field),+);
            impl_vec_ops!($VecN, Mul, mul, *, MulAssign, mul_assign, *=, $($field),+);
            impl_vec_ops!($VecN, Div, div, /, DivAssign, div_assign, /=, $($field),+);
    };
    ($VecN:ident, $ops_trait:ident, $ops_func:ident, $op:tt, $ops_assign_trait:ident, $ops_assign_func:ident, $op_assign:tt, $($field:ident),+) => {
        impl<T: Scalar> $ops_trait<Self> for $VecN<T> {
            type Output = Self;

            #[inline]
            fn $ops_func(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $field: self.$field $op rhs.$field,
                    )+
                }
            }
        }

        impl<T: Scalar> $ops_assign_trait<Self> for $VecN<T> {
            #[inline]
            fn $ops_assign_func(&mut self, rhs: Self) {
                $(
                    self.$field $op_assign rhs.$field;
                )+
            }
        }
    };
}

// impl scalar operations
macro_rules! impl_vec_scalar_ops {
    ($VecN:ident, $($field:ident),+) => {
        impl<T: Scalar> Mul<T> for $VecN<T> {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                Self {
                    $(
                        $field: self.$field * rhs,
                    )+
                }
            }
        }

        impl<T: Scalar> MulAssign<T> for $VecN<T> {
            #[inline]
            fn mul_assign(&mut self, rhs: T) {
                *self = Self {
                    $(
                        $field: self.$field * rhs,
                    )+
                }
            }
        }

        impl<T: Scalar> Div<T> for $VecN<T> {
            type Output = Self;

            #[inline]
            fn div(self, rhs: T) -> Self::Output {
                Self {
                    $(
                        $field: self.$field / rhs,
                    )+
                }
            }
        }

        impl<T: Scalar> DivAssign<T> for $VecN<T> {
            #[inline]
            fn div_assign(&mut self, rhs: T) {
                *self = Self {
                    $(
                        $field: self.$field / rhs,
                    )+
                }
            }
        }
    }
}

// impl index operations
macro_rules! impl_vec_index_ops {
    ($VecN:ident, $($field:ident, $idx:expr),+) => {
        impl<T: Scalar> Index<usize> for $VecN<T> {
            type Output = T;

            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    $(
                        $idx => &self.$field,
                    )+
                    _ => panic!("Index out of bounds"),
                }
            }
        }

        impl<T: Scalar> IndexMut<usize> for $VecN<T> {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    $(
                        $idx => &mut self.$field,
                    )+
                    _ => panic!("Index out of bounds"),
                }
            }
        }
    };
}

// impl Neg
macro_rules! impl_vec_neg {
    ($VecN:ident, $($field:ident),+) => {
        impl<T: Scalar + Neg<Output = T>> Neg for $VecN<T> {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                Self {
                    $(
                        $field: -self.$field,
                    )+
                }
            }
        }
    };
}

// Scalar impl Mul<VecN>
macro_rules! impl_scalar_vec_mul {
    ($VecN:ident, $scalar:ty, $($field:ident),+) => {
        impl Mul<$VecN<$scalar>> for $scalar {
            type Output = $VecN<$scalar>;

            #[inline]
            fn mul(self, rhs: $VecN<$scalar>) -> Self::Output {
                rhs * self
            }
        }
    };
}

// Primitive type impl Mul<VecN>
macro_rules! impl_primitive_scalar_vec_mul {
    ($VecN:ident, $($field:ident),+) => {
        impl_scalar_vec_mul!($VecN,    i8, $($field),+);
        impl_scalar_vec_mul!($VecN,   i16, $($field),+);
        impl_scalar_vec_mul!($VecN,   i32, $($field),+);
        impl_scalar_vec_mul!($VecN,   i64, $($field),+);
        impl_scalar_vec_mul!($VecN,    u8, $($field),+);
        impl_scalar_vec_mul!($VecN,   u16, $($field),+);
        impl_scalar_vec_mul!($VecN,   u32, $($field),+);
        impl_scalar_vec_mul!($VecN,   u64, $($field),+);
        impl_scalar_vec_mul!($VecN, isize, $($field),+);
        impl_scalar_vec_mul!($VecN, usize, $($field),+);
    };
}

macro_rules! impl_vec_approx {
    ($VecN:ident, $($field:ident),+) => {
        impl<T> crate::scalar::ApproxEq for $VecN<T>
        where T: FloatScalar {
            type Epsilon = T;

            #[inline]
            fn approx_eq(&self, other: &Self) -> bool {
                $( self.$field.approx_eq(&other.$field) )&&+
            }

            #[inline]
            fn approx_eq_eps(&self, other: &Self, epsilon: T) -> bool {
                $( self.$field.approx_eq_eps(&other.$field, epsilon) )&&+
            }
        }
    };
}

macro_rules! impl_vec_float_math {
    ($VecN:ident) => {
        impl<T: crate::scalar::FloatScalar> $VecN<T> {
            #[inline]
            pub fn length(self) -> T {
                self.length_squared().sqrt()
            }

            #[inline]
            pub fn distance_squared(self, other: Self) -> T {
                (self - other).length_squared()
            }

            #[inline]
            pub fn distance(self, other: Self) -> T {
                self.distance_squared(other).sqrt()
            }

            #[inline]
            pub fn try_normalize(self) -> Option<Self> {
                let len_squ = self.length_squared();
                if len_squ.approx_eq(&T::ZERO) {
                    None
                } else {
                    let len = len_squ.sqrt();
                    Some(self / len)
                }
            }

            #[inline]
            pub fn normalize(self) -> Self {
                self.try_normalize().unwrap_or(Self::ZERO)
            }

            #[inline]
            pub fn is_normalized(self) -> bool {
                self.length_squared().approx_eq(&T::ONE)
            }

            #[inline]
            pub fn reflect(self, normal: Self) -> Self {
                debug_assert!(
                    normal.is_normalized(),
                    "reflect: The normal vector must be normalized."
                );
                let two = T::ONE + T::ONE;
                self - normal * (two * self.dot(normal))
            }

            #[inline]
            pub fn project(self, rhs: Self) -> Self {
                let len_squ = rhs.length_squared();
                if len_squ.approx_eq(&T::ZERO) {
                    Self::ZERO
                } else {
                    rhs * (self.dot(rhs) / len_squ)
                }
            }

            #[inline]
            pub fn clamp_length(self, max_length: T) -> Self {
                let len_squ = self.length_squared();
                let max_squ = max_length * max_length;
                if len_squ > max_squ {
                    self * (max_length / len_squ.sqrt())
                } else {
                    self
                }
            }
        }
    };
}

macro_rules! impl_vec_cast {
    ($VecN:ident, $($field:ident),+) => {
        impl<T> $VecN<T> {
            #[inline]
            pub fn cast<U: Scalar>(self) -> $VecN<U>
            where T: num_traits::AsPrimitive<U> {
                $VecN {
                    $(
                        $field: self.$field.as_(),
                    )+
                }
            }
        }
    };
}
