use std::rand::Rng;
use std::rand;

pub struct HeightMap {
  map: Vec<int>,
  size: uint,
}

impl HeightMap {
  pub fn new(size: uint) -> HeightMap {
    HeightMap { map: Vec::from_fn(size * size, |_| 0), size: size }
  }
}

pub fn create_drop_points(number_of_drop_points: uint, min_particles: int, max_particles: int) -> Box<Vec<int>> {
  let mut drops: Vec<int> = Vec::with_capacity(number_of_drop_points);
  let mut rng = rand::task_rng();

  for _ in range(1, number_of_drop_points + 1) {
    drops.push(rng.gen_range(min_particles, max_particles));
  }

  box drops
}

pub fn drop_particles(drops: &[int], height_map: &HeightMap) {
  for drop in drops.iter() {
    let x = height_map.size / 2;
    let y = height_map.size / 2;

    for _ in range(1, *drop) {
      // drop a particle
      let HeightMap { map: my_map, size: _ } = height_map;
      my_map[x + y * height_map.size] = 1;
      // move drop point

    }
  }
}
