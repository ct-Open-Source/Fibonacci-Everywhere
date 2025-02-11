// Code taken from https://github.com/pihedron/fib
// Run with:
//   cargo run --release <<<25000000

use num_bigint::{BigInt, Sign};

fn fib_luc(mut n: isize) -> (BigInt, BigInt) {
    if n == 0 {
        return (BigInt::ZERO, BigInt::new(Sign::Plus, [2].to_vec()))
    }

    if n < 0 {
        n *= -1;
        let (fib, luc) = fib_luc(n);
        let k = n % 2 * 2 - 1;
        return (fib * k, luc * k)
    }

    if n & 1 == 1 {
        let (fib, luc) = fib_luc(n - 1);
        return (&fib + &luc >> 1, 5 * &fib + &luc >> 1)
    }

    n >>= 1;
    let k = n % 2 * 2 - 1;
    let (fib, luc) = fib_luc(n);
    (&fib * &luc, &luc * &luc + 2 * k)
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let n = s.parse::<isize>().unwrap();
    let start = std::time::Instant::now();
    let _fib = fib_luc(n).0;
    let elapsed = start.elapsed();
    // println!("{}", fib);
    println!("{:?}", elapsed);
}
