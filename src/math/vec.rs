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
pub trait VectorTypeHolder<T: VectorElement, const D: usize> {
    type Vector;
}

#[doc(hidden)]
pub struct VectorTypeResolver<T: VectorElement, const D: usize> {
    _marker: std::marker::PhantomData<fn() -> [T; D]>,
}

impl<T: VectorElement> VectorTypeHolder<T, 1> for VectorTypeResolver<T, 1> {
    type Vector = Vec1<T>;
}
impl<T: VectorElement> VectorTypeHolder<T, 2> for VectorTypeResolver<T, 2> {
    type Vector = Vec2<T>;
}
impl<T: VectorElement> VectorTypeHolder<T, 3> for VectorTypeResolver<T, 3> {
    type Vector = Vec3<T>;
}
impl<T: VectorElement> VectorTypeHolder<T, 4> for VectorTypeResolver<T, 4> {
    type Vector = Vec4<T>;
}

seq_macro::seq!(N in 5..32 {
    impl<T: VectorElement> VectorTypeHolder<T, N> for VectorTypeResolver<T, N> {
        type Vector = ArrayWrapper<T, N>;
    }
});

/// Vector type. T: Type of the element. D: Dimension. D must be in the range of 1 to 32.
pub type Vector<T, const D: usize> = <VectorTypeResolver<T, D> as VectorTypeHolder<T, D>>::Vector;

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
        assert_eq!(type_name::<Vec1::<T>>(), type_name::<Vector<T, 1>>());
        assert_eq!(type_name::<Vec2::<T>>(), type_name::<Vector<T, 2>>());
        assert_eq!(type_name::<Vec3::<T>>(), type_name::<Vector<T, 3>>());
        assert_eq!(type_name::<Vec4::<T>>(), type_name::<Vector<T, 4>>());
        assert_eq!(
            type_name::<ArrayWrapper::<T, 5>>(),
            type_name::<Vector<T, 5>>()
        );
        assert_eq!(
            type_name::<ArrayWrapper::<T, 6>>(),
            type_name::<Vector<T, 6>>()
        );
        assert_eq!(
            type_name::<ArrayWrapper::<T, 7>>(),
            type_name::<Vector<T, 7>>()
        );
        assert_eq!(
            type_name::<ArrayWrapper::<T, 8>>(),
            type_name::<Vector<T, 8>>()
        );
    }
}
