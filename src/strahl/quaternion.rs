use super::vec::*;

use std::ops::{Mul, MulAssign};

pub struct Quat
{
    // complex 3, w scalar
    pub q: Vec4
}

impl Quat
{
    pub fn new(axis: &Vec4, angle_deg: f32) -> Quat
    {
        let h = angle_deg * std::f32::consts::FRAC_PI_2 / 180.0;
        let mut q = h.sin() * axis;
        q.set(h.cos(), 3);
        Quat{q: q}
    }

    pub fn conjugate(&self) -> Quat
    {
        Quat::from(self.q * Vec4::new(-1.0, -1.0, -1.0, 1.0))
    }

    pub fn inverse(&self) -> Quat
    {
        Quat::from(self.conjugate().q / self.q.square_length())
    }

    // w component of point v should be 0.0
    pub fn rotate(&self, v: &Vec4) -> Vec4
    {
        (self * Quat::from(v) * self.inverse()).q
    }

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