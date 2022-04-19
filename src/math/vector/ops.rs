use paste::paste;

macro_rules! impl_ops {
    ($trait_name: tt, $function_name: ident, $op: tt) => {
        impl<T: VectorElement, const DIMENSION: usize> $trait_name<Self> for Vector<T, DIMENSION> {
            type Output = Self;

            fn $function_name(self, rhs: Self) -> Self::Output {
                let mut result = Self::zero();
                for i in 0..DIMENSION {
                    result.elements[i] = self.elements[i] $op rhs.elements[i];
                }
                result
            }
        }

        paste!{
            impl<T: VectorElement, const DIMENSION: usize> [<$trait_name Assign>]<Self> for Vector<T, DIMENSION> {
                fn [<$function_name _assign>](&mut self, rhs: Self) {
                    *self = *self $op rhs;
                }
            }
        }
    };
}
impl_ops!(Add, add, +);
impl_ops!(Sub, sub, -);
impl_ops!(Mul, mul, *);
impl_ops!(Div, div, /);

macro_rules! impl_ops_vector_scalar {
    ($trait_name: tt, $function_name: ident, $op: tt) => {
        impl<T: VectorElement, const DIMENSION: usize> $trait_name<T> for Vector<T, DIMENSION> {
            type Output = Self;

            fn $function_name(self, scalar: T) -> Self::Output {
                self $op Self::from([scalar; DIMENSION])
            }
        }

        paste!{
            impl<T: VectorElement, const DIMENSION: usize> [<$trait_name Assign>]<T> for Vector<T, DIMENSION> {
                fn [<$function_name _assign>](&mut self, scalar: T) {
                    *self = *self $op scalar;
                }
            }
        }
    };
}

impl_ops_vector_scalar!(Mul, mul, *);
impl_ops_vector_scalar!(Div, div, /);

macro_rules! impl_mul_scalar_vector {
    ($type: ty) => {
        impl<const DIMENSION: usize> Mul<Vector<$type, DIMENSION>> for $type {
            type Output = Vector<$type, DIMENSION>;

            fn mul(self, rhs: Vector<$type, DIMENSION>) -> Self::Output {
                rhs * self
            }
        }
    };
}

impl_mul_scalar_vector!(i8);
impl_mul_scalar_vector!(i16);
impl_mul_scalar_vector!(i32);
impl_mul_scalar_vector!(i64);
impl_mul_scalar_vector!(i128);
impl_mul_scalar_vector!(u8);
impl_mul_scalar_vector!(u16);
impl_mul_scalar_vector!(u32);
impl_mul_scalar_vector!(u64);
impl_mul_scalar_vector!(u128);
impl_mul_scalar_vector!(isize);
impl_mul_scalar_vector!(usize);
impl_mul_scalar_vector!(f32);
impl_mul_scalar_vector!(f64);

impl<T: VectorElement, const DIMENSION: usize> Index<usize> for Vector<T, DIMENSION> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T: VectorElement, const DIMENSION: usize> IndexMut<usize> for Vector<T, DIMENSION> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<T, const DIMENSION: usize> Neg for Vector<T, DIMENSION>
where
    T: VectorElement + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut v = Self::zero();
        for i in 0..DIMENSION {
            v[i] = self[i].neg();
        }
        v
    }
}

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
