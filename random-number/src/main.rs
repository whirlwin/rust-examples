use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let n: i64 = rng.gen_range(1..=10);
    println!("Random integer: {}", n);
}