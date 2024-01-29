mod traits;
/*
pub use self::traits::*;
use crate::math::vector::*;
use num::{Float, One, Zero};
use std::ops::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<T: MatrixElement, const ROW: usize, const COL: usize> {
    pub elements: [Vector<T, COL>; ROW],
}

impl<T, const ROW: usize, const COL: usize> Eq for Matrix<T, ROW, COL> where T: MatrixElement + Eq {}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Default for Matrix<T, ROW, COL> {
    fn default() -> Self {
        let default_vector = Vector::<T, COL>::default();
        Self {
            elements: [default_vector; ROW],
        }
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Zero for Matrix<T, ROW, COL> {
    fn zero() -> Self {
        Self {
            elements: [Vector::<T, COL>::zero(); ROW],
        }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Matrix<T, ROW, COL> {
    pub fn new(m: impl MatrixLike<T, ROW, COL>) -> Self {
        let mut ret = Self::default();
        for r in 0..ROW {
            for c in 0..COL {
                ret.elements[r][c] = *m.get(r, c);
            }
        }
        ret
    }

    pub fn row(&self, n: usize) -> Vector<T, COL> {
        self.elements[n]
    }

    pub fn col(&self, n: usize) -> Vector<T, ROW> {
        let mut v = Vector::<T, ROW>::default();
        for i in 0..ROW {
            v[i] = self.elements[i][n];
        }
        v
    }
}

impl<T: MatrixElement + Float> Matrix<T, 2, 2> {
    pub fn rotate(theta: T) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        [[c, s], [-s, c]].into()
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> From<[[T; COL]; ROW]>
    for Matrix<T, ROW, COL>
{
    fn from(m: [[T; COL]; ROW]) -> Self {
        Self::new(m)
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> From<[Vector<T, COL>; ROW]>
    for Matrix<T, ROW, COL>
{
    fn from(m: [Vector<T, COL>; ROW]) -> Self {
        Self::new(m)
    }
}

pub trait ToMatrix<T: MatrixElement, const ROW: usize, const COL: usize> {
    fn to_matrix(self) -> Matrix<T, ROW, COL>;
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Add<Self> for Matrix<T, ROW, COL> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::zero();
        for c in 0..ROW {
            result.elements[c] = self.elements[c] + rhs.elements[c];
        }
        result
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Sub<Self> for Matrix<T, ROW, COL> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::zero();
        for r in 0..ROW {
            result.elements[r] = self.elements[r] - rhs.elements[r];
        }
        result
    }
}

impl<T: MatrixElement, const L: usize, const M: usize, const N: usize> Mul<Matrix<T, M, N>>
    for Matrix<T, L, M>
{
    type Output = Matrix<T, L, N>;

    fn mul(self, rhs: Matrix<T, M, N>) -> Self::Output {
        let mut m = Self::Output::default();
        for l in 0..L {
            for n in 0..N {
                m.elements[l][n] = self.row(l).dot(rhs.col(n));
            }
        }
        m
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Mul<T> for Matrix<T, ROW, COL> {
    type Output = Matrix<T, ROW, COL>;

    fn mul(self, scalar: T) -> Self::Output {
        let mut m = self;
        for r in 0..ROW {
            for c in 0..COL {
                m.elements[r][c] *= scalar;
            }
        }
        m
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Div<T> for Matrix<T, ROW, COL> {
    type Output = Matrix<T, ROW, COL>;

    fn div(self, scalar: T) -> Self::Output {
        let scale = T::one() / scalar;
        self * scale
    }
}

macro_rules! impl_mul_scalar_matrix {
    ($type: ty) => {
        impl<const ROW: usize, const COL: usize> Mul<Matrix<$type, ROW, COL>> for $type {
            type Output = Matrix<$type, ROW, COL>;

            fn mul(self, m: Matrix<$type, ROW, COL>) -> Self::Output {
                m * self
            }
        }
    };
}
impl_mul_scalar_matrix!(i8);
impl_mul_scalar_matrix!(i16);
impl_mul_scalar_matrix!(i32);
impl_mul_scalar_matrix!(i64);
impl_mul_scalar_matrix!(i128);
impl_mul_scalar_matrix!(u8);
impl_mul_scalar_matrix!(u16);
impl_mul_scalar_matrix!(u32);
impl_mul_scalar_matrix!(u64);
impl_mul_scalar_matrix!(u128);
impl_mul_scalar_matrix!(isize);
impl_mul_scalar_matrix!(usize);
impl_mul_scalar_matrix!(f32);
impl_mul_scalar_matrix!(f64);

impl<T: MatrixElement, const ROW: usize, const COL: usize> Mul<Vector<T, COL>>
    for Matrix<T, ROW, COL>
{
    type Output = Matrix<T, ROW, 1>;

    fn mul(self, v: Vector<T, COL>) -> Self::Output {
        let mut m = Self::Output::default();
        for i in 0..ROW {
            m[i][0] = self.row(i).dot(v);
        }
        m
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Index<usize> for Matrix<T, ROW, COL> {
    type Output = Vector<T, COL>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> IndexMut<usize> for Matrix<T, ROW, COL> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<T, const ROW: usize, const COL: usize, M> ToMatrix<T, ROW, COL> for M
where
    T: MatrixElement,
    M: MatrixLike<T, ROW, COL>,
{
    fn to_matrix(self) -> Matrix<T, ROW, COL> {
        let mut m = Matrix::<T, ROW, COL>::default();
        for r in 0..ROW {
            for c in 0..COL {
                m.elements[r][c] = *self.get(r, c);
            }
        }
        m
    }
}

impl<T: MatrixElement + One, const N: usize> SquareMatrix for Matrix<T, N, N> {
    fn identity() -> Self {
        let mut result = Self::default();
        for i in 0..N {
            result.elements[i][i] = T::one();
        }
        result
    }
}

#[cfg(test)]
mod ops_test {
    use super::*;

    #[test]
    fn add() {
        let m1: Matrix<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        let m2: Matrix<i32, 2, 3> = [[11, 12, 13], [14, 15, 16]].into();
        assert_eq!(m1 + m2, [[12, 14, 16], [18, 20, 22]].to_matrix());
    }

    #[test]
    fn sub() {
        let m1: Matrix<i32, 2, 3> = [[3, 5, 7], [11, 13, 17]].into();
        let m2: Matrix<i32, 2, 3> = [[1, 2, 3], [3, 6, 8]].into();
        assert_eq!(m1 - m2, [[2, 3, 4], [8, 7, 9]].to_matrix());
    }

    #[test]
    fn mul1() {
        let m1: Matrix<i32, 1, 2> = [[3, 4]].into();
        let m2: Matrix<i32, 2, 1> = [[6], [8]].into();
        assert_eq!(m1 * m2, [[50]].to_matrix());
    }

    #[test]
    fn mul2() {
        let m1: Matrix<i32, 2, 1> = [[1], [2]].into();
        let m2: Matrix<i32, 1, 3> = [[3, 4, 5]].into();
        assert_eq!(m1 * m2, [[3, 4, 5], [6, 8, 10]].to_matrix());
    }

    #[test]
    fn mul3() {
        let m1: Matrix<i32, 2, 2> = [[1, 2], [3, 4]].into();
        let m2: Matrix<i32, 2, 2> = [[5, 6], [7, 8]].into();
        assert_eq!(m1 * m2, [[19, 22], [43, 50]].to_matrix());
    }

    #[test]
    fn mul4() {
        let m1: Matrix<i32, 3, 2> = [[0, 1], [2, 3], [4, 5]].into();
        let m2: Matrix<i32, 2, 4> = [[0, 1, 2, 3], [4, 5, 6, 7]].into();
        assert_eq!(
            m1 * m2,
            [[4, 5, 6, 7], [12, 17, 22, 27], [20, 29, 38, 47]].to_matrix()
        );
    }

    #[test]
    fn mul_scalar() {
        let m: Matrix<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(m * 2, [[2, 4, 6], [8, 10, 12]].to_matrix());
    }

    #[test]
    fn mul_scalar_matrix() {
        let m: Matrix<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(2 * m, [[2, 4, 6], [8, 10, 12]].to_matrix());
    }

    #[test]
    fn mul_matrix_vector() {
        let m: Matrix<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        let v: Vector<i32, 3> = [3, 4, 5].into();
        assert_eq!(m * v, [[26], [62]].to_matrix());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    type Vector2<T> = Vector<T, 2>;
    type Matrix2<T> = Matrix<T, 2, 2>;
    type Matrix3<T> = Matrix<T, 3, 3>;

    #[test]
    fn new_from_vector_array() {
        let m1 = Matrix::new([Vector2::new(1, 2), Vector2::new(3, 4), Vector2::new(5, 6)]);
        let m2 = Matrix::<i32, 3, 2> {
            elements: [[1, 2].to_vector(), [3, 4].to_vector(), [5, 6].to_vector()],
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn new_from_array() {
        let m1 = Matrix::new([[1, 2], [3, 4], [5, 6]]);
        let m2 = Matrix::<i32, 3, 2> {
            elements: [[1, 2].to_vector(), [3, 4].to_vector(), [5, 6].to_vector()],
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn from_vector_array() {
        let m1 = Matrix::from([Vector2::new(1, 2), Vector2::new(3, 4), Vector2::new(5, 6)]);
        let m2 = Matrix::<i32, 3, 2> {
            elements: [[1, 2].to_vector(), [3, 4].to_vector(), [5, 6].to_vector()],
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn from_array() {
        let m1 = Matrix::<i32, 3, 2>::from([[1, 2], [3, 4], [5, 6]]);
        let m2 = Matrix::<i32, 3, 2> {
            elements: [[1, 2].to_vector(), [3, 4].to_vector(), [5, 6].to_vector()],
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn row() {
        let m: Matrix<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(m.row(0), [1, 2, 3].to_vector());
        assert_eq!(m.row(1), [4, 5, 6].to_vector());
    }

    #[test]
    #[should_panic]
    fn row_out_of_range() {
        let m: Matrix<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        let _ = m.row(3);
    }

    #[test]
    fn col() {
        let m: Matrix<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(m.col(0), [1, 4].to_vector());
        assert_eq!(m.col(1), [2, 5].to_vector());
        assert_eq!(m.col(2), [3, 6].to_vector());
    }

    #[test]
    #[should_panic]
    fn col_out_of_range() {
        let m: Matrix<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        let _ = m.col(3);
    }

    #[test]
    fn to_matrix() {
        let m1 = Matrix::<i32, 2, 3> {
            elements: [[1, 2, 3].to_vector(), [4, 5, 6].to_vector()],
        };
        let m2: Matrix<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].to_matrix();
        assert_eq!(m1, m2);
    }

    #[test]
    fn identity() {
        let m: Matrix2<i32> = [[1, 0], [0, 1]].into();
        assert_eq!(m, Matrix::identity());

        let m: Matrix3<i32> = [[1, 0, 0], [0, 1, 0], [0, 0, 1]].into();
        assert_eq!(m, Matrix::identity());
    }
}
*/
