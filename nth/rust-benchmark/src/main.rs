use num_bigint::{BigInt, Sign};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

use std::future::Future;
use tokio::time::{timeout, Duration};

async fn with_timeout<T, F>(future: F, seconds: u64, funcname: &str) -> Option<T>
where
    F: Future<Output = T>,
{
    match timeout(Duration::from_secs(seconds), future).await {
        Ok(result) => Some(result),
        Err(_) => {
            println!("{} timed out.", funcname);
            None
        }
    }
}

fn fib_luc_priv(n: i32) -> (BigInt, BigInt) {
    let mut n = n;
    if n == 0 {
        return (BigInt::from(0), BigInt::new(Sign::Plus, [2].to_vec()));
    }

    if n < 0 {
        n *= -1;
        let (fib, luc) = fib_luc_priv(n);
        let k = n % 2 * 2 - 1;
        return (fib * k, luc * k);
    }

    if n & 1 == 1 {
        let (fib, luc) = fib_luc_priv(n - 1);
        return (&fib + &luc >> 1, 5u32 * &fib + &luc >> 1);
    }

    n >>= 1;
    let k = n % 2 * 2 - 1;
    let (fib, luc) = fib_luc_priv(n);
    (&fib * &luc, &luc * &luc + 2 * k)
}

fn fib_lucas(n: u32) -> BigInt {
    return fib_luc_priv(n as i32).0;
}

/// Fast doubling.
/// F(2n) = F(n) * (2*F(n+1) - F(n))
/// F(2n+1) = F(n+1)^2 + F(n)^2
/// for further info see https://www.nayuki.io/page/fast-fibonacci-algorithms
fn fast_fibonacci_doubling(n: u32) -> BigInt {
    let mut a = BigInt::from(0);
    let mut b = BigInt::from(1);
    let mut bit = 1 << (31 - n.leading_zeros());
    while bit != 0 {
        let d = &a * (&b * 2u32 - &a);
        let e = &a * &a + &b * &b;
        a = d;
        b = e;
        if (n & bit) != 0 {
            let c = &a + &b;
            a = b;
            b = c;
        }
        bit >>= 1;
    }
    a
}

fn matrix_multiply(x: &[BigInt], y: &[BigInt]) -> Vec<BigInt> {
    vec![
        &x[0] * &y[0] + &x[1] * &y[2],
        &x[0] * &y[1] + &x[1] * &y[3],
        &x[2] * &y[0] + &x[3] * &y[2],
        &x[2] * &y[1] + &x[3] * &y[3],
    ]
}

/// Compute the power of a matrix (row-major order)
fn matrix_pow(matrix: Vec<BigInt>, n: u32) -> Vec<BigInt> {
    let mut result = vec![
        BigInt::from(1),
        BigInt::from(0),
        BigInt::from(0),
        BigInt::from(1),
    ];
    let mut n = n;
    let mut matrix = matrix;

    while n != 0 {
        // Exponentiation by squaring
        if n % 2 != 0 {
            result = matrix_multiply(&result, &matrix);
        }
        n /= 2;
        matrix = matrix_multiply(&matrix, &matrix);
    }

    result
}

/// Fast matrix.
/// [1 1]^n   [F(n+1) F(n)  ]
/// [1 0]   = [F(n)   F(n-1)]
/// for further info see https://www.nayuki.io/page/fast-fibonacci-algorithms
fn fast_fibonacci_matrix(n: u32) -> BigInt {
    let matrix = vec![
        BigInt::from(1),
        BigInt::from(1),
        BigInt::from(1),
        BigInt::from(0),
    ];
    matrix_pow(matrix, n)[1].clone()
}

/// Simple slow method, using dynamic programming.
/// F(n+2) = F(n+1) + F(n)
/// for further info see https://www.nayuki.io/page/fast-fibonacci-algorithms
fn slow_fibonacci(n: u32) -> BigInt {
    let mut a = BigInt::from(0);
    let mut b = BigInt::from(1);

    for _ in 0..n {
        let c = &a + &b;
        a = b;
        b = c;
    }
    a
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let max_n: u32 = if args.len() >= 2 {
        match args[1].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Error: Number must be an integer");
                return Ok(());
            }
        }
    } else {
        25_000_000
    };
    let timeout_secs: u64 = if args.len() >= 3 {
        match args[2].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Error: Timeout must be an integer");
                return Ok(());
            }
        }
    } else {
        60
    };

    let csv_filename = "fibonacci_times.csv";
    let mut file = BufWriter::new(File::create(csv_filename)?);
    writeln!(file, "n,lucas[µs],doubling[µs],matrix[µs],slow[µs]")?;

    let mut disable_slow_fib = false;
    let mut n: u32 = 1000;
    while n <= max_n {
        print!(
            "Computing {}th Fibonacci number ... ",
            n.to_string()
                .chars()
                .rev()
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join(",")
                .chars()
                .rev()
                .collect::<String>()
        );
        std::io::stdout().flush().unwrap();
        let mut results: Vec<BigInt> = Vec::new();

        let start_time = Instant::now();
        let x = fib_lucas(n);
        results.push(x);
        let lucas_time: u128 = start_time.elapsed().as_micros();

        let start_time = Instant::now();
        let x = fast_fibonacci_doubling(n);
        results.push(x);
        let doubling_time: u128 = start_time.elapsed().as_micros();

        let start_time = Instant::now();
        let x = fast_fibonacci_matrix(n);
        results.push(x);
        let matrix_time: u128 = start_time.elapsed().as_micros();

        let start_time = Instant::now();
        let slow_time: u128 = if !disable_slow_fib {
            let t: u128 = match with_timeout(
                tokio::task::spawn_blocking(move || slow_fibonacci(n)),
                timeout_secs,
                "slow_fibonacci()",
            )
            .await
            {
                Some(Ok(x)) => {
                    results.push(x.clone());
                    start_time.elapsed().as_micros()
                }
                _ => {
                    disable_slow_fib = true;
                    0
                }
            };
            t
        } else {
            0
        };
        println!("Slow time: {} ms", &slow_time / 1000);

        if results.iter().all(|result| result == &results[0]) {
            println!(
                "All methods computed the same result. Saving to {} ...",
                csv_filename
            );
            writeln!(
                file,
                "{},{},{},{},{}",
                n, lucas_time, doubling_time, matrix_time, slow_time
            )?;
        } else {
            println!("Different results computed.");
        }
        n *= 2;
    }

    Ok(())
}
