use super::vec::*;
use std::ops::{Mul, MulAssign};

pub struct Mat4
{
    pub a: Vec4,
    pub b: Vec4,
    pub c: Vec4,
    pub d: Vec4
}

impl Mat4
{
    // https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry/row-major-vs-column-major-vector
    pub fn new() -> Mat4
    {
        Mat4
        {
            a: Vec4::new(1.0, 0.0, 0.0, 0.0),
            b: Vec4::new(0.0, 1.0, 0.0, 0.0),
            c: Vec4::new(0.0, 0.0, 1.0, 0.0),
            d: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}