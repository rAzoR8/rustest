use super::vec::*;

pub trait Tonemap
{
    fn tonemap(&self, color: &Vec4) -> Vec4;
}

#[derive(Copy, Clone)]
pub struct LinearTonemap
{
    pub inv_gamma: f32
}

#[derive(Copy, Clone)]
pub struct Uncharted2Tonemap
{
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub inv_white_scale: Vec4,
    pub inv_gamma: f32,
    pub exposure: f32
}

#[derive(Copy, Clone)]
pub struct ReinhardTonemap
{
    pub inv_gamma: f32,
    pub exposure: f32
}

#[derive(Copy, Clone)]
pub enum TonemapOperator
{
    LinearTonemap {op: LinearTonemap},
    Uncharted2Tonemap {op: Uncharted2Tonemap},
    ReinhardTonemap {op: ReinhardTonemap}
}

//######################################################################
// LinearTonemap
//######################################################################

impl LinearTonemap
{
    pub fn new(gamma: f32) -> LinearTonemap
    {
        LinearTonemap{inv_gamma: 1.0 / gamma}
    }

    pub fn operator(&self) -> TonemapOperator
    {
        TonemapOperator::LinearTonemap{op: *self}
    }
}

impl Tonemap for LinearTonemap
{
    fn tonemap(&self, color: &Vec4) -> Vec4
    {
        color.pow3(self.inv_gamma)
    }
}

//######################################################################
// Uncharted2Tonemap
//######################################################################

// http://filmicworlds.com/blog/filmic-tonemapping-operators/
impl Uncharted2Tonemap
{
    pub fn eval(&self, color: Vec4) -> Vec4
    {
        return ((color*(self.a*color+self.c*self.b)+self.d*self.e)/(color*(self.a*color+self.b)+self.d*self.f))-self.e/self.f;
    }

    pub fn new() -> Uncharted2Tonemap
    {
        let mut tmap = Uncharted2Tonemap{a: 0.15, b: 0.5, c: 0.1, d: 0.2, e: 0.02, f: 0.3, inv_white_scale: Vec4::one(), inv_gamma: 1.0 / 2.2, exposure: 2.0};
        tmap.inv_white_scale = 1.0 / tmap.eval(Vec4::from(11.2));
        tmap
    }

    pub fn from(_a: f32, _b: f32, _c: f32, _d: f32, _e: f32, _f: f32, _w: f32, _exposure: f32) -> Uncharted2Tonemap
    {
        let mut tmap = Uncharted2Tonemap{a: _a, b: _b, c: _c, d: _d, e: _e, f: _f, inv_white_scale: Vec4::one(), inv_gamma: 1.0 / 2.2, exposure: _exposure};
        tmap.inv_white_scale = 1.0 / tmap.eval(Vec4::from(_w));
        tmap
    }

    pub fn operator(&self) -> TonemapOperator
    {
        TonemapOperator::Uncharted2Tonemap{op: *self}
    }
}

impl Tonemap for Uncharted2Tonemap
{
    fn tonemap(&self, color: &Vec4) -> Vec4
    {
        (self.eval(self.exposure * color) * self.inv_white_scale).pow3(self.inv_gamma)
    }
}

//######################################################################
// ReinhardTonemap
//######################################################################

impl ReinhardTonemap
{
    pub fn new(gamma: f32, _exposure: f32) -> ReinhardTonemap
    {
        ReinhardTonemap{inv_gamma: 1.0 / gamma, exposure: _exposure}
    }

    pub fn operator(&self) -> TonemapOperator
    {
        TonemapOperator::ReinhardTonemap{op: *self}
    }
}

impl Tonemap for ReinhardTonemap
{
    fn tonemap(&self, color: &Vec4) -> Vec4
    {
        let mut col = self.exposure * color;
        col /= 1.0 + col;

        col.pow3(self.inv_gamma)
    }
}
