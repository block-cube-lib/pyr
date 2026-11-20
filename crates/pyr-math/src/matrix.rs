use crate::ApproxEq;
use crate::scalar::{FloatScalar, Scalar};
use crate::vector::Vec4;
use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Mat4<T: Scalar> {
    pub cols: [Vec4<T>; 4],
}

impl<T: Scalar> Mat4<T> {
    pub const IDENTITY: Self = Self {
        cols: [
            Vec4::new(T::ONE, T::ZERO, T::ZERO, T::ZERO),
            Vec4::new(T::ZERO, T::ONE, T::ZERO, T::ZERO),
            Vec4::new(T::ZERO, T::ZERO, T::ONE, T::ZERO),
            Vec4::new(T::ZERO, T::ZERO, T::ZERO, T::ONE),
        ],
    };

    pub const ZERO: Self = Self {
        cols: [
            Vec4::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO),
            Vec4::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO),
            Vec4::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO),
            Vec4::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO),
        ],
    };

    pub const fn new(cols: [Vec4<T>; 4]) -> Self {
        Self { cols }
    }

    pub const fn from_cols(c0: Vec4<T>, c1: Vec4<T>, c2: Vec4<T>, c3: Vec4<T>) -> Self {
        Self {
            cols: [c0, c1, c2, c3],
        }
    }

    pub const fn from_rows(r0: Vec4<T>, r1: Vec4<T>, r2: Vec4<T>, r3: Vec4<T>) -> Self {
        Self {
            cols: [
                Vec4::new(r0.x, r1.x, r2.x, r3.x),
                Vec4::new(r0.y, r1.y, r2.y, r3.y),
                Vec4::new(r0.z, r1.z, r2.z, r3.z),
                Vec4::new(r0.w, r1.w, r2.w, r3.w),
            ],
        }
    }

    pub const fn col(&self, index: usize) -> Vec4<T> {
        self.cols[index]
    }

    pub fn row(&self, index: usize) -> Vec4<T> {
        Vec4::new(
            self.cols[0][index],
            self.cols[1][index],
            self.cols[2][index],
            self.cols[3][index],
        )
    }

    #[inline]
    fn compute_subfactors(&self) -> SubFactors<T> {
        let s0 = self.cols[0][0] * self.cols[1][1] - self.cols[1][0] * self.cols[0][1];
        let s1 = self.cols[0][0] * self.cols[1][2] - self.cols[1][0] * self.cols[0][2];
        let s2 = self.cols[0][0] * self.cols[1][3] - self.cols[1][0] * self.cols[0][3];
        let s3 = self.cols[0][1] * self.cols[1][2] - self.cols[1][1] * self.cols[0][2];
        let s4 = self.cols[0][1] * self.cols[1][3] - self.cols[1][1] * self.cols[0][3];
        let s5 = self.cols[0][2] * self.cols[1][3] - self.cols[1][2] * self.cols[0][3];

        let c5 = self.cols[2][2] * self.cols[3][3] - self.cols[3][2] * self.cols[2][3];
        let c4 = self.cols[2][1] * self.cols[3][3] - self.cols[3][1] * self.cols[2][3];
        let c3 = self.cols[2][1] * self.cols[3][2] - self.cols[3][1] * self.cols[2][2];
        let c2 = self.cols[2][0] * self.cols[3][3] - self.cols[3][0] * self.cols[2][3];
        let c1 = self.cols[2][0] * self.cols[3][2] - self.cols[3][0] * self.cols[2][2];
        let c0 = self.cols[2][0] * self.cols[3][1] - self.cols[3][0] * self.cols[2][1];

        SubFactors {
            s: [s0, s1, s2, s3, s4, s5],
            c: [c0, c1, c2, c3, c4, c5],
        }
    }

    pub fn determinant(&self) -> T {
        let f = self.compute_subfactors();
        f.s[0] * f.c[5] - f.s[1] * f.c[4] + f.s[2] * f.c[3] + f.s[3] * f.c[2] - f.s[4] * f.c[1]
            + f.s[5] * f.c[0]
    }
}

impl<T: Scalar> std::convert::From<[Vec4<T>; 4]> for Mat4<T> {
    fn from(cols: [Vec4<T>; 4]) -> Self {
        Self { cols }
    }
}

impl<T: Scalar> std::convert::From<[[T; 4]; 4]> for Mat4<T> {
    fn from(array: [[T; 4]; 4]) -> Self {
        let cols = [
            Vec4::from(array[0]),
            Vec4::from(array[1]),
            Vec4::from(array[2]),
            Vec4::from(array[3]),
        ];
        Self { cols }
    }
}

impl<T: Scalar> std::convert::From<[(T, T, T, T); 4]> for Mat4<T> {
    fn from(tuple_array: [(T, T, T, T); 4]) -> Self {
        let cols = [
            Vec4::from(tuple_array[0]),
            Vec4::from(tuple_array[1]),
            Vec4::from(tuple_array[2]),
            Vec4::from(tuple_array[3]),
        ];
        Self { cols }
    }
}

impl<T: Scalar> std::convert::From<((T, T, T, T), (T, T, T, T), (T, T, T, T), (T, T, T, T))>
    for Mat4<T>
{
    fn from(tuple_tuple: ((T, T, T, T), (T, T, T, T), (T, T, T, T), (T, T, T, T))) -> Self {
        let cols = [
            Vec4::from(tuple_tuple.0),
            Vec4::from(tuple_tuple.1),
            Vec4::from(tuple_tuple.2),
            Vec4::from(tuple_tuple.3),
        ];
        Self { cols }
    }
}

//
// ops
//

impl<T: Scalar> Add for Mat4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_cols(
            self.cols[0] + rhs.cols[0],
            self.cols[1] + rhs.cols[1],
            self.cols[2] + rhs.cols[2],
            self.cols[3] + rhs.cols[3],
        )
    }
}

impl<T: Scalar> Sub for Mat4<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_cols(
            self.cols[0] - rhs.cols[0],
            self.cols[1] - rhs.cols[1],
            self.cols[2] - rhs.cols[2],
            self.cols[3] - rhs.cols[3],
        )
    }
}

impl<T: Scalar> Mul for Mat4<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let rhs_rows = [rhs.row(0), rhs.row(1), rhs.row(2), rhs.row(3)];
        let mut result = Self::ZERO;
        for i in 0..4 {
            let col = self.cols[i];
            let new_col = Vec4::new(
                col.dot(rhs_rows[0]),
                col.dot(rhs_rows[1]),
                col.dot(rhs_rows[2]),
                col.dot(rhs_rows[3]),
            );
            result.cols[i] = new_col;
        }
        result
    }
}

impl<T: Scalar> Mul<Vec4<T>> for Mat4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        Vec4::new(
            self.cols[0].dot(rhs),
            self.cols[1].dot(rhs),
            self.cols[2].dot(rhs),
            self.cols[3].dot(rhs),
        )
    }
}

impl<T: Scalar> Mul<T> for Mat4<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::from_cols(
            self.cols[0] * rhs,
            self.cols[1] * rhs,
            self.cols[2] * rhs,
            self.cols[3] * rhs,
        )
    }
}

impl<T: Scalar> MulAssign<T> for Mat4<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.cols[0] *= rhs;
        self.cols[1] *= rhs;
        self.cols[2] *= rhs;
        self.cols[3] *= rhs;
    }
}

impl<T: Scalar> Div<T> for Mat4<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::from_cols(
            self.cols[0] / rhs,
            self.cols[1] / rhs,
            self.cols[2] / rhs,
            self.cols[3] / rhs,
        )
    }
}

impl<T: Scalar> DivAssign<T> for Mat4<T> {
    fn div_assign(&mut self, rhs: T) {
        self.cols[0] /= rhs;
        self.cols[1] /= rhs;
        self.cols[2] /= rhs;
        self.cols[3] /= rhs;
    }
}

// index
impl<T: Scalar> Index<usize> for Mat4<T> {
    type Output = Vec4<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cols[index]
    }
}

impl<T: Scalar> IndexMut<usize> for Mat4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cols[index]
    }
}

impl<T: Scalar> Neg for Mat4<T>
where
    Vec4<T>: Neg<Output = Vec4<T>>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_cols(-self.cols[0], -self.cols[1], -self.cols[2], -self.cols[3])
    }
}

impl<T: FloatScalar> ApproxEq for Mat4<T> {
    type Epsilon = T;

    fn approx_eq(&self, other: &Self) -> bool {
        self.cols[0].approx_eq(&other.cols[0])
            && self.cols[1].approx_eq(&other.cols[1])
            && self.cols[2].approx_eq(&other.cols[2])
            && self.cols[3].approx_eq(&other.cols[3])
    }

    fn approx_eq_eps(&self, other: &Self, epsilon: T) -> bool {
        self.cols[0].approx_eq_eps(&other.cols[0], epsilon)
            && self.cols[1].approx_eq_eps(&other.cols[1], epsilon)
            && self.cols[2].approx_eq_eps(&other.cols[2], epsilon)
            && self.cols[3].approx_eq_eps(&other.cols[3], epsilon)
    }
}

struct SubFactors<T> {
    s: [T; 6],
    c: [T; 6],
}

impl<T: FloatScalar> Mat4<T> {
    pub fn ininverse(&self) -> Option<Self> {
        let f = self.compute_subfactors();
        let det = f.s[0] * f.c[5] - f.s[1] * f.c[4] + f.s[2] * f.c[3] + f.s[3] * f.c[2]
            - f.s[4] * f.c[1]
            + f.s[5] * f.c[0];

        if det.approx_eq(&T::ZERO) {
            return None;
        }

        let inv_det = T::ONE / det;
        Some(Self::new([
            Vec4::new(
                (self.cols[1][1] * f.c[5] - self.cols[1][2] * f.c[4] + self.cols[1][3] * f.c[3])
                    * inv_det,
                (-self.cols[0][1] * f.c[5] + self.cols[0][2] * f.c[4] - self.cols[0][3] * f.c[3])
                    * inv_det,
                (self.cols[3][1] * f.s[5] - self.cols[3][2] * f.s[4] + self.cols[3][3] * f.s[3])
                    * inv_det,
                (-self.cols[2][1] * f.s[5] + self.cols[2][2] * f.s[4] - self.cols[2][3] * f.s[3])
                    * inv_det,
            ),
            Vec4::new(
                (-self.cols[1][0] * f.c[5] + self.cols[1][2] * f.c[2] - self.cols[1][3] * f.c[1])
                    * inv_det,
                (self.cols[0][0] * f.c[5] - self.cols[0][2] * f.c[2] + self.cols[0][3] * f.c[1])
                    * inv_det,
                (-self.cols[3][0] * f.s[5] + self.cols[3][2] * f.s[2] - self.cols[3][3] * f.s[1])
                    * inv_det,
                (self.cols[2][0] * f.s[5] - self.cols[2][2] * f.s[2] + self.cols[2][3] * f.s[1])
                    * inv_det,
            ),
            Vec4::new(
                (self.cols[1][0] * f.c[4] - self.cols[1][1] * f.c[2] + self.cols[1][3] * f.c[0])
                    * inv_det,
                (-self.cols[0][0] * f.c[4] + self.cols[0][1] * f.c[2] - self.cols[0][3] * f.c[0])
                    * inv_det,
                (self.cols[3][0] * f.s[4] - self.cols[3][1] * f.s[2] + self.cols[3][3] * f.s[0])
                    * inv_det,
                (-self.cols[2][0] * f.s[4] + self.cols[2][1] * f.s[2] - self.cols[2][3] * f.s[0])
                    * inv_det,
            ),
            Vec4::new(
                (-self.cols[1][0] * f.c[3] + self.cols[1][1] * f.c[1] - self.cols[1][2] * f.c[0])
                    * inv_det,
                (self.cols[0][0] * f.c[3] - self.cols[0][1] * f.c[1] + self.cols[0][2] * f.c[0])
                    * inv_det,
                (-self.cols[3][0] * f.s[3] + self.cols[3][1] * f.s[1] - self.cols[3][2] * f.s[0])
                    * inv_det,
                (self.cols[2][0] * f.s[3] - self.cols[2][1] * f.s[1] + self.cols[2][2] * f.s[0])
                    * inv_det,
            ),
        ]))
    }
}
