use std::rand::Rng;
use std::rand;

pub fn create_drop_points(number_of_drop_points: uint, min_particles: int, max_particles: int) -> Box<Vec<int>> {
  let mut drops: Vec<int> = Vec::with_capacity(number_of_drop_points);
  let mut rng = rand::task_rng();

  for _ in range(1, number_of_drop_points + 1) {
    drops.push(rng.gen_range(min_particles, max_particles));
  }

  box drops
}
