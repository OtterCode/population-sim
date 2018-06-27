pub mod cohort;

use self::cohort::Cohort;

#[derive(Debug)]
pub struct Population {
    cohorts: Vec<Cohort>,
    male_remainder: f64,
    female_remainder: f64,
    current_year: i32,
}

impl Population {
    pub fn new(initial_gens: Vec<(usize, usize)>) -> Population {
        let cohorts = initial_gens.iter().enumerate().map(|(year, (male, female))| {
            Cohort::new(*male, *female, -(year as i32 + 1))
        });
        Population { cohorts: cohorts.collect()
            , male_remainder: 0.0
            , female_remainder: 0.0
            , current_year: 0 }
    }

    pub fn advance_year(mut self) -> Population {
        let babies: Vec<f64> = self.cohorts.iter().map(|cohort| {
            cohort.births(self.current_year, 2.1)
        }).collect();


        let total_babies: f64 = babies.iter().sum::<f64>();
        let sex_ratio = 0.5;
        let males = (total_babies * sex_ratio) + self.male_remainder;
        let females = (total_babies * (1.0 - sex_ratio)) + self.female_remainder;

        let new_gen = Cohort::new
            ( males.trunc() as usize
            , females.trunc() as usize
            , self.current_year );
        self.male_remainder = males.fract();
        self.female_remainder = females.fract();


        self.cohorts.insert(0, new_gen);

        let year = self.current_year;
        self.cohorts = self.cohorts
            .into_iter()
            .filter_map(|cohort| cohort.perform_deaths(year, 0, 0))
            .collect();

        self.current_year += 1;


        self
    }

    pub fn total_pop(&self) -> usize {
        self.cohorts.iter().map(|cohort| { cohort.members.total() }).sum()
    }

    pub fn total_male(&self) -> usize {
        self.cohorts.iter().map(|cohort| { cohort.members.males }).sum()
    }

    pub fn total_female(&self) -> usize {
        self.cohorts.iter().map(|cohort| { cohort.members.females }).sum()
    }

}
