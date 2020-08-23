use std::fmt::{self, Formatter};
use std::ops::{Add, Div, Index, Mul, Sub};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Copy, Clone)]
pub struct Vec3([f32; 3]);

impl Vec3 {
    pub fn empty() -> Self {
        Vec3([0.0; 3])
    }

    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3([e0, e1, e2])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn length(&self) -> f32 {
        self.0[0] + self.0[1] + self.0[2]
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Vec3) -> Self {
        Vec3([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> Self {
        Vec3([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        ])
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Vec3) -> Self {
        Vec3([
            self.0[0] * other.0[0],
            self.0[1] * other.0[1],
            self.0[2] * other.0[2],
        ])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Vec3([self.0[0] * other, self.0[1] * other, self.0[2] * other])
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, t: f32) -> Self {
        self * (1.0 / t)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.0[0] * v.0[0] + u.0[1] * v.0[1] + u.0[2] * v.0[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3([
        u.0[1] * v.0[2] - u.0[2] * v.0[1],
        u.0[2] * v.0[0] - u.0[0] * v.0[2],
        u.0[0] * v.0[1] - u.0[1] * v.0[0],
    ])
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
