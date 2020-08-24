use crate::clamp;
use crate::vec::Color;
use std::io::Write;

pub fn write_color<R: Write>(out: &mut R, pixel_color: Color, samples_per_pixel: i32) {
    let (mut r, mut g, mut b) = (pixel_color.x(), pixel_color.y(), pixel_color.z());

    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    let color_line = format!(
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32,
    );
    out.write(color_line.as_bytes()).unwrap();
}
