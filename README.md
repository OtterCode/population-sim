# pop_sim

> An Efficient, Fast, Large-Scale Population Simulator

PopSim uses deterministic statistical methods to allow you to simulate arbitrarily large or
small populations. At the moment it simulates a static birth and death rate. In the future it
will allow you to add a timeline of events to simulate changes in birth/death rate,
environmental carrying capacity, immigration, and catastrophic events.


```rust
    let mut pop = pop_sim::population::Population::new(vec![ ( 10_000, 10_000 ) ]);

    for _ in 0..1000 {
        pop = pop.advance_year();
        println!("Males: {:?}", pop.total_male());
        println!("Females: {:?}", pop.total_female());
    }
```
