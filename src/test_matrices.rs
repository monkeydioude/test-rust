use std::ops::Mul;

use ndarray::array;

pub fn main() {
    let a = &array![[2, 2]];
    let b = &array![[3], [3]];

    println!("a: {:?}\nb: {:?}\n a*b: {:?}", a, b, a.dot(b));
}