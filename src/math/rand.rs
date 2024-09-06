use crate::math::vec::{traits::*, Vec1, Vec2, Vec3, Vec4};
use num::Float;
use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;

// impl Distoribution for Vec
impl<T> Distribution<Vec1<T>> for Standard
where
    T: VectorElement,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec1<T> {
        Vec1 { x: rng.gen() }
    }
}

impl<T> Distribution<Vec2<T>> for Standard
where
    T: VectorElement,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec2<T> {
        Vec2 {
            x: rng.gen(),
            y: rng.gen(),
        }
    }
}

impl<T> Distribution<Vec3<T>> for Standard
where
    T: VectorElement,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3<T> {
        Vec3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
}

impl<T> Distribution<Vec4<T>> for Standard
where
    T: VectorElement,
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec4<T> {
        Vec4 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
            w: rng.gen(),
        }
    }
}

//
// impl Uniform for Vec1
//
pub struct UniformVec1<T>
where
    T: VectorElement + SampleUniform,
{
    x_sampler: Uniform<T>,
}

impl<T> UniformSampler for UniformVec1<T>
where
    T: VectorElement + SampleUniform,
{
    type X = Vec1<T>;
    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        Self {
            x_sampler: Uniform::new(low.borrow().x, high.borrow().x),
        }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        Self {
            x_sampler: Uniform::new_inclusive(low.borrow().x, high.borrow().x),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Vec1 {
            x: self.x_sampler.sample(rng),
        }
    }
}

impl<T> SampleUniform for Vec1<T>
where
    T: VectorElement + SampleUniform,
{
    type Sampler = UniformVec1<T>;
}

//
// impl Uniform for Vec2
//
pub struct UniformVec2<T>
where
    T: VectorElement + SampleUniform,
{
    x_sampler: Uniform<T>,
    y_sampler: Uniform<T>,
}

impl<T> UniformSampler for UniformVec2<T>
where
    T: VectorElement + SampleUniform,
{
    type X = Vec2<T>;
    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low.borrow();
        let high = high.borrow();
        Self {
            x_sampler: Uniform::new(low.x, high.x),
            y_sampler: Uniform::new(low.y, high.y),
        }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low.borrow();
        let high = high.borrow();
        Self {
            x_sampler: Uniform::new_inclusive(low.x, high.x),
            y_sampler: Uniform::new_inclusive(low.y, high.y),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Vec2 {
            x: self.x_sampler.sample(rng),
            y: self.y_sampler.sample(rng),
        }
    }
}

impl<T> SampleUniform for Vec2<T>
where
    T: VectorElement + SampleUniform,
{
    type Sampler = UniformVec2<T>;
}

//
// impl Uniform for Vec3
//
pub struct UniformVec3<T>
where
    T: VectorElement + SampleUniform,
{
    x_sampler: Uniform<T>,
    y_sampler: Uniform<T>,
    z_sampler: Uniform<T>,
}

impl<T> UniformSampler for UniformVec3<T>
where
    T: VectorElement + SampleUniform,
{
    type X = Vec3<T>;
    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low.borrow();
        let high = high.borrow();
        Self {
            x_sampler: Uniform::new(low.x, high.x),
            y_sampler: Uniform::new(low.y, high.y),
            z_sampler: Uniform::new(low.z, high.z),
        }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low.borrow();
        let high = high.borrow();
        Self {
            x_sampler: Uniform::new_inclusive(low.x, high.x),
            y_sampler: Uniform::new_inclusive(low.y, high.y),
            z_sampler: Uniform::new_inclusive(low.z, high.z),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Vec3 {
            x: self.x_sampler.sample(rng),
            y: self.y_sampler.sample(rng),
            z: self.z_sampler.sample(rng),
        }
    }
}

impl<T> SampleUniform for Vec3<T>
where
    T: VectorElement + SampleUniform,
{
    type Sampler = UniformVec3<T>;
}

//
// impl Uniform for Vec4
//
pub struct UniformVec4<T>
where
    T: VectorElement + SampleUniform,
{
    x_sampler: Uniform<T>,
    y_sampler: Uniform<T>,
    z_sampler: Uniform<T>,
    w_sampler: Uniform<T>,
}

impl<T> UniformSampler for UniformVec4<T>
where
    T: VectorElement + SampleUniform,
{
    type X = Vec4<T>;
    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low.borrow();
        let high = high.borrow();
        Self {
            x_sampler: Uniform::new(low.x, high.x),
            y_sampler: Uniform::new(low.y, high.y),
            z_sampler: Uniform::new(low.z, high.z),
            w_sampler: Uniform::new(low.w, high.w),
        }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low.borrow();
        let high = high.borrow();
        Self {
            x_sampler: Uniform::new_inclusive(low.x, high.x),
            y_sampler: Uniform::new_inclusive(low.y, high.y),
            z_sampler: Uniform::new_inclusive(low.z, high.z),
            w_sampler: Uniform::new_inclusive(low.w, high.w),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Vec4 {
            x: self.x_sampler.sample(rng),
            y: self.y_sampler.sample(rng),
            z: self.z_sampler.sample(rng),
            w: self.w_sampler.sample(rng),
        }
    }
}

impl<T> SampleUniform for Vec4<T>
where
    T: VectorElement + SampleUniform,
{
    type Sampler = UniformVec4<T>;
}

//
// UniformVec3InUnitSphere
//
pub struct UniformVec3InUnitSphere<T>
where
    T: VectorElement + SampleUniform,
{
    uniform: Uniform<T>,
}

impl<T> UniformVec3InUnitSphere<T>
where
    T: VectorElement + SampleUniform + Float,
{
    pub fn new() -> Self {
        let min = T::from(-1.0).unwrap();
        let max = T::from(1.0).unwrap();
        Self {
            uniform: Uniform::new(min, max),
        }
    }
}

impl<T> Default for UniformVec3InUnitSphere<T>
where
    T: VectorElement + SampleUniform + Float,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Distribution<Vec3<T>> for UniformVec3InUnitSphere<T>
where
    T: VectorElement + SampleUniform + Float,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3<T> {
        loop {
            let v = Vec3 {
                x: self.uniform.sample(rng),
                y: self.uniform.sample(rng),
                z: self.uniform.sample(rng),
            };
            if v.length_squared() <= T::ONE {
                break v;
            }
        }
    }
}

//
// UniformUnitVec
//
pub struct UniformUnitVec<T>
where
    T: VectorElement + SampleUniform + Float,
{
    uniform: Uniform<T>,
}

impl<T> UniformUnitVec<T>
where
    T: VectorElement + SampleUniform + Float,
{
    pub fn new() -> Self {
        let min = T::from(-1.0).unwrap();
        let max = T::from(1.0).unwrap();
        Self {
            uniform: Uniform::new(min, max),
        }
    }
}

impl<T> Default for UniformUnitVec<T>
where
    T: VectorElement + SampleUniform + Float,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Distribution<Vec1<T>> for UniformUnitVec<T>
where
    T: VectorElement + SampleUniform + Float,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec1<T> {
        Vec1 {
            x: self.uniform.sample(rng).signum(),
        }
    }
}

impl<T> Distribution<Vec2<T>> for UniformUnitVec<T>
where
    T: FloatVectorElement + SampleUniform,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec2<T> {
        loop {
            let v = Vec2 {
                x: self.uniform.sample(rng),
                y: self.uniform.sample(rng),
            };

            let length_squared = v.length_squared();
            if T::ZERO < length_squared && length_squared <= T::ONE {
                break v.normalized();
            }
        }
    }
}

impl<T> Distribution<Vec3<T>> for UniformUnitVec<T>
where
    T: FloatVectorElement + SampleUniform,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3<T> {
        loop {
            let v = Vec3 {
                x: self.uniform.sample(rng),
                y: self.uniform.sample(rng),
                z: self.uniform.sample(rng),
            };
            let length_squared = v.length_squared();
            if T::ZERO < length_squared && length_squared <= T::ONE {
                break v.normalized();
            }
        }
    }
}

impl<T> Distribution<Vec4<T>> for UniformUnitVec<T>
where
    T: FloatVectorElement + SampleUniform,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec4<T> {
        loop {
            let v = Vec4 {
                x: self.uniform.sample(rng),
                y: self.uniform.sample(rng),
                z: self.uniform.sample(rng),
                w: self.uniform.sample(rng),
            };
            let length_squared = v.length_squared();
            if T::ZERO < length_squared && length_squared <= T::ONE {
                break v.normalized();
            }
        }
    }
}

//
// UniformOnHemiSphere
//

/// ```
/// use pyr::math::{ Vec3, rand::UniformVec3OnHemiSphere };
/// use rand::distributions::Distribution;
///
/// let mut rng = rand::thread_rng();
/// let uniform = UniformVec3OnHemiSphere::new(Vec3::<f32>::UNIT_Y);
/// let v = uniform.sample(&mut rng);
/// assert!(v.y > 0.0);
/// let length = v.length();
/// assert!(0.999 < length && length < 1.001);
/// ```
pub struct UniformVec3OnHemiSphere<T>
where
    T: VectorElement + SampleUniform + Float,
{
    normal: Vec3<T>,
    uniform: UniformUnitVec<T>,
}
impl<T> UniformVec3OnHemiSphere<T>
where
    T: VectorElement + SampleUniform + Float,
{
    pub fn new(normal: Vec3<T>) -> Self {
        Self {
            normal,
            uniform: UniformUnitVec::new(),
        }
    }
}

impl<T> Distribution<Vec3<T>> for UniformVec3OnHemiSphere<T>
where
    T: FloatVectorElement + SampleUniform + Float,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3<T> {
        let v: Vec3<T> = self.uniform.sample(rng);
        if v.dot(self.normal) >= T::ZERO {
            v
        } else {
            -v
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::num::Zero;

    #[test]
    fn rand_standard_vec2() {
        let mut rng = rand::thread_rng();
        let mut prev = Vec2::<i32>::default();
        for _ in 0..1000 {
            let v: Vec2<i32> = rng.gen();
            assert_ne!(v, prev);
            prev = v;
        }
    }

    #[test]
    fn rand_standard_vec3() {
        let mut rng = rand::thread_rng();
        let mut prev = <Vec3<i32> as Zero>::ZERO;
        for _ in 0..1000 {
            let v: Vec3<i32> = rng.gen();
            assert_ne!(v, prev);
            prev = v;
        }
    }

    #[test]
    fn uniform_vec1() {
        let range = -10..100;
        let uniform = Uniform::new(Vec1 { x: range.start }, Vec1 { x: range.end });
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            let v = uniform.sample(&mut rng);
            assert!(range.contains(&v.x));
        }
    }

    #[test]
    fn uniform_vec2() {
        let x_range = -10..100;
        let y_range = -200..-20;
        let uniform = Uniform::new(
            Vec2 {
                x: x_range.start,
                y: y_range.start,
            },
            Vec2 {
                x: x_range.end,
                y: y_range.end,
            },
        );
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            let v = uniform.sample(&mut rng);
            assert!(x_range.contains(&v.x));
            assert!(y_range.contains(&v.y));
        }
    }

    #[test]
    fn uniform_vec3() {
        let x_range = -10..100;
        let y_range = -200..-20;
        let z_range = 200..400;
        let uniform = Uniform::new(
            Vec3 {
                x: x_range.start,
                y: y_range.start,
                z: z_range.start,
            },
            Vec3 {
                x: x_range.end,
                y: y_range.end,
                z: z_range.end,
            },
        );
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            let v = uniform.sample(&mut rng);
            assert!(x_range.contains(&v.x));
            assert!(y_range.contains(&v.y));
            assert!(z_range.contains(&v.z));
        }
    }

    #[test]
    fn uniform_in_unit_sphere() {
        let mut rng = rand::thread_rng();
        let dist = UniformVec3InUnitSphere::new();
        for _ in 0..1000 {
            let v: Vec3<f32> = dist.sample(&mut rng);
            assert!(v.length_squared() <= 1.0);
        }
    }

    #[test]
    fn uniform_unit_vec1() {
        let mut rng = rand::thread_rng();
        let uniform = UniformUnitVec::new();
        for _ in 0..1000 {
            let v: Vec1<f32> = uniform.sample(&mut rng);
            assert_eq!(v.length_squared(), 1.0);
        }
    }

    #[test]
    fn uniform_unit_vec2() {
        let mut rng = rand::thread_rng();
        let uniform = UniformUnitVec::new();
        for _ in 0..1000 {
            let v: Vec2<f32> = uniform.sample(&mut rng);
            let length_squared = v.length_squared();
            assert!(
                0.999 < length_squared && length_squared < 1.001,
                "length_squared: {}",
                length_squared
            );
        }
    }

    #[test]
    fn uniform_unit_vec3() {
        let mut rng = rand::thread_rng();
        let uniform = UniformUnitVec::new();
        for _ in 0..1000 {
            let v: Vec3<f32> = uniform.sample(&mut rng);
            let length_squared = v.length_squared();
            assert!(
                0.999 < length_squared && length_squared < 1.001,
                "length_squared: {}",
                length_squared
            );
        }
    }

    #[test]
    fn uniform_unit_vec4() {
        let mut rng = rand::thread_rng();
        let uniform = UniformUnitVec::new();
        for _ in 0..1000 {
            let v: Vec4<f32> = uniform.sample(&mut rng);
            let length_squared = v.length_squared();
            assert!(
                0.999 < length_squared && length_squared < 1.001,
                "length_squared: {}",
                length_squared
            );
        }
    }
}
