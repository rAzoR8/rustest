//use packed_simd::*;
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

fn main() {
    type Vec4 = vec::Vec4;

    let v =  Vec4::from([2.0, 5.0, 5.0]);
    let v2 = Vec4::from([1.0, 2.0, 3.0]);
    let mut kdk = v.cross3(&v2);
    let test = v.cross3_simd(&v2);
    let d = kdk.dot(&test);

    let norm = kdk.norm_simd();
    let l = kdk.normalize_simd().length();

    let mut asspiss = String::new();

    let x = 3;
    let y = x * 3;

    asspiss += "test";

    println!("out {}", asspiss);

    //let mut sum = dot_prod(&[1.2, 1.4, 4.6, 3.5], &[1.2, 1.4, 4.6, 3.5]);
    //let mut sum2 = dot_prod(&[1.33, 1.4, 4.6, 3.5], &[1.2, 5.4, 4.6, 3.5]);

    //println!("sum {}", sum + sum2);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
