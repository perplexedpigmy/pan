use std::fmt;

/// A unit of weight
/// used as the basis for ingridient measurements
///
/// # Fields
///
/// * `value`: The value of the gram, in grams.
///
/// # Example
///
/// ```
///
/// let flour_weight = Gram(500.0);
/// assert_eq!(flour_weight.0, 500.0);
///
/// let water_weight = Gram::From(12.0);
/// assert_eq!(water_weight.0, 12.0);
///
/// let starter_weight: Gram = 10.into();
/// assert_eq!(starter_weight.0, 10.0);
/// ````
#[derive(Debug)]
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

impl fmt::Display for Gram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} g", self.0) 
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
}
