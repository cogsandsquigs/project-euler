use euler_utils::number_theory::gcd;

fn main() {
    let smallest_multiple: u64 = (1..=20).fold(1, |acc, x| acc * x / gcd(acc, x));

    println!("{smallest_multiple}");
}
