use std::ops::Add;

#[derive(Debug)]
pub struct Vec3 {
    e: [f32; 3],
}

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
