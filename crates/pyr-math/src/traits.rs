/// A trait for approximate equality comparison of floating-point numbers.
///
/// ```
/// let a: f32 = 0.1 + 0.2;
/// let b: f32 = 0.3;
/// assert!(a.approx_eq(&b));
/// ```
pub trait ApproxEq<T = Self> {
    /// The type of the epsilon value used for comparison.
    type Epsilon;

    /// Checks if two floating-point numbers are approximately equal within a default epsilon.
    fn approx_eq(&self, other: &T) -> bool;

    /// Checks if two floating-point numbers are approximately equal within a specified epsilon.
    fn approx_eq_eps(&self, other: &T, epsilon: Self::Epsilon) -> bool;
}

impl ApproxEq for f32 {
    type Epsilon = f32;

    #[inline]
    fn approx_eq(&self, other: &f32) -> bool {
        self.approx_eq_eps(other, 1e-5)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &f32, epsilon: f32) -> bool {
        (*self - *other).abs() <= epsilon
    }
}

impl ApproxEq for f64 {
    type Epsilon = f64;

    #[inline]
    fn approx_eq(&self, other: &f64) -> bool {
        self.approx_eq_eps(other, 1e-10)
    }

    #[inline]
    fn approx_eq_eps(&self, other: &f64, epsilon: f64) -> bool {
        (*self - *other).abs() <= epsilon
    }
}

/// A trait for linear interpolation between two values.
///
/// ```
/// let a: f32 = 0.0;
/// let b: f32 = 10.0;
/// let t: f32 = 0.5;
/// let result = a.lerp(b, t);
/// assert!(result.approx_eq(5.0));
pub trait Lerp {
    /// The type of the interpolation factor.
    type Factor;

    /// Linearly interpolates between `self` and `other` by a factor of `t`.
    /// `t` should be in the range [0, 1], where 0 returns `self` and 1 returns `other`.
    fn lerp(self, other: Self, t: Self::Factor) -> Self;
}

impl Lerp for f32 {
    type Factor = f32;

    #[inline]
    fn lerp(self, other: Self, t: Self) -> Self {
        self * (1.0 - t) + other * t
    }
}

impl Lerp for f64 {
    type Factor = f64;

    #[inline]
    fn lerp(self, other: Self, t: Self) -> Self {
        self * (1.0 - t) + other * t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_approx_eq_f32() {
        let a: f32 = 0.1 + 0.2;
        let b: f32 = 0.3;
        assert!(a.approx_eq(&b));
    }

    #[test]
    fn test_approx_eq_f64() {
        let a: f64 = 0.1 + 0.2;
        let b: f64 = 0.3;
        assert!(a.approx_eq(&b));
    }

    #[test]
    fn test_approx_eq_eps_f32() {
        let a: f32 = 0.1 + 0.2;
        let b: f32 = 0.3;
        assert!(a.approx_eq_eps(&b, 1e-5));
    }

    #[test]
    fn test_approx_eq_eps_f64() {
        let a: f64 = 0.1 + 0.2;
        let b: f64 = 0.3;
        assert!(a.approx_eq_eps(&b, 1e-10));
    }

    #[test]
    fn test_lerp_f32() {
        let a: f32 = 0.0;
        let b: f32 = 2.0;
        let t: f32 = 0.5;
        let result = a.lerp(b, t);
        assert!(result.approx_eq(&1.0));
    }
}
