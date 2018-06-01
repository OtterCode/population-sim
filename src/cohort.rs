use std::fmt;
use ::fertility::Fertility;

#[derive(Debug)]
pub struct Cohort/*<'a>*/ {
    pub members: usize,
    pub birth_year: i32,
    //pub dimensions: Vec<&'a Dimension>,
    pub fertility: Fertility,
}

pub trait Dimension: fmt::Debug {
    fn to_yaml(&self) -> String {
        String::from("unimplemented")
    }
}

impl/*<'a>*/ Cohort/*<'a>*/ {
    pub fn births (&self, year: usize, tfr: f32) -> f32 {
        if (year as i32) < self.birth_year { 0.0 }
        else {
            let rate = self.fertility.birth_rate(year - self.birth_year as usize, tfr);
            rate * self.members as f32
        }
    }
}
