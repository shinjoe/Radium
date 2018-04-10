use vector::Vec3;

struct Ray {
    origin: Vec3,
    direction : Vec3
}

#[allow(dead_code)]
impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin: orig,
            direction: dir
        }
    }

    pub fn get_origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vec3 {
        &self.direction
    }
}