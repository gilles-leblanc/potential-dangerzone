// File with main method. Can be used to start the generator without
// including it as a library from an external program.

mod map_generator;

fn main() {
  let mut height_map: Vec<int> = Vec::from_fn(1600, |_| 0);

  let number_of_drop_points = 5;
  let min_particles = 100;
  let max_particles = 400;
  let number_of_passes = 4;
  let particle_stability_radius = 1;

  let drops = create_drop_points(number_of_drop_points);

  for i in drops.iter() {
    println!("{}", i);
  }

  // for i in height_map.iter() {
  //   println!("t {}", i);
  // }
}

fn create_drop_points(number_of_drop_points: uint) -> Box<Vec<int>> {
  let mut drops: Vec<int> = Vec::with_capacity(number_of_drop_points);

  for _ in range(1, number_of_drop_points + 1) {
    drops.push(10);
  }

  box drops
}
