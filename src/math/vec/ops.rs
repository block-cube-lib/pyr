use super::traits::*;
use crate::num::{AsFloatingPoint, Zero};

pub fn into_floating_point_array<In, const N: usize>(
    v: In,
) -> [<In::ElementType as AsFloatingPoint>::Output; N]
where
    In: VectorLike<N>,
{
    let mut result = [In::ElementType::ZERO.as_floating_point(); N];
    for i in 0..N {
        result[i] = v.get(i).as_floating_point();
    }
    result
}

pub fn vector_cast<T: VectorElement, In, Out, const N: usize>(v: In) -> Out
where
    In: VectorLike<N, ElementType = T>,
    Out: VectorLike<N, ElementType = T>,
{
    let arr: [T; N] = v.into();
    arr.into()
}

pub fn add<VRet, T, const N: usize>(
    lhs: impl VectorLike<N, ElementType = T>,
    rhs: impl VectorLike<N, ElementType = T>,
) -> VRet
where
    VRet: VectorLike<N, ElementType = T>,
    T: VectorElement,
{
    let mut result = lhs;
    for i in 0..N {
        *result.get_mut(i) += *rhs.get(i);
    }
    vector_cast(result)
}
pub fn sub<VRet, T, const N: usize>(
    lhs: impl VectorLike<N, ElementType = T>,
    rhs: impl VectorLike<N, ElementType = T>,
) -> VRet
where
    VRet: VectorLike<N, ElementType = T>,
    T: VectorElement,
{
    let mut result = lhs;
    for i in 0..N {
        *result.get_mut(i) += *rhs.get(i);
    }
    vector_cast(result)
}

pub fn mul_scalar<VRet, T, const N: usize>(lhs: impl VectorLike<N, ElementType = T>, rhs: T) -> VRet
where
    VRet: VectorLike<N, ElementType = T>,
    T: VectorElement,
{
    let mut result = lhs;
    for i in 0..N {
        *result.get_mut(i) *= rhs;
    }
    vector_cast(result)
}

pub fn div_scalar<VRet, T, const N: usize>(lhs: impl VectorLike<N, ElementType = T>, rhs: T) -> VRet
where
    VRet: VectorLike<N, ElementType = T>,
    T: VectorElement,
{
    let mut result = lhs;
    for i in 0..N {
        *result.get_mut(i) *= rhs;
    }
    vector_cast(result)
}

pub fn dot<const N: usize, T: VectorElement>(
    a: impl VectorLike<N, ElementType = T>,
    b: impl VectorLike<N, ElementType = T>,
) -> T {
    let mut result = T::ZERO;
    for i in 0..N {
        result += *a.get(i) * *b.get(i);
    }
    result
}

pub fn length_squared<T: VectorElement, const N: usize>(
    v: impl VectorLike<N, ElementType = T>,
) -> T {
    dot(v, v)
}

pub fn length<T: VectorElement, const N: usize>(
    v: impl VectorLike<N, ElementType = T>,
) -> <T as AsFloatingPoint>::Output
where
    <T as AsFloatingPoint>::Output: VectorElement,
{
    let fv: [<T as AsFloatingPoint>::Output; N] = into_floating_point_array(v);
    let f_ls = length_squared(fv);
    ::num::Float::sqrt(f_ls)
}

pub fn normalized<VRet, V: VectorLike<N>, const N: usize>(v: V) -> VRet
where
    V: VectorLike<N>,
    <V::ElementType as AsFloatingPoint>::Output: VectorElement,
    VRet: FloatVectorLike<N, ElementType = <V::ElementType as AsFloatingPoint>::Output>,
{
    let len = length(v);
    let mut result = into_floating_point_array(v);
    for i in 0..N {
        result[i] = result[i] / len;
    }
    result.into()
}

pub fn angle<T: FloatVectorElement, const N: usize>(
    a: impl VectorLike<N, ElementType = T>,
    b: impl VectorLike<N, ElementType = T>,
) -> T {
    let dot = dot(a, b);
    let len = length(a) * length(b);
    (dot / len).acos()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_2d_tuple() {
        let a = (1.0, 2.0);
        let b = (3.0, 4.0);
        let result = dot(a, b);
        assert_eq!(result, 11.0); // 1*3 + 2*4 = 11
    }

    #[test]
    fn test_dot_3d_tuple() {
        let a = (1.0, 2.0, 3.0);
        let b = (4.0, 5.0, 6.0);
        let result = dot(a, b);
        assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_dot_4d_tuple() {
        let a = (1.0, 2.0, 3.0, 4.0);
        let b = (5.0, 6.0, 7.0, 8.0);
        let result = dot(a, b);
        assert_eq!(result, 70.0); // 1*5 + 2*6 + 3*7 + 4*8 = 70
    }

    #[test]
    fn test_dot_mixed_3d() {
        let a = (1.0, 2.0, 3.0);
        let b = [4.0, 5.0, 6.0];
        let result = dot(a, b);
        assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_dot_negative_3d() {
        let a = (-1.0, 2.0, -3.0);
        let b = (4.0, -5.0, 6.0);
        let result = dot(a, b);
        assert_eq!(result, -32.0); // -1*4 + 2*(-5) + (-3)*6 = -32
    }

    #[test]
    fn test_angle_2d() {
        let a = (1.0, 0.0);
        let b = (0.0, 1.0);
        let result = angle(a, b);
        assert!((result - std::f32::consts::FRAC_PI_2).abs() < 1e-6); // pi/2 radians
    }

    #[test]
    fn test_angle_3d() {
        let a = (1.0, 0.0, 0.0);
        let b = (0.0, 1.0, 0.0);
        let result = angle(a, b);
        assert!((result - std::f32::consts::FRAC_PI_2).abs() < 1e-6); // pi/2 radians
    }

    #[test]
    fn test_angle_same_vector() {
        let a = (1.0_f32, 0.0, 0.0);
        let result = angle(a, a);
        assert!((result).abs() < 1e-6); // 0度
    }

    #[test]
    fn test_angle_opposite_vector() {
        let a = (1.0, 0.0, 0.0);
        let b = (-1.0, 0.0, 0.0);
        let result = angle(a, b);
        assert!((result - std::f32::consts::PI).abs() < 1e-6); // pi radians
    }
}
