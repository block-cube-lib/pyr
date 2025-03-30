mod traits;

pub use self::traits::*;
use crate::math::vec::traits::*;
use num::Float;
use std::ops::*;

/// Matrix type. column-major order.
/// Mat<i32, 2, 3> is a 2 row 3 column matrix of i32.
/// [[a1, a2, a3]
///  [b1, b2, b3]]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat<T: MatrixElement, const ROW: usize, const COL: usize> {
    pub elements: [[T; COL]; ROW],
}

impl<T, const ROW: usize, const COL: usize> Eq for Mat<T, ROW, COL> where T: MatrixElement + Eq {}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Default for Mat<T, ROW, COL> {
    fn default() -> Self {
        use std::default::Default;
        Self {
            elements: [[<T as Default>::default(); COL]; ROW],
        }
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Mat<T, ROW, COL> {
    /// ```
    /// use pyr::math::Mat;
    /// assert_eq!(Mat::<i32, 3, 2>::ZERO, Mat::<i32, 3, 2> { elements: [[0, 0], [0, 0], [0, 0]] });
    /// ```
    pub const ZERO: Self = Self {
        elements: [[T::ZERO; COL]; ROW],
    };

    /// ```
    /// use pyr::math::Mat;
    /// assert_eq!(Mat::<i32, 3, 2>::ONE, Mat::<i32, 3, 2> { elements: [[1, 1], [1, 1], [1, 1]] });
    /// ```
    pub const ONE: Self = Self {
        elements: [[T::ONE; COL]; ROW],
    };
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Mat<T, ROW, COL> {
    /// Create a new matrix from a MatrixLike object.
    /// ```
    /// use pyr::math::Mat;
    /// let m = Mat::new([[1, 2], [3, 4], [5, 6]]);
    /// assert_eq!(m, Mat::<i32, 3, 2> { elements: [[1, 2], [3, 4], [5, 6]] });
    /// ```
    pub fn new(m: impl MatrixLike<ROW, COL, ElementType = T>) -> Self {
        let mut ret = Self::default();
        for r in 0..ROW {
            for c in 0..COL {
                ret.elements[r][c] = *m.get(r, c);
            }
        }
        ret
    }

    /// ```
    /// use pyr::math::Mat;
    /// let m = Mat::new([[1, 2], [3, 4], [5, 6]]);
    /// assert_eq!(m.col(0), [1, 2]);
    /// assert_eq!(m.col(1), [3, 4]);
    /// assert_eq!(m.col(2), [5, 6]);
    /// ```
    pub fn col(&self, n: usize) -> [T; COL] {
        self.elements[n]
    }

    /// ```
    /// use pyr::math::Mat;
    /// let m = Mat::new([[1, 2], [3, 4], [5, 6]]);
    /// assert_eq!(m.row(0), [1, 3, 5]);
    /// assert_eq!(m.row(1), [2, 4, 6]);
    /// ```
    pub fn row(&self, n: usize) -> [T; ROW] {
        let mut v = [T::ZERO; ROW];
        for i in 0..ROW {
            v[i] = self.elements[i][n];
        }
        v
    }
}

impl<T: MatrixElement + Float> Mat<T, 2, 2> {
    pub fn rotate(theta: T) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self::new([[c, s], [-s, c]])
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> MatrixLike<ROW, COL>
    for Mat<T, ROW, COL>
{
    type ElementType = T;

    fn get(&self, row: usize, col: usize) -> &T {
        &self.elements[row][col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.elements[row][col]
    }
}

/// ```
/// use pyr::math::Mat;
/// let m1 = Mat::new([[1, 2], [3, 4], [5, 6]]);
/// let m2 = Mat::from([[1, 2], [3, 4], [5, 6]]);
/// assert_eq!(m1, m2);
/// ```
/// ```
/// use pyr::math::{Mat, Vec2};
/// let m1 = Mat::new([Vec2::new(1, 2), Vec2::new(3, 4), Vec2::new(5, 6)]);
/// let m2 = Mat::from([[1, 2], [3, 4], [5, 6]]);
/// assert_eq!(m1, m2);
/// ```
impl<T, V, const ROW: usize, const COL: usize> From<[V; ROW]> for Mat<T, ROW, COL>
where
    V: VectorLike<COL, ElementType = T>,
    T: MatrixElement,
{
    fn from(m: [V; ROW]) -> Self {
        Self::new(m)
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Add<Self> for Mat<T, ROW, COL> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::ZERO;
        for r in 0..ROW {
            for c in 0..COL {
                result.elements[r][c] = self.elements[r][c] + rhs.elements[r][c];
            }
        }
        result
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Sub<Self> for Mat<T, ROW, COL> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::ZERO;
        for r in 0..ROW {
            for c in 0..COL {
                result.elements[r][c] = self.elements[r][c] - rhs.elements[r][c];
            }
        }
        result
    }
}

impl<T, const L: usize, const M: usize, const N: usize> Mul<Mat<T, M, N>> for Mat<T, L, M>
where
    T: MatrixElement,
{
    type Output = Mat<T, L, N>;

    fn mul(self, rhs: Mat<T, M, N>) -> Self::Output {
        use crate::math::vec::ops::dot;
        let mut m = Self::Output::ZERO;
        for l in 0..L {
            for n in 0..N {
                m.elements[l][n] = dot(self.col(l), rhs.row(n));
            }
        }
        m
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> Mul<T> for Mat<T, ROW, COL> {
    type Output = Mat<T, ROW, COL>;

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

impl<T: MatrixElement, const ROW: usize, const COL: usize> Div<T> for Mat<T, ROW, COL> {
    type Output = Mat<T, ROW, COL>;

    fn div(self, scalar: T) -> Self::Output {
        let scale = T::ONE / scalar;
        self * scale
    }
}

macro_rules! impl_mul_scalar_matrix {
    ($type: ty) => {
        impl<const ROW: usize, const COL: usize> Mul<Mat<$type, ROW, COL>> for $type {
            type Output = Mat<$type, ROW, COL>;

            fn mul(self, m: Mat<$type, ROW, COL>) -> Self::Output {
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

impl<T: MatrixElement, const ROW: usize, const COL: usize> Index<usize> for Mat<T, ROW, COL> {
    type Output = [T; COL];

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T: MatrixElement, const ROW: usize, const COL: usize> IndexMut<usize> for Mat<T, ROW, COL> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

mod helper {
    use super::*;
    pub const fn identity<T: MatrixElement, const N: usize>() -> Mat<T, N, N> {
        let mut result = Mat::<T, N, N>::ZERO;
        let mut i = 0;
        while i < N {
            result.elements[i][i] = T::ONE;
            i += 1;
        }
        result
    }
}

impl<T: MatrixElement, const N: usize> SquareMatrix<N> for Mat<T, N, N> {
    const IDENTITY: Self = helper::identity();
}

#[cfg(test)]
mod ops_test {
    use super::*;
    use crate::math::vec::*;

    #[test]
    fn add() {
        let m1: Mat<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        let m2: Mat<i32, 2, 3> = [[11, 12, 13], [14, 15, 16]].into();
        assert_eq!(m1 + m2, [[12, 14, 16], [18, 20, 22]].into());
    }

    #[test]
    fn sub() {
        let m1: Mat<i32, 2, 3> = [[3, 5, 7], [11, 13, 17]].into();
        let m2: Mat<i32, 2, 3> = [[1, 2, 3], [3, 6, 8]].into();
        assert_eq!(m1 - m2, [[2, 3, 4], [8, 7, 9]].into());
    }

    #[test]
    fn mul1() {
        let m1: Mat<i32, 1, 2> = [[3, 4]].into();
        let m2: Mat<i32, 2, 1> = [[6], [8]].into();
        assert_eq!(m1 * m2, [[50]].into());
    }

    #[test]
    fn mul2() {
        let m1: Mat<i32, 2, 1> = [[1], [2]].into();
        let m2: Mat<i32, 1, 3> = [[3, 4, 5]].into();
        assert_eq!(m1 * m2, [[3, 4, 5], [6, 8, 10]].into());
    }

    #[test]
    fn mul3() {
        let m1: Mat<i32, 2, 2> = [[1, 2], [3, 4]].into();
        let m2: Mat<i32, 2, 2> = [[5, 6], [7, 8]].into();
        assert_eq!(m1 * m2, [[19, 22], [43, 50]].into());
    }

    #[test]
    fn mul4() {
        let m1: Mat<i32, 3, 2> = [[0, 1], [2, 3], [4, 5]].into();
        let m2: Mat<i32, 2, 4> = [[0, 1, 2, 3], [4, 5, 6, 7]].into();
        assert_eq!(
            m1 * m2,
            [[4, 5, 6, 7], [12, 17, 22, 27], [20, 29, 38, 47]].into()
        );
    }

    #[test]
    fn mul_scalar() {
        let m: Mat<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(m * 2, Mat::new([[2, 4, 6], [8, 10, 12]]));
    }

    #[test]
    fn mul_scalar_matrix() {
        let m: Mat<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(2 * m, Mat::new([[2, 4, 6], [8, 10, 12]]));
    }

    #[test]
    fn mul_matrix_vector() {
        let m: Mat<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        let v: Vec3<i32> = [3, 4, 5].into();
        assert_eq!(m * v, Vec2::new(26, 62));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::math::vec::*;

    #[test]
    fn matrix_like_get() {
        let m = Mat::<i32, 2, 3>::new([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(m.elements[0][0], *m.get(0, 0));
        assert_eq!(m.elements[0][1], *m.get(0, 1));
        assert_eq!(m.elements[0][2], *m.get(0, 2));
        assert_eq!(m.elements[1][0], *m.get(1, 0));
        assert_eq!(m.elements[1][1], *m.get(1, 1));
        assert_eq!(m.elements[1][2], *m.get(1, 2));
    }

    #[test]
    fn new_from_vec_array() {
        let m1 = Mat::new([Vec2::new(1, 2), Vec2::new(3, 4), Vec2::new(5, 6)]);
        let m2 = Mat::<i32, 3, 2> {
            elements: [[1, 2], [3, 4], [5, 6]],
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn new_from_array() {
        let m1 = Mat::new([[1, 2], [3, 4], [5, 6]]);
        let m2 = Mat::<i32, 3, 2> {
            elements: [[1, 2], [3, 4], [5, 6]],
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn from_vector_array() {
        let m1 = Mat::from([Vec2::new(1, 2), Vec2::new(3, 4), Vec2::new(5, 6)]);
        let m2 = Mat::<i32, 3, 2> {
            elements: [[1, 2], [3, 4], [5, 6]],
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn from_array() {
        let m1 = Mat::<i32, 3, 2>::from([[1, 2], [3, 4], [5, 6]]);
        let m2 = Mat::<i32, 3, 2> {
            elements: [[1, 2], [3, 4], [5, 6]],
        };
        assert_eq!(m1, m2);
    }

    #[test]
    fn row() {
        let m: Mat<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(m.row(0), [1, 4]);
        assert_eq!(m.row(1), [2, 5]);
        assert_eq!(m.row(2), [3, 6]);
    }

    #[test]
    #[should_panic]
    fn row_out_of_range() {
        let m: Mat<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        let _ = m.row(3);
    }

    #[test]
    fn col() {
        let m: Mat<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(m.col(0), [1, 2, 3]);
        assert_eq!(m.col(1), [4, 5, 6]);
    }

    #[test]
    #[should_panic]
    fn col_out_of_range() {
        let m: Mat<i32, 2, 3> = [[1, 2, 3], [4, 5, 6]].into();
        let _ = m.col(3);
    }

    #[test]
    fn identity_mut() {
        let m: Mat<i32, 2, 2> = [[1, 2], [3, 4]].into();
        assert_eq!(m * Mat::IDENTITY, Mat::IDENTITY * m);
    }

    #[test]
    fn identity() {
        let m: Mat<i32, 2, 2> = [[1, 0], [0, 1]].into();
        assert_eq!(m, Mat::IDENTITY);

        let m: Mat<i32, 3, 3> = [[1, 0, 0], [0, 1, 0], [0, 0, 1]].into();
        assert_eq!(m, Mat::IDENTITY);
    }
}
