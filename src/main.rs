
//#![feature(type_alias_enum_variants)]

pub mod strahl;
use crate::strahl::hit::Hitable;

extern crate image;

use image::{ImageBuffer, imageops};

type Vec4 = strahl::vec::Vec4;
type Ray = strahl::ray::Ray;
type Sphere = strahl::primitives::Sphere;
type HitInfo = strahl::hit::HitInfo;
type Camera = strahl::camera::Camera;
type Scene = strahl::scene::Scene;
//type Primitive = strahl::primitives::Primitive;

fn color(ray: &Ray, scn: &Scene) -> Vec4
{
    let mut hit = HitInfo::new();

    if scn.hit(&ray, &mut hit, 0.0, 100.0)
    {
        return (hit.normal + 1.0) * 0.5;
    }
    else // background
    {
        let t = 0.5 * (ray.direction.norm().y() + 1.0);
        return Vec4::from(1.0-t) + t * Vec4::from3(0.5, 0.7, 1.0);
    }
}

fn main() {
    let mut world = Scene::new();

    let sphere = Sphere::new(Vec4::from3(0.0, 0.0, -1.0), 0.5);

    let s = strahl::primitives::Primitive::SphereT{obj: sphere, mat: 0};

    world.add(s);

    let width = 800;
    let height = 450;

    let origin = Vec4::zero();
    let target = Vec4::from3(0.0, 0.0, -1.0);
    let up = Vec4::from3(0.0, 1.0, 0.0);

    let cam = Camera::new(origin, target, up, 60.0, width as f32 / height as f32, 0.0, 100.0);

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f32 / width as f32;
        let v = y as f32 / height as f32;

        let ray = cam.get_ray(u, v);

        let col = color(&ray, &world) * 255.99;

        let r = col.r() as u8;
        let g = col.g() as u8;
        let b = col.b() as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("test.png").unwrap();
}
