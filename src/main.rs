
pub mod strahl;
use crate::strahl::hit::*;
use crate::strahl::primitives::*;
use crate::strahl::scene::*;
use crate::strahl::camera::*;
use crate::strahl::ray::*;
use crate::strahl::vec::*;
use crate::strahl::material::*;
use crate::strahl::random;

extern crate image;

use image::{ImageBuffer, imageops};

pub struct RayInfo
{
    pub depth: u32,
    pub ray: Ray,
    pub mat_info: [MaterialInfo; 10]
}

impl RayInfo
{
    pub fn new() -> RayInfo
    {
        RayInfo{ray: Ray::invalid(), mat_info: [MaterialInfo::new(); 10], depth: 0}
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

        for i in 2..self.depth+1
        {
            let cur = self.mat_info[(self.depth - i) as usize];
            col *= cur.attenuation;
            col += cur.emission;
        }

        col
    }

    pub fn reset(&mut self, ray: &Ray)
    {
        self.ray = *ray;
        self.depth = 0;
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
            let mut scattered_ray = Ray::invalid();

            let scattered = match scn.get_mat(hit.material)
            {
                Material::Lambertian {mat} => {mat.scatter(&r.ray, &hit, &mut mat_info, &mut scattered_ray)},
                Material::Emissive {mat} => {mat.scatter(&r.ray, &hit, &mut mat_info, &mut scattered_ray)},
                Material::Metal {mat} => {mat.scatter(&r.ray, &hit, &mut mat_info, &mut scattered_ray)}
            };

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
        mat_info.attenuation = Vec4::one();
        r.add_mat(&mat_info);

        return true;
    }
}

#[cfg(debug_assertions)]
fn debug_divisior() -> u32 {
    4
}

#[cfg(not(debug_assertions))]
fn debug_divisior() -> u32 {
    1
}

fn main() {
    let mut world = Scene::new();

    let lamb1 = world.add_mat(Lambertian::new(0.8, 0.3, 0.3).material());
    let lamb2 = world.add_mat(Lambertian::new(0.8, 0.8, 0.0).material());

    let em1 = world.add_mat(Emissive::new(10.0, 10.0, 10.0).material());
    let em2 = world.add_mat(Emissive::new(10.0, 10.0, 100.0).material());
    let metal1 = world.add_mat(Metal::new(0.8, 0.6, 0.2, 0.0).material());
    let metal2 = world.add_mat(Metal::new(0.8, 0.8, 1.8, 0.0).material());

    let sphere1 = Sphere::new(Vec4::from3(0.0, 0.0, -1.0), 0.5).primitive(lamb1);
    let sphere2 = Sphere::new(Vec4::from3(0.0, -100.5, -1.0), 100.0).primitive(lamb2);
    //let sphere3 = Sphere::new(Vec4::from3(-1.5, 0.0, -1.0), 0.5).primitive(em2);
    let sphere4 = Sphere::new(Vec4::from3(1.0, 0.0, -1.0), 0.3).primitive(metal1);
    let sphere5 = Sphere::new(Vec4::from3(-1.0, 0.0, -1.0), 0.3).primitive(metal2);

    //let plane = Plane::new(Vec4::from3(0.0, 0.0, -10.0), Vec4::from3(-0.5, 0.0, -1.0).norm()).primitive(0); 

    world.add(sphere1);
    world.add(sphere2);
    //world.add(sphere3);
    world.add(sphere4);
    world.add(sphere5);

    //world.add(plane);

    let debug = debug_divisior();

    let width = 800 / debug;
    let height = 450 / debug;
    let sampels = 100;

    let origin = Vec4::from3(0.0, 0.0, 1.0);
    let target = Vec4::from3(0.0, 0.0, -1.0);
    let up = Vec4::from3(0.0, -1.0, 0.0);

    let cam = Camera::new(origin, target, up, 60.0, width as f32 / height as f32, 0.0, 100.0);

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let mut last_y = 0;
    let mut ray_count = 0;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        let mut ray = RayInfo::new();
        let mut col = Vec4::zero();

        for _ in 0..sampels {
            
            let (s, t) = random::random_tuple(-1.0, 1.0);
            //let (s, t) = (0.0, 0.0);

            let u = (x as f32 + s) / width as f32;
            let v = (y as f32 + t) / height as f32;
            ray.reset(&cam.get_ray(u, v));

            for depth in 0..10 {
                if trace(&mut ray, &world, false)
                {
                    break;
                }
            }

            col += ray.accumulate();
            ray_count += ray.depth;
        }

        col /= sampels as f32;

        let final_color = col * 255.99;

        // todo: gamma corret

        let r = final_color.r() as u8;
        let g = final_color.g() as u8;
        let b = final_color.b() as u8;
        *pixel = image::Rgb([r, g, b]);

        if y != last_y
        {
            last_y = y;
            let percent = (y * 100) as f32 / height as f32;
            print!("Progress {} \t Rays {} \n", percent, ray_count);
        }
    }

    imgbuf.save("test.png").unwrap();

    print!("done!");
}
