use super::primitives::*;
use super::material::*;
use super::hit::*;
use super::ray::*;
use super::vec::*;
use super::texture::DynamicTextureType;
//use std::vec::*;

pub struct Scene
{
    objects: std::vec::Vec<Object>,
    materials: std::vec::Vec<Material>,
    miss: u32
}

impl Scene {
    pub fn new() -> Scene {
        let mut scn = Scene {
            objects: std::vec::Vec::new(),
            materials: std::vec::Vec::new(),
            miss: 0,
        };

        scn.miss = scn.add_mat(Background::new(Vec4::from3(0.5, 0.7, 1.0), 1.0));
        scn
    }

    pub fn set_miss_mat(&mut self, mat: u32)
    {
        self.miss = mat;
    }

    pub fn get_miss_mat(&self) -> &Material
    {
        self.get_mat(self.miss)
    }

    pub fn add_prmitive(&mut self, obj: Object) -> u32
    {
        self.objects.push(obj);
        (self.objects.len() - 1) as u32
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

    pub fn get_mat_mut(&mut self, mat: u32) -> &mut Material
    {
        &mut self.materials[(mat as usize)]
    }

    // returns prmitive & material id
    pub fn set_envmap<P>(&mut self, path: P, _strength: Vec4, _type: DynamicTextureType) -> (u32, u32)
    where P: AsRef<std::path::Path>,
    {
        let mat = self.add_mat(Emissive::from_path(path, _strength, _type));
        (self.add_prmitive(Sphere::new_with_uv(Vec4::zero(), 100.0).object(mat)), mat)
    }
}

impl Hitable for Scene {
    fn hit(&self, r: &Ray, out: &mut HitInfo, min: f32, max: f32) -> bool {
        let mut best_info = HitInfo::new();
        best_info.depth = max;

        let mut info = HitInfo::new();
        for obj in self.objects.iter() {
            if obj.hit(r, &mut info, min, best_info.depth)
            {
                best_info = info;
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
