fn main() {
    let height = 1000;
    let width = 1000;
    let mut img = bmp::Image::new(width, height);

    // Face center
    let center_x = width / 2;
    let center_y = height / 2;

    // Face radius
    let face_radius = 300;

    for y in 0..height {
        for x in 0..width {
            let dx = (x as i32) - (center_x as i32);
            let dy = (y as i32) - (center_y as i32);
            let distance = ((dx * dx + dy * dy) as f64).sqrt();

            if distance >= (face_radius - 5) as f64 && distance <= (face_radius + 5) as f64 {
                img.set_pixel(x, y, bmp::consts::WHITE);
            }

            let left_eye_x = center_x - 100;
            let left_eye_y = center_y - 80;
            let eye_dx = (x as i32) - (left_eye_x as i32);
            let eye_dy = (y as i32) - (left_eye_y as i32);
            let eye_distance = ((eye_dx * eye_dx + eye_dy * eye_dy) as f64).sqrt();
            if eye_distance <= 30.0 {
                img.set_pixel(x, y, bmp::consts::WHITE);
            }

            let right_eye_x = center_x + 100;
            let right_eye_y = center_y - 80;
            let eye_dx = (x as i32) - (right_eye_x as i32);
            let eye_dy = (y as i32) - (right_eye_y as i32);
            let eye_distance = ((eye_dx * eye_dx + eye_dy * eye_dy) as f64).sqrt();
            if eye_distance <= 30.0 {
                img.set_pixel(x, y, bmp::consts::WHITE);
            }

            let smile_center_y = center_y + 50;
            let smile_radius = 150;
            let smile_dx = (x as i32) - (center_x as i32);
            let smile_dy = (y as i32) - (smile_center_y as i32);
            let smile_distance = ((smile_dx * smile_dx + smile_dy * smile_dy) as f64).sqrt();

            if smile_distance >= (smile_radius - 5) as f64
                && smile_distance <= (smile_radius + 5) as f64
                && y >= smile_center_y
            {
                img.set_pixel(x, y, bmp::consts::WHITE);
            }
        }
    }

    let _ = img
        .save("image.bmp")
        .unwrap_or_else(|e| panic!("Failed to save: {}", e));
}
