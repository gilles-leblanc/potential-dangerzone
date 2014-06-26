use std::io::File;
use map_generator::HeightMap;
use coordinate_formula::CoordinateFormula;

mod map_generator;
mod coordinate_formula;

static FILENAME: &'static str = "height_map.out";

fn main() {
  let size = 50;
  let number_of_drop_points = 20;
  let min_particles = 400;
  let max_particles = 2000;
  let number_of_passes = 5;

  let mut coordinate_formula = CoordinateFormula::new();
  let mut height_map: HeightMap = HeightMap::new(size);

  let drops: Vec<Box<Vec<int>>>;
  drops = Vec::from_fn(number_of_passes,
                       |_| map_generator::create_drop_points(number_of_drop_points *
                                                             number_of_passes / 2,
                                                             min_particles,
                                                             max_particles));


  for drop_point in drops.iter() {
    let (x, y) = coordinate_formula.calculate_coordinates(size);
    map_generator::drop_particles(drop_point.as_slice(), (x, y), &mut height_map);
    coordinate_formula = coordinate_formula.change_iteration();
  }

  // output height map in YAML format
  let path = Path::new(FILENAME);
  let mut file = match File::create(&path) {
    Err(why) => fail!("couldn't create {}: {}", FILENAME, why.desc),
    Ok(file) => file,
  };

  file.write_line("---");
  for n in range(0u, size * size) {
    file.write_line(format!("- {}", *height_map.map.get_mut(n)).as_slice());
  }
}
