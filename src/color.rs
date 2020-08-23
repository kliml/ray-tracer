use crate::vec::Color;
use std::io::Write;

pub fn write_color<R: Write>(out: &mut R, pixel_color: Color) {
    let color_line = format!(
        "{} {} {}\n",
        pixel_color.x() * 255.999,
        pixel_color.y() * 255.999,
        pixel_color.z() * 255.999
    );
    out.write(color_line.as_bytes()).unwrap();
}
