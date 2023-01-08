// What annoys me about this solution is that it's not as beautiful as it could be. It's
// really just a brute force through all number combinations, which is not very elegant.

fn main() {
    let largest = (100..1000)
        .flat_map(|i| (100..1000).map(move |j| i * j))
        .filter(|&n| is_number_palindromic(n))
        .max()
        .unwrap();

    println!("{}", largest);
}

/// Returns `true` if `n` is a palindromic number, `false` otherwise.
fn is_number_palindromic(n: u64) -> bool {
    let n_len = (n as f64).log10() as u32 + 1;

    // We only need to check the first half of the digits because the second half is
    // the same as the first half, but reversed.
    for i in 0..n_len / 2 {
        let left = get_digit(n, i);
        let right = get_digit(n, n_len - i - 1);

        if left != right {
            return false;
        }
    }

    true
}

/// Returns the `i`th digit of `n`, where `i` is zero-indexed.
fn get_digit(n: u64, i: u32) -> u64 {
    let n_len = (n as f64).log10() as u32 + 1;
    let i = n_len - i - 1;
    let divisor = 10_u64.pow(i);
    (n / divisor) % 10
}
