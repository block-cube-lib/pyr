mod traits;

pub use self::traits::*;
use crate::math::{
    quat::*,
    vec::{traits::*, *},
};
use crate::num::*;
use num::Float;
use std::ops::*;

/// Matrix type.
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

/// ```
/// use pyr::math::Mat;
/// use pyr::num::Zero;;
/// assert_eq!(Mat::<i32, 3, 2>::ZERO, Mat::<i32, 3, 2> { elements: [[0, 0], [0, 0], [0, 0]] });
/// ```
impl<T: MatrixElement, const ROW: usize, const COL: usize> Zero for Mat<T, ROW, COL> {
    const ZERO: Self = Self {
        elements: [[T::ZERO; COL]; ROW],
    };

    fn is_near_zero(&self) -> bool {
        for r in 0..ROW {
            for c in 0..COL {
                if !self.elements[r][c].is_near_zero() {
                    return false;
                }
            }
        }
        true
    }
}

/// ```
/// use pyr::math::Mat;
/// use pyr::num::One;
/// assert_eq!(Mat::<i32, 3, 2>::ONE, Mat::<i32, 3, 2> { elements: [[1, 1], [1, 1], [1, 1]] });
/// ```
impl<T: MatrixElement, const ROW: usize, const COL: usize> One for Mat<T, ROW, COL> {
    const ONE: Self = Self {
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
        for (i, v_elem) in v.iter_mut().enumerate().take(ROW) {
            *v_elem = self.elements[i][n];
        }
        v
    }

    pub fn transposed(self) -> Mat<T, COL, ROW> {
        let mut m = Mat::<T, COL, ROW>::ZERO;
        for r in 0..ROW {
            for c in 0..COL {
                m.elements[c][r] = self.elements[r][c];
            }
        }
        m
    }

    pub fn as_float_mat(&self) -> Mat<<T as crate::num::AsFloatingPoint>::Output, ROW, COL>
    where
        <T as AsFloatingPoint>::Output: MatrixElement,
    {
        let mut m = Mat::ZERO;
        for r in 0..ROW {
            for c in 0..COL {
                m[r][c] = self.elements[r][c].as_floating_point();
            }
        }
        m
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

    fn transpose(&self) -> Self {
        let mut m = Self::ZERO;
        for r in 0..N / 2 {
            for c in 0..N / 2 {
                m.elements[c][r] = self.elements[r][c];
            }
        }
        m
    }
}

impl<T: MatrixElement> Determinant<1> for Mat<T, 1, 1> {
    fn determinant(&self) -> T {
        *self.get(0, 0)
    }
}

impl<T: MatrixElement> Determinant<2> for Mat<T, 2, 2> {
    fn determinant(&self) -> T {
        self.elements[0][0] * self.elements[1][1] - self.elements[0][1] * self.elements[1][0]
    }
}

impl<T: MatrixElement> Adjugate<2, 1> for Mat<T, 2, 2> {
    type Output = Mat<T, 1, 1>;

    fn adjugate(&self, r: usize, c: usize) -> Self::Output {
        match (r, c) {
            (0, 0) => Mat::new([[self.elements[1][1]]]),
            (0, 1) => Mat::new([[self.elements[1][0]]]),
            (1, 0) => Mat::new([[self.elements[0][1]]]),
            (1, 1) => Mat::new([[self.elements[0][0]]]),
            _ => panic!("out of range"),
        }
    }
}

impl<T: MatrixElement> Adjugate<3, 2> for Mat<T, 3, 3> {
    type Output = Mat<T, 2, 2>;
}

impl<T: MatrixElement> Cofactor<3> for Mat<T, 3, 3>
where
    T: std::ops::Neg<Output = T>,
{
    fn cofactor(&self, r: usize, c: usize) -> T {
        let adj = self.adjugate(r, c);
        let sign = if (r + c) % 2 == 0 { T::ONE } else { -T::ONE };
        sign * adj.determinant()
    }
}

impl<T: MatrixElement> Determinant<3> for Mat<T, 3, 3>
where
    T: std::ops::Neg<Output = T>,
{
    fn determinant(&self) -> T {
        let mut det = T::ZERO;
        for c in 0..3 {
            det += self.elements[0][c] * self.cofactor(0, c);
        }
        det
    }
}

impl<T: MatrixElement> Adjugate<4, 3> for Mat<T, 4, 4> {
    type Output = Mat<T, 3, 3>;
}

impl<T: MatrixElement> Cofactor<4> for Mat<T, 4, 4>
where
    T: std::ops::Neg<Output = T>,
{
    fn cofactor(&self, r: usize, c: usize) -> T {
        let adj = self.adjugate(r, c);
        let sign = if (r + c) % 2 == 0 { T::ONE } else { -T::ONE };
        sign * adj.determinant()
    }
}

impl<T: MatrixElement> Determinant<4> for Mat<T, 4, 4>
where
    T: std::ops::Neg<Output = T>,
{
    fn determinant(&self) -> T {
        let mut det = T::ZERO;
        for c in 0..4 {
            det += self.elements[0][c] * self.cofactor(0, c);
        }
        det
    }
}

impl<T: MatrixElement> Inverse<2> for Mat<T, 2, 2>
where
    <T as AsFloatingPoint>::Output: FloatMatrixElement,
{
    type Output = Mat<<T as AsFloatingPoint>::Output, 2, 2>;

    fn inverse(&self) -> Option<Self::Output> {
        let det = self.determinant().as_floating_point();
        if det.is_near_zero() {
            return None;
        }
        let inv_det = <T as AsFloatingPoint>::Output::ONE / det;
        let mut m = Mat::ZERO;
        m.elements[0][0] = self.elements[1][1].as_floating_point() * inv_det;
        m.elements[0][1] = -self.elements[0][1].as_floating_point() * inv_det;
        m.elements[1][0] = -self.elements[1][0].as_floating_point() * inv_det;
        m.elements[1][1] = self.elements[0][0].as_floating_point() * inv_det;
        Some(m)
    }
}

impl<T: MatrixElement> Inverse<3> for Mat<T, 3, 3>
where
    T: std::ops::Neg<Output = T>,
    <T as AsFloatingPoint>::Output: FloatMatrixElement,
{
    type Output = Mat<<T as AsFloatingPoint>::Output, 3, 3>;

    fn inverse(&self) -> Option<Self::Output> {
        let fm = self.as_float_mat();
        let det = fm.determinant();
        if det.is_near_zero() {
            return None;
        }
        let inv_det = <T as AsFloatingPoint>::Output::ONE / det;
        let mut m = Mat::ZERO;
        for r in 0..3 {
            for c in 0..3 {
                m.elements[c][r] = fm.cofactor(r, c) * inv_det;
            }
        }
        Some(m)
    }
}

impl<T: MatrixElement> Inverse<4> for Mat<T, 4, 4>
where
    T: std::ops::Neg<Output = T>,
    <T as AsFloatingPoint>::Output: FloatMatrixElement,
{
    type Output = Mat<<T as AsFloatingPoint>::Output, 4, 4>;

    fn inverse(&self) -> Option<Self::Output> {
        let fm = self.as_float_mat();
        let det = fm.determinant();
        if det.is_near_zero() {
            return None;
        }
        let inv_det = <T as AsFloatingPoint>::Output::ONE / det;
        let mut m = Mat::ZERO;
        for r in 0..4 {
            for c in 0..4 {
                m.elements[c][r] = fm.cofactor(r, c) * inv_det;
            }
        }
        Some(m)
    }
}

impl<T: MatrixElement> Mat<T, 4, 4> {
    /// ```
    /// use pyr::math::Mat;
    /// let m = Mat::<i32, 4, 4>::translate([2, 3, 4]);
    /// assert_eq!(m, Mat::<i32, 4, 4>::new([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [2, 3, 4, 1]]));
    /// ```
    pub fn translate(v: impl VectorLike<3, ElementType = T>) -> Self {
        let mut m = Self::IDENTITY;
        m.elements[3][0] = *v.get(0);
        m.elements[3][1] = *v.get(1);
        m.elements[3][2] = *v.get(2);
        m
    }

    pub fn rotate_x(redian: T) -> Self
    where
        T: FloatMatrixElement,
    {
        let c = redian.cos();
        let s = redian.sin();
        Self::new([
            [T::ONE, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, c, s, T::ZERO],
            [T::ZERO, -s, c, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }

    pub fn rotate_y(radian: T) -> Self
    where
        T: FloatMatrixElement,
    {
        let c = radian.cos();
        let s = radian.sin();
        Self::new([
            [c, T::ZERO, -s, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO, T::ZERO],
            [s, T::ZERO, c, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }

    pub fn rotate_z(radian: T) -> Self
    where
        T: FloatMatrixElement,
    {
        let c = radian.cos();
        let s = radian.sin();
        Self::new([
            [c, s, T::ZERO, T::ZERO],
            [-s, c, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }

    pub fn rotate_axis(axis: impl VectorLike<3, ElementType = T>, radian: T) -> Self
    where
        T: FloatMatrixElement,
        <T as AsFloatingPoint>::Output: FloatVectorElement,
    {
        use crate::math::vec::Vec3;
        let n = Vec3::from_vector(axis).normalized();
        let c = radian.cos();
        let s = radian.sin();
        let t = T::ONE - c;
        let x = *n.get(0);
        let y = *n.get(1);
        let z = *n.get(2);
        let tx = t * x;
        let ty = t * y;
        let tz = t * z;
        let sx = s * x;
        let sy = s * y;
        let sz = s * z;
        let txy = tx * y;
        let txz = tx * z;
        let tyz = ty * z;
        Self::new([
            [tx * x + c, txy + sz, txz - sy, T::ZERO],
            [txy - sz, ty * y + c, tyz + sx, T::ZERO],
            [txz + sy, tyz - sx, tz * z + c, T::ZERO],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }

    pub fn rotate(quat: Quat<T>) -> Self
    where
        T: QuatElement,
    {
        let two = T::ONE + T::ONE;
        let ww = two * quat.w;
        let xx = two * quat.x;
        let yy = two * quat.y;
        let zz = two * quat.z;
        Self::new([
            [
                T::ONE - yy * quat.y - zz * quat.z,
                xx * quat.y + ww * quat.z,
                xx * quat.z - ww * quat.y,
                T::ZERO,
            ],
            [
                xx * quat.y - ww * quat.z,
                T::ONE - xx * quat.x - zz * quat.z,
                yy * quat.z + ww * quat.x,
                T::ZERO,
            ],
            [
                xx * quat.z + ww * quat.y,
                yy * quat.z - ww * quat.x,
                T::ONE - xx * quat.x - yy * quat.y,
                T::ZERO,
            ],
            [T::ZERO, T::ZERO, T::ZERO, T::ONE],
        ])
    }

    pub fn perspective(fov_y: T, aspect: T, near: T, far: T) -> Self
    where
        T: FloatMatrixElement,
        <T as AsFloatingPoint>::Output: FloatVectorElement,
    {
        let f = fov_y.tan().recip();
        let r = far / (far - near);
        Self::new([
            [f / aspect, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, f, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, r, T::ONE],
            [T::ZERO, T::ZERO, -near * r, T::ZERO],
        ])
    }

    pub fn orthographic(width: T, height: T, near: T, far: T) -> Self
    where
        T: FloatMatrixElement,
        <T as AsFloatingPoint>::Output: FloatVectorElement,
    {
        let r = (far - near).recip();
        let two = T::ONE + T::ONE;
        Self::new([
            [two / width, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, two / height, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, r, T::ZERO],
            [T::ZERO, T::ZERO, -near * r, T::ONE],
        ])
    }

    pub fn look_at(
        eye: impl VectorLike<3, ElementType = T>,
        target: impl VectorLike<3, ElementType = T>,
        up: impl VectorLike<3, ElementType = T>,
    ) -> Self
    where
        T: FloatMatrixElement,
        <T as AsFloatingPoint>::Output: FloatVectorElement,
    {
        let eye = Vec3::<T>::from_vector(eye);
        let target = Vec3::<T>::from_vector(target);
        let f = (target - eye).normalized();
        let r = f.cross(up).normalized();
        let u = r.cross(f);
        Self::new([
            [r.x, u.x, -f.x, T::ZERO],
            [r.y, u.y, -f.y, T::ZERO],
            [r.z, u.z, -f.z, T::ZERO],
            [-r.dot(eye), -u.dot(eye), f.dot(eye), T::ONE],
        ])
    }
}

#[cfg(test)]
mod ops_test {
    use super::*;

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

    fn format_mat<T: MatrixElement, const ROW: usize, const COL: usize>(
        m: Mat<T, ROW, COL>,
    ) -> String
    where
        T: std::fmt::Display,
    {
        let mut s = String::new();
        s.push('[');
        for r in 0..ROW {
            s.push('[');
            for c in 0..COL {
                s.push_str(&format!("{} ", m.elements[r][c]));
                if c < COL - 1 {
                    s.push(',');
                }
            }
            s.push(']');
            if r < ROW - 1 {
                s.push('\n');
            }
        }
        s.push(']');
        s
    }

    fn mat_near_eq<T: MatrixElement, const ROW: usize, const COL: usize>(
        m1: Mat<T, ROW, COL>,
        m2: Mat<T, ROW, COL>,
    ) -> bool {
        for r in 0..ROW {
            for c in 0..COL {
                if !(m1.elements[r][c] - m2.elements[r][c]).is_near_zero() {
                    return false;
                }
            }
        }
        true
    }

    macro_rules! assert_mat_near_eq {
        ($m1:expr, $m2:expr) => {
            let m1 = $m1;
            let m2 = $m2;
            assert!(
                mat_near_eq(m1, m2),
                "{}\nnot equals\n{}",
                format_mat(m1),
                format_mat(m2)
            );
        };
    }

    fn vec3_near_eq<T: VectorElement>(v1: Vec3<T>, v2: Vec3<T>) -> bool {
        for i in 0..3 {
            if !(v1[i] - v2[i]).is_near_zero() {
                return false;
            }
        }
        true
    }

    macro_rules! assert_vec3_near_eq {
        ($v1:expr, $v2:expr) => {
            let v1 = $v1;
            let v2 = $v2;
            assert!(vec3_near_eq(v1, v2), "{v1} != {v2}");
        };
    }

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

    #[test]
    fn adjugate2x2() {
        let m: Mat<i32, 2, 2> = [[1, 2], [3, 4]].into();
        assert_eq!(m.adjugate(0, 0), [[4]].into());
        assert_eq!(m.adjugate(0, 1), [[3]].into());
        assert_eq!(m.adjugate(1, 0), [[2]].into());
        assert_eq!(m.adjugate(1, 1), [[1]].into());
    }

    #[test]
    fn adjugate3x3() {
        let m: Mat<i32, 3, 3> = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].into();
        assert_eq!(m.adjugate(0, 0), [[5, 6], [8, 9]].into());
        assert_eq!(m.adjugate(0, 1), [[4, 6], [7, 9]].into());
        assert_eq!(m.adjugate(0, 2), [[4, 5], [7, 8]].into());
        assert_eq!(m.adjugate(1, 0), [[2, 3], [8, 9]].into());
        assert_eq!(m.adjugate(1, 1), [[1, 3], [7, 9]].into());
        assert_eq!(m.adjugate(1, 2), [[1, 2], [7, 8]].into());
        assert_eq!(m.adjugate(2, 0), [[2, 3], [5, 6]].into());
        assert_eq!(m.adjugate(2, 1), [[1, 3], [4, 6]].into());
        assert_eq!(m.adjugate(2, 2), [[1, 2], [4, 5]].into());
    }

    #[test]
    fn cofactor3x3() {
        let m = Mat::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(m.cofactor(1, 1), -12);
        assert_eq!(m.cofactor(1, 2), 6);
    }

    #[test]
    fn determinant1x1() {
        let m = Mat::new([[1]]);
        assert_eq!(m.determinant(), 1);
    }

    #[test]
    fn determinant2x2() {
        let m = Mat::new([[1, 2], [3, 4]]);
        assert_eq!(m.determinant(), -2);
    }

    #[test]
    fn determinant3x3() {
        let m = Mat::new([[1, -3, 8], [9, 2, -2], [7, 6, 4]]);
        assert_eq!(m.determinant(), 490);
    }

    #[test]
    fn determinant4x4() {
        let m = Mat::new([[1, 7, 2, 4], [1, 5, 2, 4], [3, 0, 1, 0], [2, 1, 5, -3]]);
        assert_eq!(m.determinant(), -134);
    }

    #[test]
    fn inverse2x2() {
        let m = Mat::<i32, 2, 2>::new([[1, 2], [3, 4]]);
        let inv = m.inverse().unwrap();
        assert_eq!(m.as_float_mat() * inv, Mat::IDENTITY);
        assert_eq!(inv * m.as_float_mat(), Mat::IDENTITY);

        let zero = Mat::<f32, 2, 2>::ZERO;
        assert_eq!(zero.inverse(), None);
    }

    #[test]
    fn inverse3x3() {
        let m = Mat::<i64, 3, 3>::new([[2, 3, 5], [4, 5, 6], [7, 8, 9]]).as_float_mat();
        let inv = m.inverse().unwrap();
        assert_mat_near_eq!(m * inv, Mat::IDENTITY);
        assert_mat_near_eq!(inv * m, Mat::IDENTITY);

        let zero = Mat::<f32, 3, 3>::ZERO;
        assert_eq!(zero.inverse(), None);
    }

    #[test]
    fn inverse4x4() {
        let m = Mat::<i64, 4, 4>::new([[1, 2, 7, -7], [4, -4, 2, 4], [-9, 2, -6, 2], [8, 2, 0, 2]])
            .as_float_mat();
        let inv = m.inverse().unwrap();
        assert!(mat_near_eq(m * inv, Mat::IDENTITY));
        assert!(mat_near_eq(inv * m, Mat::IDENTITY));

        let zero = Mat::<f32, 3, 3>::ZERO;
        assert_eq!(zero.inverse(), None);
    }

    #[test]
    fn mat4x4_rotate_x() {
        let m = Mat::<f32, 4, 4>::rotate_x(std::f32::consts::FRAC_PI_2);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let rot_v = m * v;
        assert!(
            -0.00001 < rot_v.x && rot_v.x < 0.00001,
            "rot_v.x is {}",
            rot_v.x
        );
        assert!(
            -0.00001 < rot_v.y && rot_v.y < 0.00001,
            "rot_v.y is {}",
            rot_v.y
        );
        assert!(
            -1.00001 < rot_v.z && rot_v.z < -0.99999,
            "rot_v.z is {}",
            rot_v.z
        );
    }

    #[test]
    fn mat4x4_rotate_y() {
        let m = Mat::<f32, 4, 4>::rotate_y(std::f32::consts::FRAC_PI_2);
        let v = Vec3::new(0.0, 0.0, 1.0);
        let rot_v = m * v;
        assert!(
            -1.00001 < rot_v.x && rot_v.x < -0.99999,
            "rot_v.x is {}",
            rot_v.x
        );
        assert!(
            -0.00001 < rot_v.y && rot_v.y < 0.00001,
            "rot_v.y is {}",
            rot_v.y
        );
        assert!(
            -0.00001 < rot_v.z && rot_v.z < 0.00001,
            "rot_v.z is {}",
            rot_v.z
        );
    }

    #[test]
    fn mat4x4_rotate_z() {
        let m = Mat::<f32, 4, 4>::rotate_z(std::f32::consts::FRAC_PI_2);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let rot_v = m * v;
        assert!(
            0.99999 < rot_v.x && rot_v.x < 1.00001,
            "rot_v.x is {}",
            rot_v.x
        );
        assert!(
            -0.00001 < rot_v.y && rot_v.y < 0.00001,
            "rot_v.y is {}",
            rot_v.y
        );
        assert!(
            -0.00001 < rot_v.z && rot_v.z < 0.00001,
            "rot_v.z is {}",
            rot_v.z
        );
    }

    #[test]
    fn mat4x4_rotate_axis() {
        {
            let m1 = Mat::<f64, 4, 4>::rotate_axis(Vec3::UNIT_X, std::f64::consts::FRAC_PI_2);
            let m2 = Mat::<f64, 4, 4>::rotate_x(std::f64::consts::FRAC_PI_2);
            assert!(mat_near_eq(m1, m2), "{:?} != {:?}", m1, m2);
        }
        {
            let m1 = Mat::<f64, 4, 4>::rotate_axis(Vec3::UNIT_Y, std::f64::consts::FRAC_PI_2);
            let m2 = Mat::<f64, 4, 4>::rotate_y(std::f64::consts::FRAC_PI_2);
            assert!(mat_near_eq(m1, m2), "{:?} != {:?}", m1, m2);
        }
        {
            let m1 = Mat::<f64, 4, 4>::rotate_axis(Vec3::UNIT_Z, std::f64::consts::FRAC_PI_2);
            let m2 = Mat::<f64, 4, 4>::rotate_z(std::f64::consts::FRAC_PI_2);
            assert!(mat_near_eq(m1, m2), "{:?} != {:?}", m1, m2);
        }
        {
            let m = Mat::<f64, 4, 4>::rotate_axis(Vec3::new(1.0, 1.0, 0.0), std::f64::consts::PI);
            let v = Vec3::new(1.0, 0.0, 0.0);
            assert_vec3_near_eq!(v * m, Vec3::UNIT_Y);
        }
    }

    #[test]
    fn mat4x4_rotate_quat() {
        let n = Vec3::new(1.0, 1.0, 0.5).normalized();
        let angle = 30.0_f64.to_radians();
        let q = Quat::<f64>::rotate_axis_angle(n, angle);
        let m1 = Mat::<f64, 4, 4>::rotate(q);
        let m2 = Mat::<f64, 4, 4>::rotate_axis(n, angle);
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_vec3_near_eq!(v * m1, v * m2);
        let zero = Vec3::ZERO;
        assert_vec3_near_eq!(zero * m1, zero);
        assert_vec3_near_eq!(zero * m2, zero);
    }

    #[test]
    fn mat4x4_perspective() {
        let aspect = 16.0 / 9.0;
        let near = 10.0;
        let far = 20.0;
        let m = Mat::<f32, 4, 4>::perspective(60.0_f32.to_radians(), aspect, near, far);
        {
            let v = Vec4::new(0.0, 0.0, 15.0, 1.0) * m;
            let v = Vec3::new(v.x, v.y, v.z) / v.w;
            assert!(v.x < 0.00001);
            assert!(-0.00001 < v.x);
            assert!(v.y < 0.00001);
            assert!(-0.00001 < v.y);
            assert!(v.z <= 1.0);
            assert!(0.0 <= v.z);
        }
        {
            let v = Vec4::new(0.0, 0.0, far, 1.0) * m;
            let v = Vec3::new(v.x, v.y, v.z) / v.w;
            assert!(v.x < 0.00001);
            assert!(-0.00001 < v.x);
            assert!(v.y < 0.00001);
            assert!(-0.00001 < v.y);
            assert!(v.z <= 1.00001);
            assert!(0.999999 <= v.z);
        }
        {
            let v = Vec4::new(0.0, 0.0, near, 1.0) * m;
            let v = Vec3::new(v.x, v.y, v.z) / v.w;
            assert!(v.x < 0.00001);
            assert!(-0.00001 < v.x);
            assert!(v.y < 0.00001);
            assert!(-0.00001 < v.y);
            assert!(v.z <= 0.00001);
            assert!(-0.00001 <= v.z);
        }
    }

    #[test]
    fn mat4x4_orthographic() {
        let width = 800.0;
        let height = 600.0;
        let near = 10.0;
        let far = 20.0;
        let m = Mat::<f32, 4, 4>::orthographic(width, height, near, far);
        {
            let v = Vec4::new(width / 2.0, height / 2.0, (near + far) / 2.0, 1.0) * m;
            let v = Vec3::new(v.x, v.y, v.z) / v.w;
            assert!(v.x < 1.00001);
            assert!(0.99999 < v.x);
            assert!(v.y < 1.00001);
            assert!(0.99999 < v.y);
            assert!(v.z <= 0.500001);
            assert!(0.499999 <= v.z);
        }
        {
            let v = Vec4::new(-width / 2.0, -height / 2.0, near, 1.0) * m;
            let v = Vec3::new(v.x, v.y, v.z) / v.w;
            assert!(v.x < -0.99999);
            assert!(-1.00001 < v.x);
            assert!(v.y < -0.99999);
            assert!(-1.00001 < v.y);
            assert!(v.z <= 0.00001);
            assert!(-0.00001 <= v.z);
        }
        {
            let v = Vec4::new(0.0, 0.0, far, 1.0) * m;
            let v = Vec3::new(v.x, v.y, v.z) / v.w;
            assert!(v.x < 0.00001);
            assert!(-0.00001 < v.x);
            assert!(v.y < 0.00001);
            assert!(-0.00001 < v.y);
            assert!(v.z <= 1.00001);
            assert!(0.99999 <= v.z);
        }
    }
}
