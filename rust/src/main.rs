use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;

fn fib(n: usize) {
    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();
    for _ in 0..n {
        print!("{} ", a);
        let c = a + &b;
        a = replace(&mut b, c);
    }
}

fn main() {
    fib(100);
}
