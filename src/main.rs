use pyrite::math::prelude::*;
use pyrite::math::Vector4;

fn main() {
    let v = Vector4::new(1, 3, 6, 87);
    println!("{}", v.length_squared());
}
