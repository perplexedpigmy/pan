use crate::common::Gram;

#[derive(Debug)]
pub struct Water {
    pub weight: Gram
}

impl std::fmt::Display for Water{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Water: {}", self.weight)
    }
}
