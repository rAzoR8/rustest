//#[cfg(feature = "into_bits")]
//use packed_simd::*;
//use packed_simd::{f32x4, shuffle, u32x4};
use packed_simd::{f32x4, shuffle, u32x4, FromBits};

use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg};

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

    pub fn extract(&self) -> (f32, f32, f32, f32)
    {
        (self.v.extract(0), self.v.extract(1), self.v.extract(2), self.v.extract(3))
    }

    pub fn extract3(&self) -> (f32, f32, f32)
    {
        (self.v.extract(0), self.v.extract(1), self.v.extract(2))
    }

    pub fn extract2(&self) -> (f32, f32)
    {
        (self.v.extract(0), self.v.extract(1))
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

    pub fn reflect(&self, n: &Vec4) -> Vec4
    {
        self.sub(2.0 * self.dot(n) * n)
    }

    pub fn sqrt(&self) -> Vec4
    {
        Vec4::new(self.x().sqrt(), self.y().sqrt(), self.z().sqrt(), self.w().sqrt())
    }

    pub fn sqrt3(&self) -> Vec4
    {
        Vec4::new(self.x().sqrt(), self.y().sqrt(), self.z().sqrt(), self.w())
    }

    pub fn fast_inv_sqrt(&self) -> Vec4
    {
        let half = self.v * 0.5;
        // our fitted version fro min max error: 0x5F387D4A
        let mut y = f32x4::from_bits(u32x4::splat(0x5f3759df) - (u32x4::from_bits(self.v) >> 1));

        // newton iteration
        y = y * (1.5 - (half * y * y));

        //y = y * (1.5 - (half * y * y));
        Vec4{v: y}
    }

    pub fn fast_sqrt(&self) -> Vec4
    {
        1.0 / self.fast_inv_sqrt()
    }

    pub fn exp(&self) -> Vec4
    {
        Vec4::new(self.x().exp(), self.y().exp(), self.z().exp(), self.w().exp())
    }

    pub fn exp3(&self) -> Vec4
    {
        Vec4::new(self.x().exp(), self.y().exp(), self.z().exp(), self.w())
    }

    pub fn pow(&self, exponent: f32) -> Vec4
    {
        Vec4::new(self.x().powf(exponent), self.y().powf(exponent), self.z().powf(exponent), self.w().powf(exponent))
    }

    pub fn pow3(&self, exponent: f32) -> Vec4
    {
        Vec4::new(self.x().powf(exponent), self.y().powf(exponent), self.z().powf(exponent), self.w())
    }

    pub fn min(&self, o: &Vec4) -> Vec4
    {
        Vec4{v: self.v.lt(o.v).select(self.v, o.v)}
    }

    pub fn max(&self, o: &Vec4) -> Vec4
    {
        Vec4{v: self.v.gt(o.v).select(self.v, o.v)}
    }

    pub fn max_elem(&self) -> f32
    {
        self.x().max(self.y().max(self.z().max(self.w())))
    }

    pub fn max_elem3(&self) -> f32
    {
        self.x().max(self.y().max(self.z()))
    }

    pub fn clamp(&self, min: &Vec4, max: &Vec4) -> Vec4
    {
        self.max(min).min(max)
    }

    pub fn clamp_scalar(&self, min: f32, max: f32) -> Vec4
    {
        self.max(&Vec4::from(min)).min(&Vec4::from(max))
    }

    // sign returns -1.0 if x is less than 0.0, 0.0 if x is equal to 0.0, and +1.0 if x is greater than 0.0. 
    pub fn sign(&self) -> Vec4
    {
        let zero = f32x4::splat(0.0);
        let sign = self.v.lt(f32x4::splat(0.0)).select(f32x4::splat(-1.0), f32x4::splat(1.0));
        Vec4{v: self.v.eq(f32x4::splat(0.0)).select(zero, sign)}
    }

    pub fn abs(&self) -> Vec4
    {
        // mask out sign bit
        Vec4{v: f32x4::from_bits(u32x4::from_bits(self.v) & u32x4::splat(0x7FFFFFFFu32))}
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
// Mul Vec4 x Vec4
//######################################################################

impl Mul<&Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, o: &Vec4) -> Vec4 {
        Vec4 { v: self.v * o.v }
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, o: Vec4) -> Vec4 {
        Vec4 { v: self.v * o.v }
    }
}

impl MulAssign<&Vec4> for Vec4 {
    fn mul_assign(&mut self, o: &Vec4) {
        self.v *= o.v
    }
}

impl MulAssign<Vec4> for Vec4 {
    fn mul_assign(&mut self, o: Vec4) {
        self.v *= o.v
    }
}

//######################################################################
// Mul Vec4 x f32
//######################################################################

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, o: f32) -> Vec4 {
        Vec4 { v: self.v * o }
    }
}

impl Mul<&f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, o: &f32) -> Vec4 {
        Vec4 { v: self.v * *o }
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

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, o: f32) {
        self.v *= o
    }
}

impl MulAssign<&f32> for Vec4 {
    fn mul_assign(&mut self, o: &f32) {
        self.v *= *o
    }
}

//######################################################################
// Div
//######################################################################

impl Div<Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, o: Vec4) -> Vec4 {
        Vec4 { v: self.v / o.v }
    }
}

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

impl DivAssign<Vec4> for Vec4 {
    fn div_assign(&mut self, o: Vec4) {
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

impl DivAssign<&f32> for Vec4 {
    fn div_assign(&mut self, o: &f32) {
        self.v /= *o
    }
}

impl Div<Vec4> for f32
{
    type Output = Vec4;

    fn div(self, o: Vec4) -> Vec4 {
        Vec4 { v: 1.0 / o.v }
    }
}

impl Div<&Vec4> for f32
{
    type Output = Vec4;

    fn div(self, o: &Vec4) -> Vec4 {
        Vec4 { v: 1.0 / o.v }
    }
}

//######################################################################
// Equal
//######################################################################

impl PartialEq for Vec4 {
    fn eq(&self, o: &Vec4) -> bool {
        self.v == o.v
    }
}

//######################################################################
// Neq
//######################################################################

impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Vec4 {
        Vec4{v: -self.v}
    }
}