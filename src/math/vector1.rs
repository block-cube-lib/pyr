use std;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vector1 {
    pub x: f32,
}
include!("vector_ops.rs");
vector_operators!(Vector1; x, 0);
include!("vector_functions.rs");
vector_functions!(Vector1; x, 0);

#[cfg(test)]
mod test {
    use super::Vector1;
    vector_operators_test!(Vector1; x, 0);
    vector_functions_test!(Vector1; x, 0);
}
