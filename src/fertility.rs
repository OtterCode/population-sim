use ::skew_normal::Curve;

#[derive(Debug)]
pub struct Fertility {
    curve: Vec<f32>,
    min_age: usize,
    max_age: usize,
}

impl Fertility {
    pub fn new(min_age: usize, max_age: usize) -> Fertility {
        let curve = normalize(Curve::new(-3.0).limits_range(-2.0, 3.0, max_age - min_age));
        println!("{:?}", curve);
        Fertility { curve, min_age, max_age }
    }

    pub fn birth_rate(&self, age: usize, total_fertility_rate: f32) -> f32 {
        if age < self.min_age || age > self.max_age { 0.0 }
        else { self.curve.get(age - self.min_age).unwrap_or(&0.0) * total_fertility_rate }
    }
}

// We are normalizing against the mean, not space.
// We want to multiply any index by the TFR, and get the instantaneous birth rate.
fn normalize(vector: Vec<f32>) -> Vec<f32> {

    let sum: f32  = vector.iter().sum::<f32>();

    vector.iter().map(|x| x/sum).collect()

}