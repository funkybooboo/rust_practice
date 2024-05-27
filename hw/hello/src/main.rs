use hello::greet;
use rand::{Rng, thread_rng};

fn main() {
    greet();
    println!("{}", thread_rng().gen_range(0..100));
}
