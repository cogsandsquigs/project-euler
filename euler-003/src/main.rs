use std::vec::IntoIter;

fn main() {
    println!(
        "{}",
        factors(600851475143)
            .last() // The list is sorted from lowest to highest, so the last element is the largest.
            .unwrap()
    );
}

/// Returns all the prime factors of `n`. These are sorted from lowest to highest.
fn factors(n: u64) -> IntoIter<u64> {
    let sqrt_n = (n as f64).sqrt() as u64;
    let mut primes = vec![];
    let mut n = n;

    // We only need to check up to `sqrt_n` numbers because the largest factor
    // of `n` is `sqrt_n` or less.
    for i in 2..=sqrt_n {
        if n % i == 0 {
            primes.push(i);
            n /= i;
        }
    }

    primes.into_iter()
}
