use crate::garder::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus{};
    println!("I'm growing {plant:?}!");
}
