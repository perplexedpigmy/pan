use crate::common::Gram;

#[derive(Debug)]
/// A sourdough starter
///
/// Sourdough is composed of flour and water and a hefty amount of healthy lactobacillus bacteria.
/// The flour and water are tracked, the bacteria takes care of itself.
///
/// The parameters thet make affect the effectiveness and properties of the starter are 
/// - Hydration 
/// - Flour/s used
/// - Temperture
/// - Environment 
///
/// The above has effect on the bateria/yiest profile of the starter, but this abstraction only
/// trackes hydration which is the ratio of water to flour.
///
/// # fields
///
/// * `flour`: The flour content
/// * `water`: The water content
///
///
pub struct Starter {
    flour: Gram,
    water: Gram,
}

impl Starter {
    /**
    * 
    * @param total_flour(f32) total flour weight in grams
    * @param hydration(f32) hydration percentage
    * @param percent_starter(f32) percentage of starter flour from total flour
    */
    /// Create a new starter
    ///
    /// # Fields 
    ///
    /// `flour`: The flour weight 
    /// `water`: The water weight
    ///
    /// # Example
    /// 
    /// let starter = Starter::create(200.0, 0.7, 0.1);
    /// ```
    /// assert_eq!(1, 1)
    /// ````
    pub fn create(total_weight: f32, hydration: f32, percent_starter: f32) -> Starter {
        let starter_weight = total_weight * percent_starter;
        let flour_ratio = 1.0 / hydration; // The four ratio for each 1 unit of water
        let portion = starter_weight / ( flour_ratio + 1.0);

        let flour_in_gr = portion * flour_ratio;
        let water_in_gr = portion;
        
        Starter {
            flour: Gram(flour_in_gr),
            water: Gram(water_in_gr),
        }
    }

    pub fn get_hydration(&self) -> f32 {
        self.water.0 / self.flour.0 
    }
    
    pub fn get_flour_weight(&self) -> f32 {
        self.flour.0
    }
   
    pub fn get_water_weight(&self) -> f32 {
        self.water.0
    }

    pub fn get_total_weight(&self) -> f32 {
        self.flour.0 + self.water.0
    }

}

impl std::fmt::Display for Starter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Starter({}% hydration):\n     Flour {:.2}\n     Water {:.2}\n         = {:.2} g", 
            self.water.0 / self.flour.0 * 100.0, 
            self.flour, 
            self.water, 
            self.get_total_weight())
    }
}

