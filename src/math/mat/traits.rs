use crate::math::vec::traits::*;

pub trait MatrixElement: VectorElement {}
pub trait FloatMatrixElement:
    MatrixElement + num::Float + crate::num::AsFloatingPoint<Output = Self>
{
}

impl<T> MatrixElement for T where T: VectorElement {}
impl<T> FloatMatrixElement for T where
    T: MatrixElement + num::Float + crate::num::AsFloatingPoint<Output = Self>
{
}

pub trait MatrixLike<const ROW: usize, const COL: usize> {
    type ElementType: MatrixElement;

    fn get(&self, row: usize, col: usize) -> &Self::ElementType;
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::ElementType;
}
pub trait FloatMatrixLike<const ROW: usize, const COL: usize>: MatrixLike<ROW, COL>
where
    Self::ElementType: FloatMatrixElement,
{
}

impl<M, const ROW: usize, const COL: usize> FloatMatrixLike<ROW, COL> for M
where
    M: MatrixLike<ROW, COL>,
    M::ElementType: FloatMatrixElement,
{
}

pub trait SquareMatrix<const N: usize>: MatrixLike<N, N> {
    const IDENTITY: Self;

    fn transpose(&self) -> Self;
}
pub trait FloatSquareMatrix<const N: usize>: SquareMatrix<N> {}
impl<M, const N: usize> FloatSquareMatrix<N> for M
where
    M: SquareMatrix<N>,
    M::ElementType: FloatMatrixElement,
{
}

pub trait Determinant<const N: usize>: SquareMatrix<N> {
    fn determinant(&self) -> Self::ElementType;
}

pub trait Adjugate<const N: usize, const M: usize>: SquareMatrix<N> {
    type Output: SquareMatrix<M, ElementType = Self::ElementType>;
    fn adjugate(&self, r: usize, c: usize) -> Self::Output {
        let mut m = Self::Output::IDENTITY;
        let mut i = 0;
        for rr in 0..N {
            if rr == r {
                continue;
            }
            let mut j = 0;
            for cc in 0..N {
                if cc == c {
                    continue;
                }
                *m.get_mut(i, j) = *self.get(rr, cc);
                j += 1;
            }
            i += 1;
        }
        m
    }
}

pub trait Cofactor<const N: usize>: SquareMatrix<N>
where
    Self::ElementType: std::ops::Neg,
{
    fn cofactor(&self, r: usize, c: usize) -> Self::ElementType;
}

pub trait Inverse<const N: usize>: SquareMatrix<N> + Sized {
    type Output: FloatSquareMatrix<N>;

    fn inverse(&self) -> Option<Self::Output>;
}

impl<const ROW: usize, const COL: usize, V> MatrixLike<ROW, COL> for [V; ROW]
where
    V: VectorLike<COL>,
    <V as VectorLike<COL>>::ElementType: MatrixElement,
{
    type ElementType = <V as VectorLike<COL>>::ElementType;

    fn get(&self, row: usize, col: usize) -> &Self::ElementType {
        self[row].get(col)
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::ElementType {
        self[row].get_mut(col)
    }
}

impl<const N: usize, T> SquareMatrix<N> for [[T; N]; N]
where
    T: MatrixElement,
{
    const IDENTITY: Self = {
        let mut result = [[T::ZERO; N]; N];
        let mut i = 0;
        while i < N {
            result[i][i] = T::ONE;
            i += 1;
        }
        result
    };

    fn transpose(&self) -> Self {
        let mut result = [[T::ZERO; N]; N];
        for i in 0..N {
            for j in 0..N {
                *result.get_mut(j, i) = *self.get(i, j);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_array() {
        let a = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(*a.get(0, 0), 1);
        assert_eq!(*a.get(0, 1), 2);
        assert_eq!(*a.get(0, 2), 3);
        assert_eq!(*a.get(1, 0), 4);
        assert_eq!(*a.get(1, 1), 5);
        assert_eq!(*a.get(1, 2), 6);
    }

    #[test]
    fn set_array() {
        let mut a = [[0, 0, 0], [0, 0, 0]];
        *a.get_mut(0, 0) = 1;
        *a.get_mut(0, 1) = 2;
        *a.get_mut(0, 2) = 3;
        *a.get_mut(1, 0) = 4;
        *a.get_mut(1, 1) = 5;
        *a.get_mut(1, 2) = 6;
        assert_eq!(*a.get(0, 0), 1);
        assert_eq!(*a.get(0, 1), 2);
        assert_eq!(*a.get(0, 2), 3);
        assert_eq!(*a.get(1, 0), 4);
        assert_eq!(*a.get(1, 1), 5);
        assert_eq!(*a.get(1, 2), 6);
    }
}
