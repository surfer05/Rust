// A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main(){
    let plant = Asparagus{};
    println!("I'm growing up{:?}",plant);
}


