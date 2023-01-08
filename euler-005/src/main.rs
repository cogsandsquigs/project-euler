fn main() {
    let smallest_multiple: u64 = (1..=20).fold(1, |acc, x| acc * x / gcd(acc, x));

    println!("{}", smallest_multiple);
}

/// Returns the greatest common divisor of `a` and `b`, using Euclid's algorithm.
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
