mod cohort;
mod fertility;
mod skew_normal;

//#[macro_use]
//extern crate serde_derive;
//extern crate serde_yaml;

use std::env::current_dir;
use cohort::Cohort;
//use serde_yaml;

fn main() {

    let mut path = current_dir().unwrap();
    path.push("inpop.yml");

    let fertility = fertility::Fertility::new(15, 50);

    let gen1 = Cohort { members: 200, birth_year: 5, fertility };

    let babbies: f32 = (0..50).map(|x| gen1.births(x, 2.1)).sum();

    println!("{:?}", babbies);

}
