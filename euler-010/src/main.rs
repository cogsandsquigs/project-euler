use euler_utils::number_theory::primes::primes_up_to;

fn main() {
    let sum: u64 = primes_up_to(2_000_000).iter().sum();

    println!("{sum}");
}
