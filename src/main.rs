#![feature(duration_as_u128)]
#![feature(duration_float)]
#![feature(integer_atomics)]

extern crate image;
extern crate rayon;

pub mod strahl;
use crate::strahl::material::*;
use crate::strahl::primitives::*;
use crate::strahl::scene::*;
use crate::strahl::camera::*;
use crate::strahl::vec::*;
use crate::strahl::hit::*;
use crate::strahl::ray::*;
use crate::strahl::random::*;

use image::{ImageBuffer, imageops};
use rayon::prelude::*;
use std::time::{Duration, SystemTime};
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(debug_assertions)]
fn debug_divisior() -> u32 {
    4
}

#[cfg(not(debug_assertions))]
fn debug_divisior() -> u32 {
    1
}

const MAX_DEPTH : usize = 10;

pub struct RayInfo
{
    pub depth: u32,
    pub ray: Ray,
    pub mat_info: [MaterialInfo; MAX_DEPTH]
}

type ScanLine = std::vec::Vec<image::Rgb<u8>>;

impl RayInfo
{
    #[inline]
    pub fn new() -> RayInfo
    {
        RayInfo{ray: Ray::invalid(), mat_info: [MaterialInfo::new(); MAX_DEPTH], depth: 0}
    }

    #[inline]
    pub fn add_mat(&mut self, mat: &MaterialInfo)
    {
        if self.depth < MAX_DEPTH as u32
        {
            self.mat_info[self.depth as usize] = *mat;
            self.depth += 1;
        }
    }

    #[inline]
    pub fn accumulate(&self) -> Vec4
    {
        if self.depth == 0 { return Vec4::zero();}

        let mut col = Vec4::zero();

        for i in 1..self.depth+1
        {
            let cur = self.mat_info[(self.depth - i) as usize];
            col *= cur.attenuation;
            col += cur.emission;
        }

        col
    }

    #[inline]
    pub fn reset(&mut self, ray: &Ray)
    {
        self.ray = *ray;
        self.depth = 0;
    }
}

// return true if terminated
#[inline]
pub fn trace(r: &mut RayInfo, scn: &Scene, normal: bool) -> bool
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
        mat_info.emission *= 0.5;
        mat_info.attenuation = Vec4::one();
        r.add_mat(&mat_info);

        return true;
    }
}

#[inline]
pub fn color(scn: &Scene, cam: &Camera, x: u32, y: u32, ray_info: &mut RayInfo, samples: u32, ray_count: &mut u32) -> image::Rgb<u8>
{
    let mut col = Vec4::zero();
            
    for _ in 0..samples {
        
        let (s, t) = random_in_unit_disk2();

        let u = (x as f32 + s * 0.5) / cam.width as f32;
        let v = (y as f32 + t * 0.5) / cam.height as f32;
        ray_info.reset(&cam.get_ray(u, v));

        for _ in 0..MAX_DEPTH {
            if trace(ray_info, &scn, false)
            {
                break;
            }
        }

        col += ray_info.accumulate();
        *ray_count += ray_info.depth;
    }

    col /= samples as f32;

    let final_color = col.sqrt();

    let r = (final_color.r() * 255.99) as u8;
    let g = (final_color.g() * 255.99) as u8;
    let b = (final_color.b() * 255.99) as u8;

    image::Rgb([r, g, b])
}

pub fn trace_image(cam: &Camera, scn: &Scene, samples: u32, print_progress: bool) -> image::RgbImage
{
    let ray_count = AtomicU32::new(0);
    let line_count = AtomicU32::new(0);

    let trace_scan_line = |y: u32| -> ScanLine
    {
        let mut scan_line = ScanLine::with_capacity(cam.width as usize);

        let scan_time = SystemTime::now();
        let mut ray = RayInfo::new();        

        let mut local_ray_count = 0;
        for x in 0..cam.width
        {
            scan_line.push(color(&scn, &cam, x, y, &mut ray, samples, &mut local_ray_count));        
        }

        let cur_ray_count = ray_count.fetch_add(local_ray_count, Ordering::SeqCst);

        if print_progress
        {
            let cur_line_count = line_count.fetch_add(1, Ordering::SeqCst);
            let duration = scan_time.elapsed().unwrap().as_micros();
            let speed = local_ray_count as f64 / duration as f64;
            let percent = (cur_line_count * 100) as f32 / cam.height as f32;
            print!("Y {} Progress {} \t Rays {} {} MRay/s \n", y, percent, cur_ray_count, speed as f32);
        }

        scan_line
    };

    //// PARALELL TRACING ////
    let total_time = SystemTime::now();

    let par_iter = (0..cam.height).into_par_iter().map(|y| trace_scan_line(y));
    let scanlines: std::vec::Vec<_> = par_iter.collect();

    let elapsed = total_time.elapsed().unwrap();
    //// PARALELL TRACING ////

    let duration = elapsed.as_micros() as f64;
    let seconds = elapsed.as_float_secs();
    let speed = ray_count.into_inner() as f64 / duration;

    print!("Avg {} MRay/s {} Seconds", speed as f32, seconds);

    let mut imgbuf = image::ImageBuffer::new(cam.width, cam.height);

    for (y, scanline) in scanlines.iter().enumerate() {
        for (x, pixel) in scanline.iter().enumerate() {
            imgbuf.put_pixel(x as u32, y as u32, *pixel); 
        }
    }

    imgbuf
}

fn main() {
    let mut world = Scene::new();

    let lamb1 = world.add_mat(Lambertian::new(0.8, 0.3, 0.3).material());
    let lamb2 = world.add_mat(Lambertian::new(0.1, 0.1, 0.0).material());

    let em1 = world.add_mat(Emissive::new(10.0, 10.0, 10.0).material());
    let em2 = world.add_mat(Emissive::new(1.0, 1.0, 1.0).material());
    let metal1 = world.add_mat(Metal::new(1.0, 0.0, 0.0, 0.0).material()); // red
    let metal2 = world.add_mat(Metal::new(1.0, 1.0, 1.0, 0.0).material());

    let sphere1 = Sphere::new(Vec4::from3(0.0, 0.0, -1.0), 0.5).primitive(lamb1);
    let sphere2 = Sphere::new(Vec4::from3(0.0, -100.5, -1.0), 100.0).primitive(lamb2);
    //let sphere3 = Sphere::new(Vec4::from3(-1.5, 0.0, -1.0), 0.5).primitive(em2);
    let sphere4 = Sphere::new(Vec4::from3(1.0, 0.0, -1.0), 0.3).primitive(em2); // right one
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
    let samples = 100;

    let origin = Vec4::from3(0.0, 1.0, 1.0);
    let target = Vec4::from3(0.0, 0.0, -1.0);
    let up = Vec4::from3(0.0, 1.0, 0.0);

    let cam = Camera::new(origin, target, up, 60.0, width, height, 0.0, 100.0);
    
    trace_image(&cam, &world, samples, false).save("test.png").unwrap();
}
