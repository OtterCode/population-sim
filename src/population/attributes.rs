use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeData {
    ratio: f64,
    count: u64,
    flags: u64,
    name: String
}


impl AttributeData {
    fn advance_year(&self, consequences: &HashMap<&str, Consequence>) -> (AttributeData, Consequence) {
        unimplemented!()
    }
}


struct Consequence {
    years_remaining: u64,
}
