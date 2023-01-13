// Given that we know a^2 + b^2 = c^2, we can rewrite a + b + c = 1000 as
// a^2 + b^2 + sqrt(a^2 + b^2) = 1000
// ...algebra...
// x + y - (1/1000) * x * y = 500
// ...more algebra...
// y = (500 - x)/(1 - x/1000)
//
// Now we solve for integer solutions to this equation. We know that x < 1000 and
// y < 1000, so we can just brute force it.

fn main() {
    for x in 1..1000 {
        let y = (500 - x) as f64 / (1.0 - x as f64 / 1000.0);
        if y.fract() == 0.0 {
            let y = y as i32;
            let z = 1000 - x - y;
            println!("{}", x * y * z);
            return;
        }
    }
}
