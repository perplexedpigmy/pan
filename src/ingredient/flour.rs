use gen_seq_first_elem;
use crate::common::Gram;
use crate::common::Percent;
use crate::ingredient::starter::Starter;
use capitalize::Capitalize;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;

pub type FlourPercentage = Percent<1, 100>;

#[derive(Debug, PartialEq)]
pub enum Measure {
    Weight(Gram),
    Ratio(FlourPercentage),
}

impl std::fmt::Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Measure::Weight(weight) => format!("{}", weight),
            Measure::Ratio(ratio) => format!("{}", ratio),
        })
    }
}

#[derive(Debug)]
pub struct Flour {   
    pub name: String,
    pub measure: Measure,

    /// Actual weight of flour to insert ( starter flour excluded )
    pub weight: Option<Gram>,
}


impl std::fmt::Display for Flour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.weight {
            Some(weight) => format!("{}({}): {}", self.name, self.measure, weight),
            None => format!("{}: {}", self.name, self.measure),
        })
    }
}

impl Flour {
    pub fn new(name: &str, measure: Measure) -> Self {
       Self {
         name: name.to_string().capitalize(),
         measure,
         weight: None, 
        } 
    }
}

#[derive(Debug)]
pub struct FlourMix {
    pub total_weight: Option<Gram>,
    pub flours: Vec<Flour>,
}

impl std::fmt::Display for FlourMix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Flour Blend:\n     {}",
          self.flours.iter()
                     .map(|i| i.to_string()).collect::<Vec<String>>()
                     .join("\n     ")
        )
    }
}

impl FlourMix {
    pub fn new(total_weight: Gram) -> Self {
        FlourMix {
           total_weight: Some(total_weight),
           flours: Vec::new(),
        }
    }

    /// Calculates the flour weight and Deduce the starter from the flour.
    /// For now there is only one stategy implemented.
    /// All the starter's flour is deduced from the first flour.
    ///   Assumptions:
    ///   1. There is at least 1 flour 
    ///   2. The flour's weight is greater than the starter's weight.
    ///
    ///   TODO: Possible to add more strategies, the question is how to communicate it via command
    ///   line arguments.
    pub fn apply_starter(mut self, starter: &Starter) -> Self {
       let mut seq = gen_seq_first_elem!(starter.get_flour_weight(), 0.0);
       for flour in self.flours.iter_mut() {

            flour.weight = match flour.measure {
                Measure::Weight(weight) => Some(weight - seq.next().unwrap_or(0.0)),
                Measure::Ratio(ratio) => Some(self.total_weight.unwrap() * ratio - seq.next().unwrap_or(0.0)),
            }
        } 

        self
    }
    
    /// Adds a new flour by weight to the mix, providing the list does not have any flour by ratio,
    /// a zero weight is not allowed.
    fn add_flour_weight(&mut self, weight: &Gram) {
        let all_weights = self.flours.iter().all( |f| 
            match f.measure {
                Measure::Weight(_) => true, 
                Measure::Ratio(_) => false
            });
        assert!(all_weights, "Adding flour by ratio is not possible when flour mix is by weight");
        assert!(weight.0 > 0.0, "Flour weight must be greater than 0");

        self.total_weight = match self.total_weight {
            Some(tw) => Some(tw + *weight),
            None => Some(*weight),
        }
    }

    /// Add a flour to the flour Mix list, providing that all flours in list are measured by
    /// percentage(ratio), a zero percentage is not allowed.
    fn add_flour_ratio(&mut self, ratio: &FlourPercentage) {
        let total_ratio: FlourPercentage = self.flours.iter().map( |f| 
            match f.measure {
                Measure::Weight(_) => panic!("Adding flour by ratio is not possible when flour mix is by weight"),
                Measure::Ratio(ratio) => ratio,
            }).sum();
        assert!(total_ratio + *ratio <= FlourPercentage::MAX, 
            "Flour mix percentage must not exceed {}, provided {}", FlourPercentage::MAX, total_ratio + *ratio);
        assert!(*ratio > FlourPercentage::ZERO, "Flour ratio must be greater than 0");
    }

    /// Add a flour to the flour Mix
    /// Flours can be added by weight or ratio
    /// Intermixing these two requires a development of conventional arithemtic 
    /// and is not yet supported
    pub fn add_flour(&mut self, flour: Flour) {
        match flour.measure {
            Measure::Weight(weight) => self.add_flour_weight(&weight),
            Measure::Ratio(ratio) => self.add_flour_ratio(&ratio)
        }
        self.flours.push(flour);
    }
}

// ========================================
#[derive(Debug)]
pub struct FlourOld {
    pub name: String,
    pub weight: Gram,
}

impl std::fmt::Display for FlourOld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Flour({}): {}", self.name, self.weight)
    }
}

impl Add<Gram> for FlourOld {
    type Output = Self;
    fn add(self, other: Gram) -> Self {
        Self {
            name: self.name,
            weight: self.weight + other.0
        }
    }
}

impl Add<i32> for FlourOld {
    type Output = Self;
    fn add(self, other: i32) -> Self {
        Self {
            name: self.name,
            weight: self.weight + other as f32
        }
    }
}

impl Add<f32> for FlourOld {
    type Output = Self;
    fn add(self, other: f32) -> Self {
        Self {
            name: self.name,
            weight: self.weight + other 
        }
    }
}

impl Mul<i32> for FlourOld {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Self {
            name: self.name,
            weight: self.weight * other as f32 
        }
    }
}

impl Mul<f32> for FlourOld {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            name: self.name,
            weight: self.weight * other
        }
    }
}

impl Div<i32> for FlourOld {
    type Output = Self;
    fn div(self, other: i32) -> Self {
        Self{
            name: self.name,
            weight: self.weight / other as f32
        }
    }
}

impl Div<f32> for FlourOld{
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self{
            name: self.name,
            weight: self.weight / other
        }
    }
}

impl PartialEq for FlourOld{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.weight == other.weight
    }
}

impl PartialEq<Gram> for FlourOld {
    fn eq(&self, other: &Gram) -> bool {
        self.weight == *other
    }
}


#[derive(Debug)]
pub struct FlourItem {
    pub name: String,
    pub percentage: FlourPercentage,
}

impl FlourItem {
    pub fn new(name: &str, percentage: FlourPercentage) -> FlourItem {
        FlourItem {
            name: String::from(name),
            percentage
        }
    }
}

#[cfg(test)]
mod tests {
    const FLOUR_TYPE: &str = "TEST";
    use super::*;

    #[test]
    fn test_flour_constructor() {
        let flour = FlourOld{
            name: FLOUR_TYPE.into(), 
            weight: 500.into()
        };
        assert_eq!(flour.name, FLOUR_TYPE);
        assert_eq!(flour.weight, 500.into());
    }

    #[test]
    fn test_flour_add_gram() {
        let flour = FlourOld{
            name: FLOUR_TYPE.into(), 
            weight: 500.into()
        };
        assert_eq!(flour + Gram(30.0), Gram(530.0));
    }

    #[test]
    fn test_flour_add_i32() {
        let flour = FlourOld{
            name: FLOUR_TYPE.into(), 
            weight: 500.into()
        };
        assert_eq!(flour + 30, Gram(530.0));
    }

    #[test]
    fn test_flour_add_f32() {
        let flour = FlourOld{
            name: FLOUR_TYPE.into(), 
            weight: 500.into()
        };
        assert_eq!(flour + 30.5, Gram(530.5));
    }

    #[test]
    fn test_flour_mul_i32() {
        let flour = FlourOld{
            name: FLOUR_TYPE.into(), 
            weight: 200.into()
        };
        assert_eq!(flour * 3, Gram(600.0));
    }

    #[test]
    fn test_flour_mul_f32() {
        let flour = FlourOld{
            name: FLOUR_TYPE.into(), 
            weight: 200.into()
        };
        assert_eq!(flour * 3.5, Gram(700.0));
    }

    #[test]
    fn test_flour_div_i32() {
        let flour = FlourOld{
            name: FLOUR_TYPE.into(), 
            weight: 600.into()
        };
        assert_eq!(flour / 3, Gram(200.0));
    }

    #[test]
    fn test_flour_div_f32() {
        let flour = FlourOld{
            name: FLOUR_TYPE.into(), 
            weight: 700.into()
        };
        assert_eq!(flour / 3.5, Gram(200.0));
    }
}
