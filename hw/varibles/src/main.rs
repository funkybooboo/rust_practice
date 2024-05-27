
const READY_AMOUNT: i32 = 2;
const STARTING_MISSILES: i32 = 8;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    println!("Firing {} of my {} missles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
    
