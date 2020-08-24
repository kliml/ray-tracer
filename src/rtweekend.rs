// Constants

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

// Utility Functions

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
