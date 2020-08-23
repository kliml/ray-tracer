use std::io;

mod color;
mod vec;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Renderer
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color = vec::Color::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.25,
            );
            color::write_color(&mut handle, pixel_color);
            // let r = i as f32 / (image_width - 1) as f32;
            // let g = j as f32 / (image_height - 1) as f32;
            // let b = 0.25;

            // let ir = (255.99 * r) as i32;
            // let ig = (255.99 * g) as i32;
            // let ib = (255.99 * b) as i32;

            // print!("{} {} {}\n", ir, ig, ib);
        }
    }

    eprint!("\nDone.\n");
}
