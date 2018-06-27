#[derive(Debug)]
pub struct Members {
    pub males: usize,
    pub females: usize,
    pub male_dying: f32,
    pub female_dying: f32,
}


impl Members {
    pub fn total(&self) -> usize {
        self.males + self.females
    }
}
