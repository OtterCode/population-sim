extern crate pop_sim;

use std::iter::repeat;
use std::env::current_dir;
use pop_sim::population::Population;
//use serde_yaml;

fn main() {

    let mut path = current_dir().unwrap();
    path.push("inpop.yml");

    let tens = repeat(100000).take(1);

    let pop_: Vec<(usize, usize)> = tens.map(|x| (x, x)).collect();

    let mut pop = Population::new(pop_);


    for _x in 0..1000 {
        pop = pop.advance_year();
        println!("{:?}", pop.total_pop());
    }
    println!("Males: {:?}", pop.total_male());
    println!("Females: {:?}", pop.total_female());

}
