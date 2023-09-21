use crate::common::Gram;

#[derive(Debug)]
pub struct Flour {
    pub name: String,
    pub weight: Gram,
}

impl std::fmt::Display for Flour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Flour({}): {}", self.name, self.weight)
    }
}


pub struct FlourItem {
    pub name: String,
    pub percentage: f32,
}

impl FlourItem {
    pub fn new(name: &str, percentage: f32) -> FlourItem {
        FlourItem {
            name: String::from(name),
            percentage
        }
    }
}
