use crate::color::traits::*;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rgb<T: ColorElement> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: ColorElement> Rgb<T> {
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl<T: ColorElement + Eq> Eq for Rgb<T> where T: Eq {}

impl<T: ColorElement> Color<T> for Rgb<T> {
    fn r(&self) -> T {
        self.r
    }

    fn g(&self) -> T {
        self.g
    }

    fn b(&self) -> T {
        self.b
    }

    fn a(&self) -> T {
        T::ONE
    }
}
