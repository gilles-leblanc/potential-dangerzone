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

  let drops = map_generator::create_drop_points(number_of_drop_points,
                                                min_particles,
                                                max_particles);

  for i in drops.iter() {
    println!("{}", i);
  }
}
