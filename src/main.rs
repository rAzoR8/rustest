mod vec;

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

use image::{GenericImage, ImageBuffer, imageops};

fn main() {
    type Vec4 = vec::Vec4;

    let v =  Vec4::from([2.0, 5.0, 5.0]);
    let v2 = Vec4::from([1.0, 2.0, 3.0]);
    let mut kdk = v.cross3(&v2);
    let test = v.cross3_simd(&v2);
    let d = kdk.dot(&test);

    let norm = kdk.norm_simd();
    let l = kdk.normalize_simd().length();

    let width = 800;
    let height = 800;

    // Create a new ImgBuf with width: imgx and height: imgy
    //let mut imgbuf = RgbaImage::new(width as u32, height as u32);

    //let mut imgbuf: image::ImageBuffer<image::Rgba<u8>, _> = image::ImageBuffer::new(width as u32, height as u32);
    //let mut imgbuf: image::RgbaImage = image::ImageBuffer::new(width as u32, height as u32);

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    imgbuf.save("test.png").unwrap();
}
