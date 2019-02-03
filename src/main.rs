
pub mod strahl;
use crate::strahl::hit::*;
use crate::strahl::primitives::*;
use crate::strahl::scene::*;
use crate::strahl::camera::*;
use crate::strahl::ray::*;
use crate::strahl::vec::*;
use crate::strahl::material::*;

extern crate image;

use image::{ImageBuffer, imageops};

pub struct RayInfo
{
    pub ray: Ray,
    pub mat_info: [MaterialInfo; 10],
    pub depth: u32
}

impl RayInfo
{
    pub fn new(_ray: Ray) -> RayInfo
    {
        RayInfo{ray: _ray, mat_info: [MaterialInfo::new(); 10], depth: 0}
    }

    pub fn add_mat(&mut self, mat: &MaterialInfo)
    {
        if self.depth < 10
        {
            self.mat_info[self.depth as usize] = *mat;
            self.depth += 1;
        }
    }

    pub fn accumulate(&self) -> Vec4
    {
        if self.depth == 0 { return Vec4::zero();}

        let mut col = (self.mat_info[(self.depth - 1) as usize]).emission;

        for i in 1..self.depth 
        {
            let cur = self.mat_info[(self.depth - i) as usize];
            col *= (cur.emission + cur.attenuation);
        }

        col
    }
}

// return true if terminated
fn trace(r: &mut RayInfo, scn: &Scene, normal: bool) -> bool
{
    let mut hit = HitInfo::new();
    let mut mat_info = MaterialInfo::new();

    if scn.hit(&r.ray, &mut hit, 0.0, 100.0)
    {
        if normal
        {            
            mat_info.attenuation = (hit.normal + 1.0) * 0.5; 
            r.add_mat(&mat_info);

            return true; // terminated
        }
        else
        {
            let hit_mat = match scn.get_mat(hit.material)
            {
                Material::Lambertian {mat} => {mat}
            };

            let mut scattered_ray = Ray::invalid();

            let scattered = hit_mat.scatter(&r.ray, &hit, &mut mat_info, &mut scattered_ray);

            if scattered
            {
                r.ray = scattered_ray;
            }

            r.add_mat(&mat_info);

            return !scattered;            
        }
    }
    else // background
    {
        let t = 0.5 * (r.ray.direction.norm().y() + 1.0);
        mat_info.emission = Vec4::from(1.0-t) + t * Vec4::from3(0.5, 0.7, 1.0);
        r.add_mat(&mat_info);

        return true;
    }
}

fn main() {
    let mut world = Scene::new();

    let lamb = world.add_mat(Lambertian::new(0.5, 0.1, 0.0).material());

    let sphere1 = Sphere::new(Vec4::from3(0.0, 0.0, -1.0), 0.5).primitive(lamb);
    let sphere2 = Sphere::new(Vec4::from3(0.0, -100.5, -1.0), 100.0).primitive(lamb);

    //let plane = Plane::new(Vec4::from3(0.0, 0.0, -10.0), Vec4::from3(-0.5, 0.0, -1.0).norm()).primitive(0); 

    world.add(sphere1);
    world.add(sphere2);

    //world.add(plane);

    let width = 800;
    let height = 450;

    let origin = Vec4::zero();
    let target = Vec4::from3(0.0, 0.0, -1.0);
    let up = Vec4::from3(0.0, -1.0, 0.0);

    let cam = Camera::new(origin, target, up, 60.0, width as f32 / height as f32, 0.0, 100.0);

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f32 / width as f32;
        let v = y as f32 / height as f32;

        let mut ray = RayInfo::new(cam.get_ray(u, v));

        for depth in 0..10 {
            if trace(&mut ray, &world, false)
            {
                break;
            }
        }

        let col = ray.accumulate() * 255.99;

        let r = col.r() as u8;
        let g = col.g() as u8;
        let b = col.b() as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("test.png").unwrap();

    print!("done!");
}
