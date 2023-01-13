pub mod primes;

/// Returns the greatest common divisor of `a` and `b`, using Euclid's algorithm.
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
