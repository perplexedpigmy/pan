use crate::common::Gram;
use crate::common::Percent;

pub type SaltPercentage = Percent<1, 4>;

#[derive(Debug)]
pub struct Salt {
    pub weight: Gram
}

impl std::fmt::Display for Salt{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Salt: {}", self.weight)
    }
}


