use crate::garden::vegetables::Asparagus;

// pub mod garden; to make the garden module public 
mod garden;

fn main() {
    let plant = Asparagus {};

    println!("{:?}", plant);
}
