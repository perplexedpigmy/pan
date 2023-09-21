use crate::common::Gram;

#[derive(Debug)]
pub struct Salt {
    pub weight: Gram
}

impl std::fmt::Display for Salt{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Salt: {}", self.weight)
    }
}


