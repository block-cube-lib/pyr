use num::Num;

pub trait VectorElement: Num + Clone + Copy {}
impl<T> VectorElement for T where T: Num + Clone + Copy {}

pub trait Vector: Clone + Copy {
    type ElementType: VectorElement;
    const DIMENSION: usize;
}
