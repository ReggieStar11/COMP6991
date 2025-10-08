fn main() {
    
    let height = 1000;
    let width = 1000;
    let mut img = bmp::Image::new(width, height);

    for y in 0..height {
        for x in 0..width {
            if x == y/2 {
                (&mut img).set_pixel(x, y, bmp::consts::RED)
            }

            if x == 2*y {
                (&mut img).set_pixel(x, y, bmp::consts::RED)
            }

            if x == 3*y {
                (&mut img).set_pixel(x, y, bmp::consts::RED)
            }
            if x == y/4 {
                (&mut img).set_pixel(x, y, bmp::consts::RED)
            }
        }
    }

    let _ = img.save("image.bmp").unwrap_or_else(|e| {
    panic!("Failed to save: {}", e)
});

}
