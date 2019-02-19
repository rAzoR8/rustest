use super::camera::*;
use super::vec::*;
use super::ray::*;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct AS3DCamera
{
    pos: std::vec::Vec<Vec4>,
    lower_left_corner: std::vec::Vec<Vec4>,
    horizontal: std::vec::Vec<Vec4>,
    vertical: std::vec::Vec<Vec4>,
    width: u32,
    height: u32,
    samples: u32,
    lenticular_slope: f32
}

impl AS3DCamera {
    pub fn new(origin: Vec4, target: Vec4, up: Vec4, fovy: f32, _width: u32, _height: u32, far: f32, _samples: u32, step_width: f32, view_count: u32, slope_y: f32, slope_x: f32) -> AS3DCamera
    {
        let half_height = (fovy*PI/360.0).tan();
        let half_width = ((_width as f32) / (_height as f32)) * half_height;

        let mut llcs = std::vec::Vec::with_capacity(view_count as usize);
        let mut origins = std::vec::Vec::with_capacity(view_count as usize);
        let mut verticals = std::vec::Vec::with_capacity(view_count as usize);
        let mut horizontals = std::vec::Vec::with_capacity(view_count as usize);

        let mut cur_origin = origin - ((view_count as f32 - 1.0) * 0.5) * step_width;
        for _ in 0..view_count {
            origins.push(cur_origin);

            let _w = (cur_origin - target).norm();
            let _u = (up.cross3(&_w)).norm();
            let _v = _u.cross3(&_w);

            llcs.push(cur_origin - half_width*far*_u -half_height*far*_v - far*_w);
            verticals.push(2.0*half_height*far*_v);
            horizontals.push(2.0*half_width*far*_u);

            cur_origin += step_width;
        }

        AS3DCamera
        {
            pos: origins,
            lower_left_corner: llcs,
            horizontal: horizontals,
            vertical: verticals,
            width: _width,
            height: _height,
            samples: _samples,
            lenticular_slope: slope_y / slope_x
        }
    }
}

impl Camera for AS3DCamera
{
    fn get_ray(&self, x: f32, y: f32, channel: Channel) -> Ray
    {
        let s = x / (self.width as f32);
        let t = y / (self.height as f32);

        let view = ((x * 3.0 + y * self.lenticular_slope + 4.0) + (channel as u32) as f32) as usize % self.pos.len();

        Ray
        {
            origin: self.pos[view],
            direction: self.lower_left_corner[view] + s*self.horizontal[view] + t*self.vertical[view] - self.pos[view]
        }
    }

    fn mode(&self ) -> Mode { Mode::Separate }

    fn sample_count(&self) -> u32 {self.samples}
    fn width(&self) -> u32 {self.width}
    fn height(&self) -> u32 {self.height}
}

unsafe impl Sync for AS3DCamera{}