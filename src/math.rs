pub mod matrix;
pub mod quaternion;
pub mod vector;

pub use self::matrix::*;
pub use self::quaternion::Quaternion;
pub use self::vector::*;

impl<T: MatrixElement, const ROW: usize, const COL: usize> std::ops::Mul<Matrix<T, ROW, COL>>
    for Vector<T, ROW>
{
    type Output = Vector<T, COL>;

    fn mul(self, m: Matrix<T, ROW, COL>) -> Self::Output {
        let mut ret = Self::Output::default();
        for i in 0..COL {
            ret[i] = self.dot(m.col(i));
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_vector_matrix() {
        let v: Vector<i32, 3> = [3, 4, 5].into();
        let m: Matrix<i32, 3, 2> = [[1, 2], [3, 4], [5, 6]].into();
        assert_eq!(v * m, [40, 52].into());
    }
}

pub type F32Vector1 = Vector<f32, 1>;
pub type F32Vector2 = Vector<f32, 2>;
pub type F32Vector3 = Vector<f32, 3>;
pub type F32Vector4 = Vector<f32, 4>;
pub type F64Vector1 = Vector<f64, 1>;
pub type F64Vector2 = Vector<f64, 2>;
pub type F64Vector3 = Vector<f64, 3>;
pub type F64Vector4 = Vector<f64, 4>;

pub type F32Matrix3 = Matrix<f32, 3, 3>;
pub type F32Matrix4 = Matrix<f32, 4, 4>;
pub type F64Matrix3 = Matrix<f64, 3, 3>;
pub type F64Matrix4 = Matrix<f64, 4, 4>;
