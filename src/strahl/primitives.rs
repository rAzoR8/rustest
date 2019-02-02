use super::hit::*;
use super::vec::*;
use super::ray::*;

pub struct Sphere
{
    pos: Vec4,
    radius: f32
}

impl Sphere
{
    pub fn new(_pos: Vec4, _radius: f32) -> Sphere
    {
        Sphere{pos: _pos, radius: _radius}
    }
}

impl Hitable for Sphere
{
    fn hit(&self, r: &Ray, out: &mut HitInfo, min: f32, max: f32) -> bool
    {
        let oc = self.pos - r.origin;
        true
    }
}