use super::vec::*;
use rand::prelude::*;

pub fn random_tuple(min: f32, max: f32) -> (f32, f32)
{
    let mut rng = rand::thread_rng();
    (rng.gen_range(min, max), rng.gen_range(min, max))
}

pub fn random_in_unit_disk() -> Vec4
{
    let mut rng = rand::thread_rng();
    Vec4::from3(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 0.0).norm()
}

pub fn random_in_unit_disk_tuple() -> (f32, f32)
{
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0, 1.0);
    let y = rng.gen_range(0.0, 1.0);
    let len = (x*x + y*y as f32).sqrt();    

    (x / len, y / len)
}

// http://mathworld.wolfram.com/DiskPointPicking.html
pub fn random_in_unit_disk2() -> (f32, f32)
{
    let mut rng = rand::thread_rng();
    let phi = rng.gen_range(0.0, 2.0 * std::f32::consts::PI) as f32;
    let r = (rng.gen_range(0.0, 1.0) as f32).sqrt();
    (r * phi.cos(), r * phi.sin())
}

pub fn random_in_unit_sphere() -> Vec4
{
    let mut rng = rand::thread_rng();

    // method 2
    Vec4::from3(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)).norm()
}

pub fn random_in_unit_sphere2() -> Vec4
{
    let mut rng = rand::thread_rng();

    //https://hbfs.wordpress.com/2010/10/12/random-points-on-a-sphere-generating-random-sequences-iii/
    //https://corysimon.github.io/articles/uniformdistn-on-sphere/
    let theta = (rng.gen_range(-1.0, 1.0) as f32).asin();
    let cos_theta = theta.cos();

    let phi = rng.gen_range(0.0, 2.0 * std::f32::consts::PI) as f32;
    let x = cos_theta * phi.cos();
    let y = phi.sin();
    let z = cos_theta * y;

    Vec4::from3(x, y, z)
}