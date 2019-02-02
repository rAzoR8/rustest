use super::vec::*;

pub struct Ray {
    pub origin: Vec4,
    pub direction: Vec4
}

impl Ray {
    pub fn new(_origin: Vec4, _dir: Vec4) -> Ray {
        Ray {
            origin: _origin,
            direction: _dir
        }
    }

    pub fn point_at(&self, dist: f32) -> Vec4
    {
        self.origin + self.direction * dist
    }
}