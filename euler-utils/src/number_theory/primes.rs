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

/// Returns all prime numbers less than or equal to `n`. Uses the
/// Sieve of Eratosthenes.
pub fn primes_up_to(n: u64) -> Vec<u64> {
    let mut primes = vec![2];
    let mut candidate = 3;

    while candidate <= n {
        if primes.iter().all(|&p| candidate % p != 0) {
            primes.push(candidate);
        }
        candidate += 2;
    }

    primes
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

    #[test]
    fn test_primes_up_to() {
        assert_eq!(primes_up_to(10), vec![2, 3, 5, 7]);
        assert_eq!(
            primes_up_to(100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }
}
