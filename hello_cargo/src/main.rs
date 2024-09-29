use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Hello, world!");
    let now = SystemTime::now();
    println!("{:?}", now);
    print!("{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
}
