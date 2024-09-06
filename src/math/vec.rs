mod array_wrapper;
pub(crate) mod traits;
mod vec1;
mod vec2;
mod vec3;
mod vec4;

pub use array_wrapper::ArrayWrapper;
pub use traits::{FloatVectorElement, VectorElement, VectorLike};
pub use vec1::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;

#[doc(hidden)]
pub trait VecTypeHolder<T: VectorElement, const D: usize> {
    type Vec;
}

#[doc(hidden)]
pub struct VectorTypeResolver<T: VectorElement, const D: usize> {
    _marker: std::marker::PhantomData<fn() -> [T; D]>,
}

impl<T: VectorElement> VecTypeHolder<T, 1> for VectorTypeResolver<T, 1> {
    type Vec = Vec1<T>;
}
impl<T: VectorElement> VecTypeHolder<T, 2> for VectorTypeResolver<T, 2> {
    type Vec = Vec2<T>;
}
impl<T: VectorElement> VecTypeHolder<T, 3> for VectorTypeResolver<T, 3> {
    type Vec = Vec3<T>;
}
impl<T: VectorElement> VecTypeHolder<T, 4> for VectorTypeResolver<T, 4> {
    type Vec = Vec4<T>;
}

seq_macro::seq!(N in 5..32 {
    impl<T: VectorElement> VecTypeHolder<T, N> for VectorTypeResolver<T, N> {
        type Vec = ArrayWrapper<T, N>;
    }
});

/// Vector type. T: Type of the element. D: Dimension. D must be in the range of 1 to 32.
pub type Vec<T, const D: usize> = <VectorTypeResolver<T, D> as VecTypeHolder<T, D>>::Vec;

#[cfg(test)]
mod test {
    use super::*;
    use std::any::type_name;

    #[test]
    fn vector_alias() {
        vector_alias_impl::<i32>();
        vector_alias_impl::<i64>();
        vector_alias_impl::<f32>();
        vector_alias_impl::<f64>();
    }

    fn vector_alias_impl<T: VectorElement>() {
        assert_eq!(type_name::<Vec1::<T>>(), type_name::<Vec<T, 1>>());
        assert_eq!(type_name::<Vec2::<T>>(), type_name::<Vec<T, 2>>());
        assert_eq!(type_name::<Vec3::<T>>(), type_name::<Vec<T, 3>>());
        assert_eq!(type_name::<Vec4::<T>>(), type_name::<Vec<T, 4>>());
        assert_eq!(
            type_name::<ArrayWrapper::<T, 5>>(),
            type_name::<Vec<T, 5>>()
        );
        assert_eq!(
            type_name::<ArrayWrapper::<T, 6>>(),
            type_name::<Vec<T, 6>>()
        );
        assert_eq!(
            type_name::<ArrayWrapper::<T, 7>>(),
            type_name::<Vec<T, 7>>()
        );
        assert_eq!(
            type_name::<ArrayWrapper::<T, 8>>(),
            type_name::<Vec<T, 8>>()
        );
    }
}
