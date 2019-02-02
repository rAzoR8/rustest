pub mod strahl;

use crate::strahl::hit::Hitable;

// pub fn dot_prod(a: &[f32], b: &[f32]) -> f32 {
//     assert_eq!(a.len(), b.len());
//     assert!(a.len() % 4 == 0);

//     let mut sum = f32x4::splat(0.);

//     for i in (0..a.len()).step_by(4) {
//         sum += f32x4::from_slice_unaligned(&a[i..])
//             * f32x4::from_slice_unaligned(&b[i..]);
//     }

//     sum.sum()
// }

extern crate image;

use image::{ImageBuffer, imageops};

fn main() {
    type Vec4 = strahl::vec::Vec4;
    type Ray = strahl::ray::Ray;
    type Sphere = strahl::primitives::Sphere;
    type HitInfo = strahl::hit::HitInfo;

    let mut v =  Vec4::from3(0.0, 0.0, 1.0);
    let v2 =  Vec4::from3(1.0, 2.0, 3.0);

    let c1 = v.cross3_trimmed(&v2);
    let c2 = v.cross3_validate(&v2);

    let c3 = c1 + c2 * 2.0;

    if c1 != c2
    {
        print!("hey!")
    }


    let test = Vec4::from(0.0);

    let ray = Ray::new(test, Vec4::from3(0.0, 0.0, 1.0));

    let s = Sphere::new(Vec4::from3(0.0, 0.0, 2.0), 1.0);

    let mut hit = HitInfo::new();
    let didhit = s.hit(&ray, &mut hit, 0.0, 100.0);

    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    imgbuf.save("test.png").unwrap();
}
