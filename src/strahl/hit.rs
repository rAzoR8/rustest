use super::ray::*;
use super::vec::*;

#[derive(Copy, Clone)]
pub struct HitInfo
{
    pub point: Vec4,
    pub normal: Vec4,
    pub depth: f32,
    pub u: f32,
    pub v: f32,
    pub material: u32 // supplied by primitive
}

impl HitInfo
{
    pub fn new() -> HitInfo
    {
        HitInfo{point: Vec4::from(0.0), normal: Vec4::from(0.0), depth: 0.0, material: 0, u: 0.0, v: 0.0}
    }
}

pub trait Hitable
{
    fn hit(&self, r: &Ray, out: &mut HitInfo, min: f32, max: f32) -> bool;
} 