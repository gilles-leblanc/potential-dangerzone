use std::rand::Rng;
use std::rand;

pub enum CoordinateFormula {
  FirstIteration,
  SecondIteration,
  SubsequentIteration,
}

impl CoordinateFormula {
  pub fn new() -> CoordinateFormula {
    FirstIteration
  }

  pub fn change_iteration(&mut self) -> CoordinateFormula {
    match *self {
      FirstIteration => SecondIteration,
      SecondIteration => SubsequentIteration,
      SubsequentIteration => SubsequentIteration,
    }
  }

  // Calculate the x and y coordinates for a drop point.
  pub fn calculate_coordinates(&mut self, map_size: uint) -> (uint, uint) {
    let mut rng = rand::task_rng();

    match *self {
      FirstIteration => (map_size / 2, map_size / 2),
      SecondIteration => (rng.gen_range(4, map_size - 4),
                          rng.gen_range(4, map_size - 4)),
      SubsequentIteration => (rng.gen_range(0, map_size / 2) + map_size / 4,
                              rng.gen_range(0, map_size / 2) + map_size / 4),
    }
  }
}
