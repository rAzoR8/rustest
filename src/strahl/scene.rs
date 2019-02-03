use super::primitives::*;
use super::hit::*;
use super::ray::*;
use std::vec::*;

pub struct Scene
{
    pub container: std::vec::Vec<Primitive>
}

impl Scene
{
    pub fn new() -> Scene
    {
        Scene{container: std::vec::Vec::new()}
    }

    pub fn add(&mut self, obj: Primitive)
    {
        self.container.push(obj);
    }
}

impl Hitable for Scene
{
    fn hit(&self, r: &Ray, out: &mut HitInfo, min: f32, max: f32) -> bool
    {
        let mut best_info = HitInfo::new();
        best_info.depth = max;

        let mut process_hit = |obj: &Hitable, mat: u32| 
        {
            let mut info = HitInfo::new();
            if obj.hit(r, &mut info, min, best_info.depth) //&& info.depth < best_info.depth
            {             
                best_info = info;
                best_info.material = mat;
            }
        };

        for primitve in self.container.iter() {
            match primitve 
            {
                Primitive::SphereT{obj, mat} => { process_hit(obj, *mat); },
                _ => {}
                //Primitive::PlaneT{obj, mat} => { process_hit(obj, mat); },
                //Primitive::AABBT{obj, mat} => { process_hit(obj, mat); }
            }
        }

        if best_info.depth > min && best_info.depth < max
        {
            *out = best_info;
            return true;
        }

        false
    }
}