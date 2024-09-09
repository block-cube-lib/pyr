use super::traits::*;

pub fn dot<const N: usize, T: VectorElement>(
    a: impl VectorLike<N, ElementType = T>,
    b: impl VectorLike<N, ElementType = T>,
) -> T {
    (0..N).fold(T::ZERO, |acc, i| acc + a.get(i) * b.get(i))
}

pub fn length_squared<T: VectorElement, const N: usize>(
    v: impl VectorLike<N, ElementType = T>,
) -> T {
    let mut sum = T::ZERO;
    for i in 0..N {
        sum += v.get(i) * v.get(i);
    }
    sum
}

pub fn length<T: VectorElement, const N: usize>(
    v: impl VectorLike<N, ElementType = T>,
) -> T::FloatCalcType {
    use num::Float as _;
    let mut sum = T::ZERO.as_float_type();
    for i in 0..N {
        let element_as_float = v.get(i).as_float_type();
        sum = sum + element_as_float * element_as_float;
    }
    sum.sqrt()
}

pub fn normalized<VRet, V: VectorLike<N>, const N: usize>(v: V) -> VRet
where
    V: VectorLike<N>,
    VRet: FloatVectorLike<N, ElementType = <V::ElementType as VectorElement>::FloatCalcType>,
{
    let len = length(v.clone());
    let mut result = v.into_float_array();
    for i in 0..N {
        result[i] = result[i] / len;
    }
    VRet::from_array(result)
}

pub fn angle<T: FloatVectorElement, const N: usize>(
    a: impl VectorLike<N, ElementType = T>,
    b: impl VectorLike<N, ElementType = T>,
) -> T {
    let a = a.into_float_array();
    let b = b.into_float_array();
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
