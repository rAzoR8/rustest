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
use crate::strahl::tonemap::*;
use crate::strahl::texture::*;
use crate::strahl::as3dcamera::*;

use image::{GenericImageView, ImageBuffer, imageops};
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
const MULTTHREADING: bool = true;

pub struct RayInfo
{
    pub depth: u32,
    pub ray: Ray,
    pub mat_info: [MaterialInfo; MAX_DEPTH]
}

type ScanLine = std::vec::Vec<Vec4>;
type TraceOutput = std::vec::Vec<ScanLine>;

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

        let mut col = self.mat_info[(self.depth - 1) as usize].emission;

        for i in 2..self.depth+1
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
        if !normal
        {            
            let scattered = scn.get_mat(hit.material).scatter(&mut r.ray, &hit, &mut mat_info);
            r.add_mat(&mat_info);
            return !scattered;   
        }
        else
        {
            mat_info.attenuation = (hit.normal + 1.0) * 0.5; 
            r.add_mat(&mat_info);
            return true; // terminated
        }
    }
    else // missed / escaped scene
    {
        scn.get_miss_mat().scatter(&mut r.ray, &hit, &mut mat_info);
        r.add_mat(&mat_info);
        return true;  // terminated
    }
}

#[inline]
pub fn color(scn: &Scene, cam: &Camera, x: u32, y: u32, ray_info: &mut RayInfo, ray_count: &mut u32) -> Vec4
{
    let mut rnd_ray = |channel: Channel| -> Vec4{
        let mut col = Vec4::zero();

        for _ in 0..cam.sample_count() {
            let (s, t) = random_in_unit_disk2();
            let u = x as f32 + s * 0.5;
            let v = y as f32 + t * 0.5;
            ray_info.reset(&cam.get_ray(u, v, channel));

            for _ in 0..MAX_DEPTH {
                if trace(ray_info, &scn, false) {
                    break;
                }
            }

            col += ray_info.accumulate();
            *ray_count += ray_info.depth;
        }

        col / cam.sample_count() as f32
    };

    if cam.mode() == Mode::Combined
    {
        return rnd_ray(Channel::All);
    }
    else
    {
        return Vec4::from3(rnd_ray(Channel::R).r(), rnd_ray(Channel::G).g(), rnd_ray(Channel::B).b());
    }
}

pub fn trace_image(cam: &Camera, scn: &Scene, print_progress: bool) -> TraceOutput
{
    let ray_count = AtomicU32::new(0);
    let line_count = AtomicU32::new(0);

    let trace_scan_line = |y: u32| -> ScanLine
    {
        let mut scan_line = ScanLine::with_capacity(cam.width() as usize);

        let scan_time = SystemTime::now();
        let mut ray = RayInfo::new();        

        let mut local_ray_count = 0;
        for x in 0..cam.width()
        {
            scan_line.push(color(&scn, cam, x, y, &mut ray, &mut local_ray_count));        
        }

        let cur_ray_count = ray_count.fetch_add(local_ray_count, Ordering::SeqCst);

        if print_progress
        {
            let cur_line_count = line_count.fetch_add(1, Ordering::SeqCst);
            let duration = scan_time.elapsed().unwrap().as_micros();
            let speed = local_ray_count as f64 / duration as f64;
            let percent = (cur_line_count * 100) as f32 / cam.height() as f32;
            print!("Y {} Progress {} \t Rays {} {} MRay/s \n", y, percent, cur_ray_count, speed as f32);
        }

        scan_line
    };

    //// TRACING ////
    let total_time = SystemTime::now();

    let mut scanlines = TraceOutput::with_capacity(cam.height() as usize);
    if MULTTHREADING && debug_divisior() == 1
    {
        let par_iter = (0..cam.height()).into_par_iter().map(|y| trace_scan_line(y));
        scanlines = par_iter.collect();
    }
    else
    {
        for y in 0..cam.height()
        {
            scanlines.push(trace_scan_line(y));
        }
    }

    let elapsed = total_time.elapsed().unwrap();
    //// TRACING ////

    let duration = elapsed.as_micros() as f64;
    let seconds = elapsed.as_float_secs();
    let speed = ray_count.into_inner() as f64 / duration;

    println!("Avg {} MRay/s {} Seconds", speed as f32, seconds);

    scanlines
}

fn main() {

    println!("loading scene...");

    let mut world = Scene::new();
    //world.set_envmap("Ocean.jpg", Vec4::from(2.0), DynamicTextureType::sRGB);

    let earth = world.add_mat(Lambertian::from_path("earth.jpg", DynamicTextureType::sRGB));

    let lamb1 = world.add_mat(Lambertian::new(0.8, 0.3, 0.3));
    let lamb2 = world.add_mat(Lambertian::new(0.1, 0.1, 0.0));

    let em_bright = world.add_mat(Emissive::new(100.0, 100.0, 100.0));
    let em_white = world.add_mat(Emissive::new(1.0, 1.0, 1.0));
    let metal1 = world.add_mat(Metal::new(0.9, 0.5, 0.5, 0.0)); // red-ish
    let metal_mirror = world.add_mat(Metal::new(1.0, 1.0, 1.0, 0.0));
    let metal_rough = world.add_mat(Metal::new(1.0, 1.0, 1.0, 2.3));

    let sphere1 = Sphere::new_with_uv(Vec4::from3(0.0, 0.0, -1.0), 0.5).object(earth);
    let sphere2 = Sphere::new(Vec4::from3(0.0, -100.5, -1.0), 100.0).object(metal_rough);
    let sphere3 = Sphere::new(Vec4::from3(-1.5, 0.5, -0.5), 0.4).object(metal1);
    let sphere4 = Sphere::new(Vec4::from3(-1.0, 0.0, -0.5), 0.1).object(lamb2); // right one
    let sphere5 = Sphere::new(Vec4::from3(-1.0, 0.0, -1.0), 0.3).object(lamb1);

    let bbox = BBox::new(Vec4::from3(0.0, 0.2, -0.5), Vec4::one()).object(lamb1);

    //world.add_prmitive(bbox);
    world.add_prmitive(sphere1);
    world.add_prmitive(sphere2);
    world.add_prmitive(sphere3);
    world.add_prmitive(sphere4);
    world.add_prmitive(sphere5);

    println!("tracing...");

    let debug = debug_divisior();

    let width = 1600 / debug;
    let height = 900 / debug;
    let samples = 100;

    let origin = Vec4::from3(0.0, 1.0, 1.0);
    let target = Vec4::from3(0.0, 0.0, -1.0);
    let up = Vec4::from3(0.0, 1.0, 0.0);

    let cam = PerspectiveCamera::new(origin, target, up, 60.0, width, height, 0.0, 100.0, samples);
    //let cam = AS3DCamera::new(origin, target, up, 60.0, width, height, 100.0, samples, 0.1, 8, 2.0, 3.0);

    let scanlines = trace_image(&cam, &world, false);

    let mut imgbuf = image::ImageBuffer::new(cam.width(), cam.height());

    let tone_operator = ReinhardTonemap::new(2.2, 1.0);

    let quantize = |color: &Vec4| -> image::Rgb<u8>
    {
        let final_color = tone_operator.tonemap(color).clamp_scalar(0.0, 1.0) * 255.99;

        let r = final_color.r() as u8;
        let g = final_color.g() as u8;
        let b = final_color.b() as u8;

        image::Rgb([r, g, b])
    };

    println!("tonemapping...");

    for (y, scanline) in scanlines.iter().enumerate() {
        for (x, pixel) in scanline.iter().enumerate() {
            imgbuf.put_pixel(x as u32, y as u32, quantize(pixel)); 
        }
    }

    println!("saving...");

    imgbuf.save("output.png").unwrap();
}
