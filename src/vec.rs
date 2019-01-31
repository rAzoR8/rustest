use packed_simd::f32x4;
use std::ops::{Add, AddAssign, Sub, SubAssign, Index, IndexMut, Mul, MulAssign, Div, DivAssign};

pub struct Vec4
{
    pub v: f32x4
}

impl Vec4
{
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4
    {
        Vec4{v: f32x4::new(x, y, z, w)}
    }

    pub fn from(x: f32, y: f32, z: f32) -> Vec4
    {
        Vec4{v: f32x4::new(x, y, z, 0.0)}
    }

    pub fn dot(&self, o: &Vec4) -> f32 {
        (self.v * o.v).sum()
    }

    pub fn dot3(&self, o: &Vec4) -> f32 {
        (self.v * o.v.replace(3, 0.0)).sum()
    }

    pub fn square_length(&self) -> f32 {
        self.dot(self)
    }

    pub fn square_length3(&self) -> f32 {
        self.dot3(self)
    }

    pub fn length(&self) -> f32 {
        self.dot(&self).sqrt()
    }

    pub fn length3(&self) -> f32 {
        self.dot3(&self).sqrt()
    }

    pub fn normalize(&mut self) -> &mut Self {
        self.v /= self.length();

        self
    }

    pub fn normalize3(&mut self) -> &mut Self {
        self.v = self.v.replace(3, 0.0);
        self.v /= self.length();

        self
    }

    pub fn norm(&self) -> Self {
        Vec4 {
            v: self.v / self.length()
        }
    }

    pub fn norm3(&self) -> Self {
        let l = self.v.replace(3, 0.0);
        Vec4{v: l / (l * l).sum().sqrt()}
    }

}

//     //######################################################################
//     // Norm
//     //######################################################################

//     pub fn norm(&self) -> Self {
//         let length = self.length();
//         Vec4 {
//             x: self.x / length,
//             y: self.y / length,
//             z: self.z / length,
//             w: self.w / length,
//         }
//     }

//     pub fn norm3(&self) -> Self {
//         let length = self.length3();
//         Vec4 {
//             x: self.x / length,
//             y: self.y / length,
//             z: self.z / length,
//             w: 0.0,
//         }
//     }

//     pub fn norm_simd(&self) -> Self {
//         let mut v = f32x4::new(self.x, self.y, self.z, self.w);
//         v /= (v * v).sum().sqrt();

//         Vec4::from(v)
//     }

//     pub fn norm3_simd(&self) -> Self {
//         let mut v = f32x4::new(self.x, self.y, self.z, 0.0);
//         v /= (v * v).sum().sqrt();

//         Vec4::from(v)
//     }

//     //######################################################################
//     // Cross product
//     //######################################################################

//     pub fn cross3(&self, o: &Vec4) -> Vec4 {
//         Vec4 {
//             x: &self.y * o.z - &self.z * o.y,
//             y: &self.z * o.x - &self.x * o.z,
//             z: &self.x * o.y - &self.y * o.x,
//             w: 0.0,
//         }
//     }

//     pub fn cross3_simd(&self, o: &Vec4) -> Vec4 {
//         // left
//         let mut a = f32x4::new(self.y, self.z, self.x, 0.0);
//         let mut b = f32x4::new(o.z, o.x, o.y, 0.0);

//         let mut res = a * b;

//         // right
//         a = f32x4::new(self.z, self.x, self.y, 0.0);
//         b = f32x4::new(o.y, o.z, o.x, 0.0);

//         res -= a * b;

//         Vec4 {
//             x: res.extract(0),
//             y: res.extract(1),
//             z: res.extract(2),
//             w: 0.0,
//         }
//     }

// //######################################################################
// // Index
// //######################################################################

// impl Index<u32> for Vec4 {
//     type Output = f32;

//     fn index(&self, idx: u32) -> &f32 {
//         match idx {
//             0 => &self.x,
//             1 => &self.y,
//             2 => &self.z,
//             3 => &self.w,
//             _ => &self.x,
//         }
//     }
// }

// impl IndexMut<u32> for Vec4 {
//     fn index_mut(&mut self, idx: u32) -> &mut f32 {
//         match idx {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             2 => &mut self.z,
//             3 => &mut self.w,
//             _ => &mut self.x,
//         }
//     }
// }

// //######################################################################
// // From
// //######################################################################

// impl From<[f32; 4]> for Vec4 {
//     fn from(v: [f32; 4]) -> Self {
//         Vec4 {
//             x: v[0],
//             y: v[1],
//             z: v[2],
//             w: v[3],
//         }
//     }
// }

// impl From<[f32; 3]> for Vec4 {
//     fn from(v: [f32; 3]) -> Self {
//         Vec4 {
//             x: v[0],
//             y: v[1],
//             z: v[2],
//             w: 0.0,
//         }
//     }
// }

// impl From<[f32; 2]> for Vec4 {
//     fn from(v: [f32; 2]) -> Self {
//         Vec4 {
//             x: v[0],
//             y: v[1],
//             z: 0.0,
//             w: 0.0,
//         }
//     }
// }

// //replicate
// impl From<f32> for Vec4 {
//     fn from(v: f32) -> Self {
//         Vec4 {
//             x: v,
//             y: v,
//             z: v,
//             w: v,
//         }
//     }
// }

// impl From<f32x4> for Vec4 {
//     fn from(v: f32x4) -> Self {
//         Vec4 {
//             x: v.extract(0),
//             y: v.extract(1),
//             z: v.extract(2),
//             w: v.extract(3),
//         }
//     }
// }

// impl From<&f32x4> for Vec4 {
//     fn from(v: &f32x4) -> Self {
//         Vec4 {
//             x: v.extract(0),
//             y: v.extract(1),
//             z: v.extract(2),
//             w: v.extract(3),
//         }
//     }
// }

// //######################################################################
// // Into
// //######################################################################

// impl Into<f32x4> for Vec4 {
//     fn into(self) -> f32x4 {
//         f32x4::new(self.x, self.y, self.z, self.w)
//     }
// }

// //######################################################################
// // Add
// //######################################################################

// impl Add<&Vec4> for Vec4 {
//     type Output = Vec4;

//     fn add(self, o: &Vec4) -> Vec4 {
//         Vec4::from(self.get() + o.get())
//     }
// }

// impl AddAssign<&Vec4> for Vec4 {
//     fn add_assign(&mut self, o: &Vec4) {
//         *self = Vec4::from(self.get() + o.get())
//     }
// }

// //######################################################################
// // Sub
// //######################################################################

// impl Sub<&Vec4> for Vec4 {
//     type Output = Vec4;

//     fn sub(self, o: &Vec4) -> Vec4 {
//         Vec4::from(self.get() - o.get())
//     }
// }

// impl SubAssign<&Vec4> for Vec4 {
//     fn sub_assign(&mut self, o: &Vec4) {
//         *self = Vec4::from(self.get() - o.get())
//     }
// }

// //######################################################################
// // Mul
// //######################################################################

// impl Mul<&Vec4> for Vec4 {
//     type Output = Vec4;

//     fn mul(self, o: &Vec4) -> Vec4 {
//         Vec4::from(self.get() * o.get())
//     }
// }

// impl MulAssign<&Vec4> for Vec4 {
//     fn mul_assign(&mut self, o: &Vec4) {
//         *self = Vec4::from(self.get() * o.get())
//     }
// }

// impl Mul<f32> for Vec4 {
//     type Output = Vec4;

//     fn mul(self, o: f32) -> Vec4 {
//         Vec4::from(self.get() * f32x4::splat(o))
//     }
// }

// impl MulAssign<f32> for Vec4 {
//     fn mul_assign(&mut self, o: f32) {
//         *self = Vec4::from(self.get() * f32x4::splat(o))
//     }
// }

// //######################################################################
// // Div
// //######################################################################

// impl Div<&Vec4> for Vec4 {
//     type Output = Vec4;

//     fn div(self, o: &Vec4) -> Vec4 {
//         Vec4::from(self.get() / o.get())
//     }
// }

// impl DivAssign<&Vec4> for Vec4 {
//     fn div_assign(&mut self, o: &Vec4) {
//         *self = Vec4::from(self.get() / o.get())
//     }
// }

// impl Div<f32> for Vec4 {
//     type Output = Vec4;

//     fn div(self, o: f32) -> Vec4 {
//         Vec4::from(self.get() / f32x4::splat(o))
//     }
// }

// impl DivAssign<f32> for Vec4 {
//     fn div_assign(&mut self, o: f32) {
//         *self = Vec4::from(self.get() / f32x4::splat(o))
//     }
// }