use ::skew_normal::Curve;

/// Fertility is measured with a truncated skew-normal curve, typically starting at age 15, peaking
/// at age 23-27, and slowly declining towards the cutoff at menopause. 
#[derive(Serialize, Deserialize, Debug)]
pub struct Fertility {
    curve: Vec<f32>,
    min_age: usize,
    max_age: usize,
}

impl Fertility {
    pub fn new(min_age: usize, max_age: usize) -> Fertility {
        let curve = normalize(Curve::new(-3.0).limits_range(-2.0, 3.0, max_age - min_age));
        Fertility { curve, min_age, max_age }
    }

    pub fn birth_rate(&self, age: usize, total_fertility_rate: f32) -> f32 {
        if age < self.min_age || age > self.max_age { 0.0 }
        else { self.curve.get(age - self.min_age).unwrap_or(&0.0) * total_fertility_rate }
    }
}

/// We are normalizing against the mean, not a vector space.
/// We want to multiply any index by the TFR, and get the instantaneous birth rate.
fn normalize(vector: Vec<f32>) -> Vec<f32> {

    let sum: f32  = vector.iter().sum::<f32>();

    vector.iter().map(|x| x/sum).collect()

}
