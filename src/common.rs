use std::fmt;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;
use std::cmp::PartialEq;

fn round(value: f32, digits: i32) -> f32 {
    let multiplier = 10_f32.powi(digits);
    (value * multiplier).round() / multiplier
}

/// A unit of weight
/// used as the basis for ingridient measurements
///
/// # Fields
///
/// * `value`: The value of the gram, in grams.
///
/// # Example
///
///
/// let flour_weight = Gram(500.0);
/// assert_eq!(flour_weight.0, 500.0);
///
/// let water_weight = Gram::From(12.0);
/// assert_eq!(water_weight.0, 12.0);
///
/// let starter_weight: Gram = 10.into();
/// assert_eq!(starter_weight.0, 10.0);
#[derive(Debug, Clone, Copy)]
pub struct Gram(pub f32);

impl From<i32> for Gram {
    fn from(value: i32) -> Self {
        Gram(value as f32)
    }
}

impl From<f32> for Gram {
    fn from(value: f32) -> Self {
        Gram(value)
    }
}


// Operator overloading

impl Add<Gram> for Gram {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Gram(self.0 + other.0)
    }
}

impl Add<i32> for Gram {
    type Output = Self;
    fn add(self, other: i32) -> Self {
        Gram(self.0 + other as f32)
    }
}

impl Add<f32> for Gram {
    type Output = Self;
    fn add(self, other: f32) -> Self {
        Gram(self.0 + other )
    }
}

impl Sub<Gram> for Gram {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self - other.0
    }
}

impl Sub<i32> for Gram {
    type Output = Self;
    fn sub(self, other: i32) -> Self {
        self - other as f32
    }
}

impl Sub<f32> for Gram {
    type Output = Self;
    fn sub(self, other: f32) -> Self {
        assert!((self.0 - other) > 0.0, "negative weight not allowed. Subtration {} - {} = {} panics", self.0, other, self.0 - other);

        Gram(self.0 - other )
    }
}

impl Mul<i32> for Gram {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Gram(round(self.0 * other as f32, 2))
    }
}

impl Mul<f32> for Gram {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Gram(round(self.0 * other, 2))
    }
}

impl<const MIN : usize, const MAX: usize> Mul<Percent<MIN,MAX>> for Gram {
    type Output = Self;
    fn mul(self, other: Percent<MIN,MAX>) -> Self {
        Gram(round(self.0 * (other.0 as f32 / 100.0), 2))
    }
}

impl Div<i32> for Gram {
    type Output = Self;
    fn div(self, other: i32) -> Self {
        Gram(round(self.0 / other as f32, 2))
    }
}

impl Div<f32> for Gram {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Gram(round(self.0 / other,2) )
    }
}

impl<const MIN : usize, const MAX: usize> Div<Percent<MIN, MAX>> for Gram {
    type Output = Self;
    fn div(self, other: Percent<MIN,MAX>) -> Self {
        Gram(round(self.0 / (other.0 as f32 / 100.0),2) )
    }
}

impl PartialEq for Gram {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}


impl fmt::Display for Gram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} g", self.0) 
    }
}


// A limit bounded percentage abstraction.
// Fractional percentages are not supported.
#[derive(Debug, Clone, Copy)]
pub struct Percent<const MIN: usize, const MAX: usize> (usize);


impl<const MIN: usize, const MAX: usize> Percent<MIN, MAX> {
    pub fn new(value: usize) -> Self {
        assert!(MIN <= value && value <= MAX, "value must be between {} and {} including", MIN, MAX);
        Self ( value )
    }
}

impl<const MIN : usize, const MAX: usize> From<usize> for Percent<MIN, MAX> {
    fn from(value: usize) -> Self {
        Percent::new(value)
    }
}

impl<const MIN : usize, const MAX: usize> From<i32> for Percent<MIN, MAX> {
    fn from(value: i32) -> Self {
        Percent::new(value as usize)
    }
}

impl<const MIN : usize, const MAX: usize> From<f32> for Percent<MIN, MAX> {
    fn from(value: f32) -> Self {
        Percent::new((value * 100.0) as usize)
    }
}

impl<const MIN: usize, const MAX: usize> fmt::Display for Percent<MIN, MAX>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.0) 
    }
}


impl<const MIN : usize, const MAX: usize> Div<Percent<MIN, MAX>> for f32 {
    type Output = Self;
    fn div(self, other: Percent<MIN,MAX>) -> Self {
        round(self / (other.0 as f32 / 100.0),2) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gram_constructor() {
        let weight = Gram(500.0);
        assert_eq!(weight.0, 500.0);
    }

    #[test]
    fn test_gram_from_int() {
        let weight = Gram::from(312);
        assert_eq!(weight.0, 312.0);
    }

    #[test]
    fn test_gram_from_float() {
        let weight = Gram::from(312.5);
        assert_eq!(weight.0, 312.5);
    }
    
    #[test]
    fn test_gram_into_int() {
        let weight: Gram = 11.into();
        assert_eq!(weight.0, 11.0);
    }

    #[test]
    fn test_gram_into_float() {
        let weight: Gram = 11.3.into();
        assert_eq!(weight.0, 11.3);
    }

    #[test]
    fn test_gram_add() {
        let weight1: Gram = 11.3.into();
        let weight2: Gram = 11.into();
        assert_eq!(weight1 + weight2, 22.3.into());
    }

    #[test]
    fn test_gram_add_i32() {
        let weight1: Gram = 11.3.into();
        assert_eq!(weight1 + 3, 14.3.into());
    }

    #[test]
    fn test_gram_add_f32() {
        let weight1: Gram = 11.3.into();
        assert_eq!(weight1 + 3.5, 14.8.into());
    }


    #[test]
    fn test_gram_mul_i32() {
        let weight1: Gram = 11.3.into();
        assert_eq!(weight1 * 3, 33.9.into());
    }

    #[test]
    fn test_gram_mul_f32() {
        let weight1: Gram = 11.4.into();
        assert_eq!(weight1 * 3.6, 41.04.into());
    }

    #[test]
    fn test_gram_div_i32() {
        let weight1: Gram = 33.9.into();
        assert_eq!(weight1 / 3, 11.3.into());
    }

    #[test]
    fn test_gram_div_f32() {
        let weight1: Gram = 41.04.into();
        assert_eq!(weight1 / 3.6, 11.4.into());
    }
}


