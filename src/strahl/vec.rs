use packed_simd::{f32x4, shuffle, u32x4};
use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Vec4 {
    pub v: f32x4,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {
            v: f32x4::new(x, y, z, w),
        }
    }

    pub fn from3(x: f32, y: f32, z: f32) -> Vec4 {
        Vec4 {
            v: f32x4::new(x, y, z, 0.0),
        }
    }

    pub fn zero() -> Vec4 {
        Vec4 {
            v: f32x4::splat(0.0)
        }
    }

    pub fn one() -> Vec4 {
        Vec4 {
            v: f32x4::splat(1.0)
        }
    }

    pub fn get(&self) -> &f32x4 {
        &self.v
    }

    pub fn x(&self) -> f32 {
        self.v.extract(0)
    }

    pub fn y(&self) -> f32 {
        self.v.extract(1)
    }

    pub fn z(&self) -> f32 {
        self.v.extract(2)
    }

    pub fn w(&self) -> f32 {
        self.v.extract(3)
    }

    pub fn r(&self) -> f32 {
        self.v.extract(0)
    }

    pub fn g(&self) -> f32 {
        self.v.extract(1)
    }

    pub fn b(&self) -> f32 {
        self.v.extract(2)
    }

    pub fn a(&self) -> f32 {
        self.v.extract(3)
    }

    pub fn swizzle(&self, x: u32, y: u32, z: u32, w: u32) -> Vec4 {
        Vec4 {
            v: self.v.shuffle1_dyn(u32x4::new(x, y, z, w)),
        }
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
            v: self.v / self.length(),
        }
    }

    pub fn norm3(&self) -> Self {
        let l = self.v.replace(3, 0.0);
        Vec4 {
            v: l / (l * l).sum().sqrt(),
        }
    }

    // this version also works with Vec4 if it was initialized using Vec4::from3() to null the w component
    pub fn cross3(&self, o: &Vec4) -> Vec4 {
        // http://threadlocalmutex.com/?p=8
        let a_yzx = shuffle!(self.v, [1, 2, 0, 3]);
        let b_yzx = shuffle!(o.v, [1, 2, 0, 3]);

        Vec4 {
            v: shuffle!(self.v * b_yzx - a_yzx * o.v, [1, 2, 0, 3])
        }
    }

    // null w component 
    pub fn cross3_trimmed(&self, o: &Vec4) -> Vec4 {
        let res = self.cross3(&o);
        Vec4{v: res.v.replace(3, 0.0)}
    }

    pub fn cross3_validate(&self, o: &Vec4) -> Vec4 {
        Vec4::new(
            &self.y() * o.z() - &self.z() * o.y(),
            &self.z() * o.x() - &self.x() * o.z(),
            &self.x() * o.y() - &self.y() * o.x(),
            0.0,
        )
    }
}

//######################################################################
// From
//######################################################################

impl From<[f32; 4]> for Vec4 {
    fn from(v: [f32; 4]) -> Self {
        Vec4::new(v[0], v[1], v[2], v[3])
    }
}

impl From<[f32; 3]> for Vec4 {
    fn from(v: [f32; 3]) -> Self {
        Vec4::new(v[0], v[1], v[2], 0.0)
    }
}

impl From<[f32; 2]> for Vec4 {
    fn from(v: [f32; 2]) -> Self {
        Vec4::new(v[0], v[1], 0.0, 0.0)
    }
}

impl From<f32> for Vec4 {
    fn from(v: f32) -> Self {
        Vec4::new(v, v, v, v)
    }
}

impl From<&f32> for Vec4 {
    fn from(v: &f32) -> Self {
        Vec4::new(*v, *v, *v, *v)
    }
}

impl From<f32x4> for Vec4 {
    fn from(o: f32x4) -> Self {
        Vec4 { v: o }
    }
}

impl From<&f32x4> for Vec4 {
    fn from(o: &f32x4) -> Self {
        Vec4 { v: *o }
    }
}

//######################################################################
// Into
//######################################################################

impl Into<f32x4> for Vec4 {
    fn into(self) -> f32x4 {
        self.v
    }
}

//######################################################################
// Add Vec4 + Vec4
//######################################################################

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, o: Vec4) -> Vec4 {
        Vec4 { v: self.v + o.v }
    }
}

impl Add<&Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, o: &Vec4) -> Vec4 {
        Vec4 { v: self.v + o.v }
    }
}

impl AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, o: Vec4) {
        self.v += o.v
    }
}

impl AddAssign<&Vec4> for Vec4 {
    fn add_assign(&mut self, o: &Vec4) {
        self.v += o.v
    }
}

//######################################################################
// Add Vec4 + f32
//######################################################################

impl Add<f32> for Vec4 {
    type Output = Vec4;

    fn add(self, o: f32) -> Vec4 {
        Vec4 { v: self.v + o }
    }
}

impl Add<&f32> for Vec4 {
    type Output = Vec4;

    fn add(self, o: &f32) -> Vec4 {
        Vec4 { v: self.v + *o }
    }
}

impl AddAssign<f32> for Vec4 {
    fn add_assign(&mut self, o: f32) {
        self.v += o
    }
}

impl AddAssign<&f32> for Vec4 {
    fn add_assign(&mut self, o: &f32) {
        self.v += *o
    }
}

impl Add<Vec4> for f32 {
    type Output = Vec4;

    fn add(self, o: Vec4) -> Vec4 {
        Vec4 { v: o.v + self}
    }
}

impl Add<&Vec4> for f32 {
    type Output = Vec4;

    fn add(self, o: &Vec4) -> Vec4 {
        Vec4 { v: o.v + self}
    }
}

//######################################################################
// Sub
//######################################################################

impl Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, o: Vec4) -> Vec4 {
        Vec4 { v: self.v - o.v }
    }
}

impl Sub<&Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, o: &Vec4) -> Vec4 {
        Vec4 { v: self.v - o.v }
    }
}

impl SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, o: Vec4) {
        self.v -= o.v
    }
}

impl SubAssign<&Vec4> for Vec4 {
    fn sub_assign(&mut self, o: &Vec4) {
        self.v -= o.v
    }
}

//######################################################################
// Sub Vec4 - f32
//######################################################################

impl Sub<f32> for Vec4 {
    type Output = Vec4;

    fn sub(self, o: f32) -> Vec4 {
        Vec4 { v: self.v - o }
    }
}

impl Sub<&f32> for Vec4 {
    type Output = Vec4;

    fn sub(self, o: &f32) -> Vec4 {
        Vec4 { v: self.v - *o }
    }
}

impl SubAssign<f32> for Vec4 {
    fn sub_assign(&mut self, o: f32) {
        self.v -= o
    }
}

impl SubAssign<&f32> for Vec4 {
    fn sub_assign(&mut self, o: &f32) {
        self.v -= *o
    }
}

impl Sub<Vec4> for f32 {
    type Output = Vec4;

    fn sub(self, o: Vec4) -> Vec4 {
        Vec4 { v: self - o.v}
    }
}

impl Sub<&Vec4> for f32 {
    type Output = Vec4;

    fn sub(self, o: &Vec4) -> Vec4 {
        Vec4 { v: self - o.v }
    }
}

//######################################################################
// Mul
//######################################################################

impl Mul<&Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, o: &Vec4) -> Vec4 {
        Vec4 { v: self.v * o.v }
    }
}

impl MulAssign<&Vec4> for Vec4 {
    fn mul_assign(&mut self, o: &Vec4) {
        self.v *= o.v
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, o: f32) -> Vec4 {
        Vec4 { v: self.v * o }
    }
}

impl Mul<Vec4> for f32
{
    type Output = Vec4;

    fn mul(self, o: Vec4) -> Vec4 {
        Vec4 { v: o.v * self}
    }
}

impl Mul<&Vec4> for f32
{
    type Output = Vec4;

    fn mul(self, o: &Vec4) -> Vec4 {
        Vec4 { v: o.v * self}
    }
}

impl Mul<&f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, o: &f32) -> Vec4 {
        Vec4 { v: self.v * *o }
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, o: f32) {
        self.v *= o
    }
}

//######################################################################
// Div
//######################################################################

impl Div<&Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, o: &Vec4) -> Vec4 {
        Vec4 { v: self.v / o.v }
    }
}

impl DivAssign<&Vec4> for Vec4 {
    fn div_assign(&mut self, o: &Vec4) {
        self.v /= o.v
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, o: f32) -> Vec4 {
        Vec4 { v: self.v / o }
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, o: f32) {
        self.v /= o
    }
}

impl PartialEq for Vec4 {
    fn eq(&self, o: &Vec4) -> bool {
        self.v == o.v
    }
}

//######################################################################
// Equal
//######################################################################