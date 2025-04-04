use super::mat::*;
use super::vec::*;
use std::ops::*;

//
// impl vec * mat
//

macro_rules! impl_vec_mul_mat {
    ($VecN:ident, $MatRows:expr, $MatCols:expr, $VecOutput:ident, $($index:expr),*) => {
        impl<T: VectorElement> Mul<Mat<T, $MatRows, $MatCols>> for $VecN<T> {
            type Output = $VecOutput<T>;

            fn mul(self, rhs: Mat<T, $MatRows, $MatCols>) -> Self::Output {
                $VecOutput::new(
                    $(self.dot(rhs.row($index))),*
                )
            }
        }
    };
}

impl_vec_mul_mat!(Vec1, 1, 1, Vec1, 0);
impl_vec_mul_mat!(Vec1, 1, 2, Vec2, 0, 1);
impl_vec_mul_mat!(Vec1, 1, 3, Vec3, 0, 1, 2);
impl_vec_mul_mat!(Vec1, 1, 4, Vec4, 0, 1, 2, 3);

impl_vec_mul_mat!(Vec2, 2, 1, Vec1, 0);
impl_vec_mul_mat!(Vec2, 2, 2, Vec2, 0, 1);
impl_vec_mul_mat!(Vec2, 2, 3, Vec3, 0, 1, 2);
impl_vec_mul_mat!(Vec2, 2, 4, Vec4, 0, 1, 2, 3);

impl_vec_mul_mat!(Vec3, 3, 1, Vec1, 0);
impl_vec_mul_mat!(Vec3, 3, 2, Vec2, 0, 1);
impl_vec_mul_mat!(Vec3, 3, 3, Vec3, 0, 1, 2);
impl_vec_mul_mat!(Vec3, 3, 4, Vec4, 0, 1, 2, 3);

impl_vec_mul_mat!(Vec4, 4, 1, Vec1, 0);
impl_vec_mul_mat!(Vec4, 4, 2, Vec2, 0, 1);
impl_vec_mul_mat!(Vec4, 4, 3, Vec3, 0, 1, 2);
impl_vec_mul_mat!(Vec4, 4, 4, Vec4, 0, 1, 2, 3);

impl<T: VectorElement> Mul<Mat<T, 4, 4>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Mat<T, 4, 4>) -> Self::Output {
        let v4 = Vec4::new(self.x, self.y, self.z, T::ONE);
        let v = v4 * rhs;
        Vec3::new(v.x, v.y, v.z)
    }
}

macro_rules! impl_mat_mul_vec {
    ($VecN:ident, $MatRows:expr, $MatCols:expr, $VecOutput:ident, $($index:expr),*) => {
        impl<T: VectorElement> Mul<$VecN<T>> for Mat<T, $MatRows, $MatCols> {
            type Output = $VecOutput<T>;

            fn mul(self, rhs: $VecN<T>) -> Self::Output {
                $VecOutput::new(
                    $(rhs.dot(self.col($index))),*
                )
            }
        }
    };
}

impl_mat_mul_vec!(Vec1, 1, 1, Vec1, 0);
impl_mat_mul_vec!(Vec1, 2, 1, Vec2, 0, 1);
impl_mat_mul_vec!(Vec1, 3, 1, Vec3, 0, 1, 2);
impl_mat_mul_vec!(Vec1, 4, 1, Vec4, 0, 1, 2, 3);

impl_mat_mul_vec!(Vec2, 1, 2, Vec1, 0);
impl_mat_mul_vec!(Vec2, 2, 2, Vec2, 0, 1);
impl_mat_mul_vec!(Vec2, 3, 2, Vec3, 0, 1, 2);
impl_mat_mul_vec!(Vec2, 4, 2, Vec4, 0, 1, 2, 3);

impl_mat_mul_vec!(Vec3, 1, 3, Vec1, 0);
impl_mat_mul_vec!(Vec3, 2, 3, Vec2, 0, 1);
impl_mat_mul_vec!(Vec3, 3, 3, Vec3, 0, 1, 2);
impl_mat_mul_vec!(Vec3, 4, 3, Vec4, 0, 1, 2, 3);

impl_mat_mul_vec!(Vec4, 1, 4, Vec1, 0);
impl_mat_mul_vec!(Vec4, 2, 4, Vec2, 0, 1);
impl_mat_mul_vec!(Vec4, 3, 4, Vec3, 0, 1, 2);
impl_mat_mul_vec!(Vec4, 4, 4, Vec4, 0, 1, 2, 3);

impl<T: VectorElement> Mul<Vec3<T>> for Mat<T, 4, 4> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        let v4 = Vec4::new(rhs.x, rhs.y, rhs.z, T::ONE);
        let v = self * v4;
        Vec3::new(v.x, v.y, v.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_vec1_mat1x1() {
        let v = Vec1::new(2);
        let m = Mat::new([[3]]);
        let v2 = v * m;
        assert_eq!(v2, Vec1::new(6));
    }

    #[test]
    fn mul_vec2_mat1x2() {
        let v = Vec2::new(2, 3);
        let m = Mat::new([[4], [5]]);
        let v2 = v * m;
        assert_eq!(v2, Vec1::new(23));
    }

    #[test]
    fn mul_vec3_mat3x3() {
        let x = 1;
        let y = 2;
        let z = 3;
        let m11 = 4;
        let m12 = 5;
        let m13 = 6;
        let m21 = 7;
        let m22 = 8;
        let m23 = 9;
        let m31 = 10;
        let m32 = 11;
        let m33 = 12;
        let v = Vec3::new(x, y, z);
        let m = Mat::new([[m11, m12, m13], [m21, m22, m23], [m31, m32, m33]]);
        let v2 = v * m;
        assert_eq!(
            v2,
            Vec3::new(
                x * m11 + y * m21 + z * m31,
                x * m12 + y * m22 + z * m32,
                x * m13 + y * m23 + z * m33,
            )
        );
    }

    #[test]
    fn mul_vec3_mat4x4() {
        let m = Mat::new([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [1, 2, 3, 1]]);
        let v = Vec3::new(1, 2, 3);
        assert_eq!(v * m, Vec3::new(2, 4, 6));
    }
}
