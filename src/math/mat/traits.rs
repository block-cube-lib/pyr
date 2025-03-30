use crate::math::vec::traits::*;

pub trait MatrixElement: VectorElement {}

impl<T> MatrixElement for T where T: VectorElement {}

pub trait MatrixLike<const ROW: usize, const COL: usize> {
    type ElementType: MatrixElement;

    fn get(&self, row: usize, col: usize) -> &Self::ElementType;
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::ElementType;
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

pub trait SquareMatrix<const N: usize>: MatrixLike<N, N> {
    const IDENTITY: Self;
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
