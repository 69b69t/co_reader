pub struct Arguments<'a> {
    pub center_x: i64,
    pub center_z: i64,
    pub radius: i64,
    pub resolution: u32,
    pub co_path: &'a str,
    pub image_path: &'a str,
}

pub fn create_cursor(
    connection: &sqlite::Connection,
    config: &Arguments
) -> impl Iterator<Item = (i64, i64)> {
    //returns a Cursor, but caller dosent need to know that.
    //they only need to know that its iteratable
    let x_min = config.center_x - config.radius;
    let x_max = config.center_x + config.radius - 1;
    let z_min = config.center_z - config.radius;
    let z_max = config.center_z + config.radius - 1;

    let query = "
    SELECT *
    FROM co_block
    WHERE x BETWEEN ? AND ?
    AND z BETWEEN ? AND ?;
    ";

    connection
        .prepare(query)
        .unwrap()
        .into_iter()
        .bind((1, x_min))
        .unwrap()
        .bind((2, x_max))
        .unwrap()
        .bind((3, z_min))
        .unwrap()
        .bind((4, z_max))
        .unwrap()
        .map(|row| {
            let row = row.unwrap();
            let x = row.read::<i64, &str>("x");
            let z = row.read::<i64, &str>("z");
            (x, z)
        })
}

pub fn plot(image: &mut image::RgbImage, positions: impl Iterator<Item = (i64, i64)>, config: &Arguments) {
    //modifies image by plotting data unto its buffer

    //fill image with white
    println!("filling in");
    for pixel in image.pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    //plot
    println!("plotting");
    let scale = config.resolution as f64 / (2 * config.radius) as f64;

    for position in positions {
        //scaling and writing pixel unto buffer
        let x = position.0 + config.radius - config.center_x;
        let x = (x as f64 * scale) as u32;

        let z = position.1 + config.radius - config.center_z;
        let z = (z as f64 * scale) as u32;

        image.put_pixel(x, z, image::Rgb([0, 0, 0]));

    }
}

fn help() {
    println!("usage:");
    println!("./<executable> <center_x> <center_z> <radius> <resolution> <co_path> <image_path>");
    println!("<center_x> <center_z>: self explanatory");
    println!("<radius>: distance in blocks between center of image and edge");
    println!("<resolution>: how wide the image will be");
    println!("<co_path>: path of the coreprotect database");
    println!("<image_path>: path to write image to");

}

impl<'a> Arguments<'a> {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        //Err if theres not enough arguments or too many
        if args.len() != 7 {
            help();
            println!("{} args which is wrong", args.len());
            return Err("wrong number of arguments");
        }

        //takes a slice of String and returns an owned arguments struct
        //currently panics instead of returning an error cause i am stupid or something blah blah
        let center_x: i64 = args[1].parse().expect("center_x not an i64");
        let center_z: i64 = args[2].parse().expect("center_z not an i64");
        let radius: i64 = args[3].parse().expect("radius not an i64");
        let resolution: u32 = args[4].parse().expect("resolution not a u32");
        let co_path = &args[5];
        let image_path = &args[6];
    
    
        //Err("nothing written yet")
        Ok(
            Arguments {
                center_x,
                center_z,
                radius,
                resolution,
                co_path,
                image_path,
            }
        )

    }
}