mod members;
mod heligman_pollard;
mod fertility;

use std::fmt;
use self::fertility::Fertility;
use self::heligman_pollard::HPMortalityModel;
use self::members::Members;
use super::attributes::AttributeData;

/**
A Cohort is a tight demographic grouping. Here, it's all people born within a given year.
Cohorts independently track their fertility curve, their gender-specific mortality rate, and their
separate gender populations.
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Cohort/*<'a>*/ {
    pub members: Members,
    pub birth_year: i32,
    pub attributes: Vec<AttributeData>,
    pub fertility: Fertility,
    pub male_mortality: HPMortalityModel,
    pub female_mortality: HPMortalityModel,
}


/// Used to track timed effects on population as well as demographic details
/// (such as political balance)
pub trait Dimension: fmt::Debug {
    fn to_yaml(&self) -> String {
        String::from("unimplemented")
    }
}


impl Cohort {

    pub fn new ( males: usize, females: usize, birth_year: i32 ) -> Cohort {
        let fertility = Fertility::new(15, 50);
        let members = Members { males, females, male_dying: 0.0, female_dying: 0.0 };
        let male_mortality = HPMortalityModel::baseline_male();
        let female_mortality = HPMortalityModel::baseline_female();
        let attributes = Vec::new();
        Cohort { members, birth_year, attributes, fertility, male_mortality, female_mortality }
    }

    /// Births are currently much simpler than deaths at the cohort level. This might change as
    /// male fertility is accounted for, as well as event effects.
    pub fn births (&self, year: i32, tfr: f32) -> f64 {
        if (year as i32) < self.birth_year { 0.0 }
        else {
            let rate = self.fertility.birth_rate((year - self.birth_year) as usize, tfr);
            rate as f64 * self.members.females as f64
        }
    }

    /// Kill off population members, carrying remainders over to the next year (simulating someone
    /// still living, but in declining health.) While remainder births are saved per population,
    /// remainder deaths are saved per cohort per gender. If population is zero, this will return
    /// None, signaling the removal of the generation.
    pub fn perform_deaths (
        mut self,
        year: i32,
        extraordinary_male_deaths: usize,
        extraordinary_female_deaths: usize
    ) -> Option<Cohort> {

        if (year as i32) < self.birth_year { return Some(self); }


        let male_mort = self.male_mortality.at_age(year - self.birth_year);
        let male_unnatural_survivors =
            self.members.males.saturating_sub(extraordinary_male_deaths) as f32;
        let male_natural_deaths = (male_unnatural_survivors * male_mort) + self.members.male_dying;
        let male_deaths = male_natural_deaths + extraordinary_male_deaths as f32;


        let female_mort = self.female_mortality.at_age(year - self.birth_year);
        let female_unnatural_survivors =
            self.members.females.saturating_sub(extraordinary_female_deaths) as f32;
        let female_natural_deaths =
            (female_unnatural_survivors * female_mort) + self.members.female_dying;
        let female_deaths = female_natural_deaths + extraordinary_female_deaths as f32;


        let males = self.members.males.saturating_sub(male_deaths.trunc() as usize);
        let females = self.members.females.saturating_sub(female_deaths.trunc() as usize);


        if males + females == 0 {
            println!("Everyone's dead in gen {:?}", self.birth_year);
            return None
        }


        let survivors = Members {
            males,
            male_dying: male_deaths.fract(),
            females,
            female_dying: female_deaths.fract(),
        };


        self.members = survivors;

        Some(self)
    }
}
