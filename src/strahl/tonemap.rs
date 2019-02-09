use super::vec::*;

pub trait Tonemap
{
    fn tonemap(&self, color: &Vec4) -> Vec4;
}

pub struct LinearTonemap
{
    pub inv_gamma: f32
}

impl LinearTonemap
{
    pub fn new(gamma: f32) -> LinearTonemap
    {
        LinearTonemap{inv_gamma: 1.0 / gamma}
    }
}

impl Tonemap for LinearTonemap
{
    fn tonemap(&self, color: &Vec4) -> Vec4
    {
        color.pow3(self.inv_gamma)
    }
}