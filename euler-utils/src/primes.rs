/// Returns the `n`th prime number.
pub fn nth_prime(n: u64) -> u64 {
    let mut primes = vec![2];
    let mut candidate = 3;

    while primes.len() < n as usize {
        if primes.iter().all(|&p| candidate % p != 0) {
            primes.push(candidate);
        }
        candidate += 2;
    }

    primes[n as usize - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(6), 13);
        assert_eq!(nth_prime(10), 29);
        assert_eq!(nth_prime(100), 541);
        assert_eq!(nth_prime(1000), 7919);
        assert_eq!(nth_prime(10000), 104729);
    }
}
