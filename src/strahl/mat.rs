use super::vec::*;
use std::ops::{Mul, MulAssign};

#[derive(Copy, Clone)]
pub struct Mat4
{
    // column layout: c[0] = first column
    pub c: [Vec4; 4]
}

impl Mat4 {
    // https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry/row-major-vs-column-major-vector
    pub fn new() -> Mat4 {
        Mat4 {
            c: [
                Vec4::new(1.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 1.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, 1.0, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 1.0),
            ]
        }
    }

    pub fn transpose(&self) -> Mat4
    {
         Mat4 {
            c: [
                Vec4::new(self.c[0].x(), self.c[1].x(), self.c[2].x(), self.c[3].x()),
                Vec4::new(self.c[0].y(), self.c[1].y(), self.c[2].y(), self.c[3].y()),
                Vec4::new(self.c[0].z(), self.c[1].z(), self.c[2].z(), self.c[3].z()),
                Vec4::new(self.c[0].w(), self.c[1].w(), self.c[2].w(), self.c[3].w()),
            ]
        }
    }

    fn mul_vec(self, o: Vec4) -> Vec4
    {
        Vec4::new(self.c[0].dot(&o),self.c[1].dot(&o), self.c[2].dot(&o), 0.0)
    }
}

//######################################################################
// Mat4 x Mat4
//######################################################################

impl Mul<Mat4> for Mat4
{
    type Output = Mat4;

    fn mul(self, o: Mat4) -> Mat4
    {
        let m = o.transpose();

        Mat4 {
            c: [
                Vec4::new(self.c[0].dot(&m.c[0]), self.c[0].dot(&m.c[1]), self.c[0].dot(&m.c[2]), self.c[0].dot(&m.c[3])),
                Vec4::new(self.c[1].dot(&m.c[0]), self.c[1].dot(&m.c[1]), self.c[1].dot(&m.c[2]), self.c[1].dot(&m.c[3])),
                Vec4::new(self.c[2].dot(&m.c[0]), self.c[2].dot(&m.c[1]), self.c[2].dot(&m.c[2]), self.c[2].dot(&m.c[3])),
                Vec4::new(self.c[3].dot(&m.c[0]), self.c[3].dot(&m.c[1]), self.c[3].dot(&m.c[2]), self.c[3].dot(&m.c[3])),
            ]
        }
    }
}

//######################################################################
// Mat4 x Vec4
//######################################################################

impl Mul<Vec4> for Mat4
{
    type Output = Vec4;

    fn mul(self, o: Vec4) -> Vec4
    {
        Vec4::new(self.c[0].dot(&o),self.c[1].dot(&o), self.c[2].dot(&o), self.c[3].dot(&o))
    }
}

//######################################################################
// Equal
//######################################################################

impl PartialEq for Mat4 {
    fn eq(&self, o: &Mat4) -> bool {
        self.c[0] == o.c[0] && self.c[1] == o.c[1] && self.c[2] == o.c[2] && self.c[3] == o.c[3]
    }
}
