pub mod strahl;
use crate::strahl::hit::Hitable;

extern crate image;

use image::{ImageBuffer, imageops};

type Vec4 = strahl::vec::Vec4;
type Ray = strahl::ray::Ray;
type Sphere = strahl::primitives::Sphere;
type HitInfo = strahl::hit::HitInfo;

fn color(ray: &Ray, obj: &Hitable) -> Vec4
{
    let mut hit = HitInfo::new();

    if obj.hit(&ray, &mut hit, 0.0, 100.0)
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
    let s = Sphere::new(Vec4::from3(0.0, 0.0, -1.0), 0.5);

    let width = 200;
    let height = 100;

    let llc = Vec4::from3(-2.0, -1.0, -1.0);
    let horizontal = Vec4::from3(4.0, 0.0, 0.0);
    let vertical = Vec4::from3(0.0, 2.0, 0.0);
    let origin = Vec4::zero();

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f32 / width as f32;
        let v = y as f32 / height as f32;

        let ray = Ray::new(origin, llc + u*horizontal + v*vertical);

        let col = color(&ray, &s) * 255.99;

        let r = col.r() as u8;
        let g = col.g() as u8;
        let b = col.b() as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("test.png").unwrap();
}
