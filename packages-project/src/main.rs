// importing the Asparagus struct
use crate::garden::vegetables::{Asparagus};

pub mod garden;

fn main() {
    let mut plant = Asparagus::new();
    println!("Hello, I am growing {:?}", plant);

    plant.grow();

    garden::water_plants();
    plant.grow();
    garden::water_plants();
    plant.grow();
    garden::water_plants();

    garden::harvest_plants();
}
