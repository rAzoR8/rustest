use super::vec::*;
use super::mat::*;

use std::ops::{Mul, MulAssign, Neg};

#[derive(Copy, Clone)]
pub struct Quat
{
    // complex 3, w scalar
    pub q: Vec4
}

// http://www.essentialmath.com/GDC2012/GDC2012_JMV_Rotations.pdf
impl Quat {
    // Identity
    pub fn new() -> Quat {
        Quat {
            q: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn from_axis_angle_rad(axis: &Vec4, angle_rad: f32) -> Quat {
        let h = angle_rad * 0.5;
        let mut q = h.sin() * axis;
        q.set(h.cos(), 3);
        Quat { q: q }
    }

    pub fn from_axis_angle_deg(axis: &Vec4, angle_deg: f32) -> Quat {
        Quat::from_axis_angle_rad(axis, angle_deg * std::f32::consts::PI / 180.0)
    }

    pub fn conjugate(&self) -> Quat {
        Quat::from(self.q * Vec4::new(-1.0, -1.0, -1.0, 1.0))
    }

    pub fn inverse(&self) -> Quat {
        Quat::from(self.conjugate().q / self.q.square_length())
    }

    // w component of point v should be 0.0
    pub fn rotate(&self, v: &Vec4) -> Vec4 {
        (self * Quat::from(v) * self.inverse()).q
    }

    pub fn inverse_rotate(&self, v: &Vec4) -> Vec4 {
        (self.inverse() * Quat::from(v) * self).q
    }

    pub fn rotate_unit(&self, v: &Vec4) -> Vec4 {
        (self * Quat::from(v) * self.conjugate()).q
    }

    pub fn inverse_rotate_unit(&self, v: &Vec4) -> Vec4 {
        (self.conjugate() * Quat::from(v) * self).q
    }

    // concatenation order is right-to-left => q0q1 = q1 * q0
    fn multiply(&self, o: &Quat) -> Quat {
        //https://gist.github.com/mattatz/40a91588d5fb38240403f198a938a593
        //var.xyz = q2.xyz * q1.w + q1.xyz * q2.w + cross(q1.xyz, q2.xyz);
        //var.w = q1.w * q2.w - dot(q1.xyz, q2.xyz);

        let w1 = self.q.w();
        let w2 = o.q.w();

        let mut r = o.q * w1 + self.q * w2 + self.q.cross3(&o.q);
        r.set(w1 * w2 - self.q.dot3(&o.q), 3);

        Quat::from(r)
    }

    pub fn to_mat3(&self) -> Mat4 {
        Mat4 {
            c: [
                self.rotate_unit(&Vec4::from3(1.0, 0.0, 0.0)),
                self.rotate_unit(&Vec4::from3(0.0, 1.0, 0.0)),
                self.rotate_unit(&Vec4::from3(0.0, 0.0, 1.0)),
                Vec4::new(0.0, 0.0, 0.0, 1.0),
            ],
        }
    }
}

//######################################################################
// From
//######################################################################

impl From<Vec4> for Quat {
    fn from(o: Vec4) -> Self {
        Quat { q: o }
    }
}

impl From<&Vec4> for Quat {
    fn from(o: &Vec4) -> Self {
        Quat { q: *o }
    }
}

//######################################################################
// Into
//######################################################################

impl Into<Mat4> for Quat {
    fn into(self) -> Mat4 {
        self.to_mat3()
    }
}

//######################################################################
// Mul Quat x Quat
//######################################################################

impl Mul<Quat> for Quat {
    type Output = Quat;

    fn mul(self, o: Quat) -> Quat {
        self.multiply(&o)
    }
}

impl Mul<Quat> for &Quat {
    type Output = Quat;

    fn mul(self, o: Quat) -> Quat {
        self.multiply(&o)
    }
}

impl Mul<&Quat> for Quat {
    type Output = Quat;

    fn mul(self, o: &Quat) -> Quat {
        self.multiply(&o)
    }
}

impl Mul<&Quat> for &Quat {
    type Output = Quat;

    fn mul(self, o: &Quat) -> Quat {
        self.multiply(&o)
    }
}

impl MulAssign<Quat> for Quat {
    fn mul_assign(&mut self, o: Quat) {
        *self = self.multiply(&o)
    }
}

impl MulAssign<&Quat> for Quat {
    fn mul_assign(&mut self, o: &Quat) {
        *self = self.multiply(&o)
    }
}

//######################################################################
// Neq
//######################################################################

impl Neg for Quat {
    type Output = Quat;

    fn neg(self) -> Quat {
        self.conjugate()
    }
}