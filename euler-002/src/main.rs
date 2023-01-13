use std::iter::from_fn;

fn main() {
    let sum: u64 = fibs()
        .take_while(|&x| x < 4_000_000)
        .filter(|&x| x % 2 == 0)
        .sum();

    println!("{sum}");
}

/// Infinite iterator of Fibonacci numbers.
fn fibs() -> impl Iterator<Item = u64> {
    let mut a = 0;
    let mut b = 1;
    from_fn(move || {
        let c = a + b;
        a = b;
        b = c;
        Some(c)
    })
}
