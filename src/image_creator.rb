require 'gosu'
require 'texplay'
require 'yaml'

# Ruby helper class to output our heightmaps. Permits to test the Rust code.
class ImageCreator
  def initialize(filename)
    @data = YAML.load_file(filename)
    @size = 40
  end

  # test to output some yaml to check the format so we can replicate it in Rust
  def ouput_yaml
    array = Array.new(@size * @size, 0)
    File.open('test_yaml.out', 'w') { |file| file.puts YAML.dump(array) }
  end

  # create the image file
  def create
    window = Gosu::Window.new(@size, @size, false)
    image = TexPlay.create_image(window, @size, @size, color: Gosu::Color::BLACK)
    image.draw 0, 0, 0

    for y in 0...@size
      for x in 0...@size
        grayscale = (@data[x + y * @size] * 4.0) / 1000.0
        image.pixel x, y, color: [grayscale, grayscale, grayscale]
      end
    end

    image.save('height_map.png')
  end
end

image_creator = ImageCreator.new('height_map.out')
image_creator.create
puts 'done'
