
use crate::common::Gram;
use crate::common::Percent;
use crate::common::Measure;
use crate::gen_seq_first_elem;
use crate::ingredient::starter::Starter;
use capitalize::Capitalize;
use colored::*;

pub type FlourPercentage = Percent<1, 100, 0>;

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> std::fmt::Display for Measure<MIN, MAX, DECIMALS> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Measure::Weight(weight) => format!("{}", weight.to_string().green().bold()),
        Measure::Ratio(ratio) => format!("{}", ratio.to_string().yellow().bold()),
      }
    )
  }
}

#[derive(Debug, Clone)]
pub struct Flour<const MIN: usize = 1, const MAX: usize = 100, const DECIMALS: usize = 0> {
  pub name: String,
  pub measure: Measure<MIN, MAX, DECIMALS>,

  /// Actual weight of flour to insert ( starter flour excluded )
  pub weight: Option<Gram>,
}

impl std::fmt::Display for Flour {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self.weight {
        Some(weight) => format!("{}({}): {}", self.name.bold(), self.measure, weight.to_string().green().bold()),
        None => format!("{}: {}", self.name.bold(), self.measure),
      }
    )
  }
}

impl<const MIN: usize, const MAX: usize, const DECIMALS: usize> Flour<MIN, MAX,DECIMALS> {
  pub fn new(name: &str, measure: Measure<MIN, MAX, DECIMALS>) -> Self {
    Flour {
      name: name.to_string().capitalize(),
      measure,
      weight: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct FlourMix {
  pub total_weight: Option<Gram>, // All flour inlcuding the starter
  pub flours: Vec<Flour>,
}

impl std::fmt::Display for FlourMix {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}\n     {}",
      "Flour Blend:".bold().blue(),
      self
        .flours
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
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
    let mut seq = gen_seq_first_elem!(starter.get_flour_weight(), 0.0.into());
    for flour in self.flours.iter_mut() {
      flour.weight = match flour.measure {
        Measure::Weight(weight) => Some(weight - seq.next().unwrap_or(0.0.into())),
        Measure::Ratio(ratio) => {
          Some(self.total_weight.unwrap() * ratio - seq.next().unwrap_or(0.0.into()))
        }
      }
    }

    self
  }

  /// Adds a new flour by weight to the mix, providing the list does not have any flour by ratio,
  /// a zero weight is not allowed.
  fn add_flour_weight(&mut self, weight: &Gram) {
    let all_weights = self.flours.iter().all(|f| match f.measure {
      Measure::Weight(_) => true,
      Measure::Ratio(_) => false,
    });
    assert!(
      all_weights,
      "Adding flour by ratio is not possible when flour mix is by weight"
    );
    assert!(*weight > 0.into(), "Flour weight must be greater than 0");

    self.total_weight = match self.total_weight {
      Some(tw) => Some(tw + *weight),
      None => Some(*weight),
    }
  }

  /// Add a flour to the flour Mix list, providing that all flours in list are measured by
  /// percentage(ratio), a zero percentage is not allowed.
  fn add_flour_ratio(&mut self, ratio: &FlourPercentage) {
    let total_ratio: FlourPercentage = self
      .flours
      .iter()
      .map(|f| match f.measure {
        Measure::Weight(_) => {
          panic!("Adding flour by ratio is not possible when flour mix is by weight")
        }
        Measure::Ratio(ratio) => ratio,
      })
      .sum();
    assert!(
      total_ratio + *ratio <= FlourPercentage::MAX,
      "Flour mix percentage must not exceed {}, provided {}",
      FlourPercentage::MAX,
      total_ratio + *ratio
    );
    assert!(
      *ratio > FlourPercentage::ZERO,
      "Flour ratio must be greater than 0"
    );
  }

  /// Add a flour to the flour Mix
  /// Flours can be added by weight or ratio
  /// Intermixing these two requires a development of conventional arithemtic
  /// and is not yet supported
  pub fn add_flour(&mut self, flour: Flour) {
    match flour.measure {
      Measure::Weight(weight) => self.add_flour_weight(&weight),
      Measure::Ratio(ratio) => self.add_flour_ratio(&ratio),
    }
    self.flours.push(flour);
  }

  pub fn derive_total_weight(&self) -> Gram {
    self.flours.iter().fold(Gram::ZERO, |acc, f| {
      let weight = match f.measure {
        Measure::Weight(w) => w,
        Measure::Ratio(_) => {
         if let Some(weight) = f.weight {
            weight
          } else {
            panic!("Unable to deduce flour weight {}", f);
          }
        }
      };
      acc + weight
    })
  }
}
