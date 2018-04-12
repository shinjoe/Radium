use std::f32;
use std::fmt;
use std::ops::{Add, AddAssign, Index, Mul, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    e: [f32; 3],
}

#[allow(dead_code)]
impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn from_float(arr: [f32; 3]) -> Vec3 {
        Vec3 {
            e: [arr[0], arr[1], arr[2]]
        }
    }

    pub fn from_int(arr: [i32; 3]) -> Vec3 {
        Vec3 {
            e: [arr[0] as f32, arr[1] as f32, arr[2] as f32]
        }
    }

    pub fn dot(left: Vec3, right: Vec3) -> f32 {
        left.e[0] * right.e[0] + left.e[1] * right.e[1] + left.e[2] * right.e[2]
    }

    pub fn cross(left: Vec3, right: Vec3) -> Vec3 {
        let a = left.e[0];
        let b = left.e[1];
        let c = left.e[2];
        let d = right.e[0];
        let e = right.e[1];
        let f = right.e[2];
        Vec3 {
            e: [b * f - c * e, c * d - a * f, a * e - b * d]
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.e[0],
            1 => &self.e[1],
            2 => &self.e[2],
            _ => panic!("Unexpected vector index: {}", i)
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, val: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] * val, self.e[1] * val, self.e[2] * val]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.e[0], self.e[1], self.e[2])
    }
}




