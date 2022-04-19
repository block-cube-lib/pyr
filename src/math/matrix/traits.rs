use crate::math::vector::*;

pub trait MatrixElement: VectorElement {}

impl<T> MatrixElement for T where T: VectorElement {}

pub trait MatrixLike<T, const ROW: usize, const COL: usize> {
    fn get(&self, row: usize, col: usize) -> &T;
}

impl<T, const ROW: usize, const COL: usize, V> MatrixLike<T, ROW, COL> for [V; ROW]
where
    T: MatrixElement,
    V: VectorLike<T, COL>,
{
    fn get(&self, row: usize, col: usize) -> &T {
        self[row].get(col)
    }
}

pub trait SquareMatrix {
    fn identity() -> Self;
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
}
