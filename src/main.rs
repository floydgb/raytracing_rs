use std::{fs::File, io::Write};
use indicatif::ProgressBar;
use vec3::Vec3;

mod vec3;
mod color;

fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    // Render
    match File::create("image.ppm") {
        Ok(mut buffer) => {
            write!(&mut buffer, "P3\n{} {}\n255\n", image_width, image_height).expect("Can't write");

            let pb = ProgressBar::new(image_height as u64);

            for  j in 0..image_height  {
                pb.inc(1);
                for i in 0..image_width {

                    let pixel_color: Vec3 = Vec3::new(
                        i as f64 / (image_width as f64 - 1.0),
                        j as f64 / (image_height as f64 - 1.0),
                        0.0,
                    );

                    write!(&mut buffer, "{}", color::write_color(&pixel_color) ).expect("error writing the colors");

                }
            }
            pb.finish_with_message("Done!");

        }
        Err(e) => {
            println!("Could not open file...")
        }
    }
}
