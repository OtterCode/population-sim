mod population;
mod skew_normal;

//#[macro_use]
//extern crate serde_derive;
//extern crate serde_yaml;

use std::iter::repeat;
use std::env::current_dir;
use ::population::Population;
//use serde_yaml;

fn main() {

    let mut path = current_dir().unwrap();
    path.push("inpop.yml");

    //let fertility = fertility::Fertility::new(15, 50);

    //let babbies: f32 = (0..50).map(|x| gen1.births(x, 2.1)).sum();

    let tens = repeat(100000).take(1);
    let zeroes = repeat(0usize).take(15);
    let pop_: Vec<(usize, usize)> = zeroes.chain(tens).map(|x| (x, x)).collect();

    let mut pop = Population::new(pop_);


    for x in 0..200 {
        pop = pop.advance_year();
        println!("{:?}", pop.total_pop());
    }
    println!("Males: {:?}", pop.total_male());
    println!("Females: {:?}", pop.total_female());

}
