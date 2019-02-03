use super::vec::*;

#[derive(Copy, Clone)]
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

    pub fn invalid() -> Ray {
        Ray {
            origin: Vec4::zero(),
            direction: Vec4::zero()
        }
    }

    pub fn point_at(&self, _dist: f32) -> Vec4
    {
        self.origin + self.direction * _dist
    }
}