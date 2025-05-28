use std::env;

mod lib;

fn main() {
    //get args
    let args: Vec<String> = env::args().collect();

    //parse args
    let config = lib::Arguments::new(&args).expect("failed to parse config");

    //make connection from file path
    let connection = sqlite::open(config.co_path).unwrap();

    //make Iterator over set of positions
    let positions = lib::create_cursor(&connection, &config);

    //make an image buffer to write to
    let mut image = image::RgbImage::new(config.resolution, config.resolution);

    //plot 
    lib::plot(&mut image, positions, &config);

    //save
    image.save(config.image_path).unwrap();
}
