use crate::color::traits::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rgba<T: ColorElement> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T: ColorElement> Rgba<T> {
    pub const fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl<T: ColorElement + Eq> Eq for Rgba<T> where T: Eq {}

impl<T: ColorElement> Color<T> for Rgba<T> {
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
        self.a
    }
}
