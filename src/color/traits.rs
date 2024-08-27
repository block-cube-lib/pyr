use crate::num::*;
use num::traits::NumAssign;
use std::fmt::Debug;

pub trait ColorElement: NumAssign + Clone + Copy + Debug + Default + Zero + One {}

pub trait Color<T: ColorElement> {
    fn r(&self) -> T;
    fn g(&self) -> T;
    fn b(&self) -> T;
    fn a(&self) -> T;
}

impl<T> ColorElement for T where T: NumAssign + Clone + Copy + Debug + Default + Zero + One {}
