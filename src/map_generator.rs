use std::rand::Rng;
use std::rand;

pub struct HeightMap {
  pub map: Vec<int>,
  size: uint,
}

impl HeightMap {
  pub fn new(size: uint) -> HeightMap {
    HeightMap { map: Vec::from_fn(size * size, |_| 0), size: size }
  }
}

// Create a series of drop points that contain a number representing the number of particles per drop point
pub fn create_drop_points(number_of_drop_points: uint, min_particles: int,
                          max_particles: int) -> Box<Vec<int>> {
  let mut drops: Vec<int> = Vec::with_capacity(number_of_drop_points);
  let mut rng = rand::task_rng();
  let mut current_min = min_particles;
  let mut current_max = max_particles;

  for _ in range(1, number_of_drop_points + 1) {
    drops.push(rng.gen_range(current_min, current_max));
    current_min *= 1.1 as int;
    current_max *= 1.1 as int;
  }

  box drops
}

// Drops all particle from drops on the height map
pub fn drop_particles(drops: &[int], drop_coordinates: (uint, uint), height_map: &mut HeightMap) {
  for drop in drops.iter() {
    for _ in range(1, *drop) {
      let (x, y) = drop_coordinates;
      let target = *height_map.map.get_mut(x + y * height_map.size);

      if target == 0 {
        // Drop current particle directly at coordinates
        *height_map.map.get_mut(x + y * height_map.size) += 1;
      } else {
        // Simulate dropping particle at coordinates and then agitating as per particle deposition algo
        // for height map generation. Concretly, drop particle where agitating would have put it.
        agitate(drop_coordinates, target, height_map);
      }
    }
  }
}

// Look at a coordinate's neighbors and collect the ones with lowest value. Select one of them to receive
// particle.
fn agitate(coordinates: (uint, uint), target_height: int, height_map: &mut HeightMap) {
  let (x, y) = coordinates;
  // find all lowest neighbors
  for index_x in range(x - 2, x + 2) {
    for index_y in range(y - 2, y + 2) {
      if *height_map.map.get_mut(index_x + index_y * height_map.size) < target_height {
        *height_map.map.get_mut(index_x + index_y * height_map.size) += 1;
        return;
      }
    }
  }

  // no lower neighbor, leave particle on drop point
  *height_map.map.get_mut(x + y * height_map.size) += 1;
}
