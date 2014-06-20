use map_generator::HeightMap;
mod map_generator;

fn main() {
  let size = 40;
  let number_of_drop_points = 5;
  let min_particles = 100;
  let max_particles = 400;
  let number_of_passes = 4;
  let particle_stability_radius = 1;

  let mut height_map: HeightMap = HeightMap::new(size);

  let drops: Vec<Box<Vec<int>>>;
  drops = Vec::from_fn(number_of_passes,
                       |_| map_generator::create_drop_points(number_of_drop_points *
                                                             number_of_passes / 2,
                                                             min_particles,
                                                             max_particles));

  for drop_point in drops.iter() {
    map_generator::drop_particles(drop_point.as_slice(), &mut height_map);
  }
}
