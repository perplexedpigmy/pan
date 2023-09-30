use crate::common::Gram;
use crate::common::Percent;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;

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

impl Add<Gram> for Flour{
    type Output = Self;
    fn add(self, other: Gram) -> Self {
        Flour {
            name: self.name,
            weight: self.weight + other.0
        }
    }
}

impl Add<i32> for Flour {
    type Output = Self;
    fn add(self, other: i32) -> Self {
        Flour{
            name: self.name,
            weight: self.weight + other as f32
        }
    }
}

impl Add<f32> for Flour {
    type Output = Self;
    fn add(self, other: f32) -> Self {
        Flour{
            name: self.name,
            weight: self.weight + other 
        }
    }
}

impl Mul<i32> for Flour {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Flour{
            name: self.name,
            weight: self.weight * other as f32 
        }
    }
}

impl Mul<f32> for Flour{
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Flour{
            name: self.name,
            weight: self.weight * other
        }
    }
}

impl Div<i32> for Flour {
    type Output = Self;
    fn div(self, other: i32) -> Self {
        Flour{
            name: self.name,
            weight: self.weight / other as f32
        }
    }
}

impl Div<f32> for Flour{
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Flour{
            name: self.name,
            weight: self.weight / other
        }
    }
}

impl PartialEq for Flour{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.weight == other.weight
    }
}

impl PartialEq<Gram> for Flour{
    fn eq(&self, other: &Gram) -> bool {
        self.weight == *other
    }
}


type FlourPercentage = Percent<1, 100>;
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
        let flour = Flour{
            name: FLOUR_TYPE.into(), 
            weight: 500.into()
        };
        assert_eq!(flour.name, FLOUR_TYPE);
        assert_eq!(flour.weight, 500.into());
    }

    #[test]
    fn test_flour_add_gram() {
        let flour = Flour{
            name: FLOUR_TYPE.into(), 
            weight: 500.into()
        };
        assert_eq!(flour + Gram(30.0), Gram(530.0));
    }

    #[test]
    fn test_flour_add_i32() {
        let flour = Flour{
            name: FLOUR_TYPE.into(), 
            weight: 500.into()
        };
        assert_eq!(flour + 30, Gram(530.0));
    }

    #[test]
    fn test_flour_add_f32() {
        let flour = Flour{
            name: FLOUR_TYPE.into(), 
            weight: 500.into()
        };
        assert_eq!(flour + 30.5, Gram(530.5));
    }

    #[test]
    fn test_flour_mul_i32() {
        let flour = Flour{
            name: FLOUR_TYPE.into(), 
            weight: 200.into()
        };
        assert_eq!(flour * 3, Gram(600.0));
    }

    #[test]
    fn test_flour_mul_f32() {
        let flour = Flour{
            name: FLOUR_TYPE.into(), 
            weight: 200.into()
        };
        assert_eq!(flour * 3.5, Gram(700.0));
    }

    #[test]
    fn test_flour_div_i32() {
        let flour = Flour{
            name: FLOUR_TYPE.into(), 
            weight: 600.into()
        };
        assert_eq!(flour / 3, Gram(200.0));
    }

    #[test]
    fn test_flour_div_f32() {
        let flour = Flour{
            name: FLOUR_TYPE.into(), 
            weight: 700.into()
        };
        assert_eq!(flour / 3.5, Gram(200.0));
    }
}
