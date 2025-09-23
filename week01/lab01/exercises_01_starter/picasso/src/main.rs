use std::env;

fn main() {
    for argument in env::args().skip(1) {
        println!("===== {} =====", argument);
        match bmp::open(argument) {
            Ok(image) => {
                let width = image.get_width();
                let height = image.get_height();

                for y in 0..height {
                    for x in 0..width {
                        let pixel_colour = image.get_pixel(x, y);

                        if pixel_colour == bmp::consts::WHITE {
                            print!("W ");
                        } else if pixel_colour == bmp::consts::RED {
                            print!("R ");
                        } else if pixel_colour == bmp::consts::LIME {
                            print!("G ");
                        } else if pixel_colour == bmp::consts::BLUE {
                            print!("B ");
                        }
                    }
                    println!();
                }
            }
            Err(error) => {
                println!("Error! {:?}", error);
            }
        }
    }
}
