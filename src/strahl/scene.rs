use super::primitives::*;
use super::material::*;
use super::hit::*;
use super::ray::*;
use super::vec::*;
use std::vec::*;

pub struct Scene
{
    pub primitives: std::vec::Vec<Primitive>,
    pub materials: std::vec::Vec<Material>,
    pub miss: Material
}

impl Scene
{
    pub fn new() -> Scene
    {
        Scene{primitives: std::vec::Vec::new(), materials: std::vec::Vec::new(), miss: Background::new(Vec4::from3(0.3, 0.3, 0.8), 1.0)}
    }

    pub fn add(&mut self, obj: Primitive)
    {
        self.primitives.push(obj);
    }

    pub fn add_mat(&mut self, mat: Material) -> u32
    {
        self.materials.push(mat);
        (self.materials.len() - 1) as u32
    }

    pub fn get_mat(&self, mat: u32) -> &Material
    {
        &self.materials[(mat as usize)]
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

        for primitve in self.primitives.iter() {
            match primitve 
            {
                Primitive::Sphere{obj, mat} => { process_hit(obj, *mat); },
                Primitive::Plane{obj, mat} => { process_hit(obj, *mat); },
                _ => {}
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