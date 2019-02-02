use super::ray::*;
use super::vec::*;

pub struct HitInfo
{
    pub point: Vec4,
    pub normal: Vec4,
    pub depth: f32
}

impl HitInfo
{
    pub fn new() -> HitInfo
    {
        HitInfo{point: Vec4::from(0.0), normal: Vec4::from(0.0), depth: 0.0}
    }

}

pub trait Hitable
{
    fn hit(&self, r: &Ray, out: &mut HitInfo, min: f32, max: f32) -> bool;
} 