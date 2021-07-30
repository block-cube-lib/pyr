pub struct Matrix<T, const COL: usize, const ROW: usize> {
    pub elements: [[T; ROW]; COL],
}

pub type Matrix3<T> = Matrix<T, 3, 3>;
pub type Matrix4<T> = Matrix<T, 4, 4>;

impl<T, const COL: usize, const ROW: usize> Matrix<T, COL, ROW> {
}
