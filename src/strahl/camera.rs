use super::vec::*;
use super::ray::*;
use super::random::*;
use std::f32::consts::PI;

pub struct Camera
{
    pub pos: Vec4,
    pub w: Vec4, // look dir
    pub u: Vec4, 
    pub v: Vec4,
    pub lense_radius: f32,
    pub lower_left_corner: Vec4,
    pub horizontal: Vec4,
    pub vertical: Vec4,
    pub width: u32,
    pub height: u32,
    pub samples: u32
}

impl Camera 
{
    pub fn new(origin: Vec4, target: Vec4, up: Vec4, fovy: f32, _width: u32, _height: u32,  lense_diameter: f32, far: f32, _samples: u32) -> Camera
    {
        let half_height = (fovy*PI/360.0).tan();
        let half_width = ((_width as f32) / (_height as f32)) * half_height;
        let _w = (origin - target).norm();
        let _u = (up.cross3(&_w)).norm();
        let _v = _u.cross3(&_w);

        Camera
        {
            pos: origin,
            w: _w,
            u: _u,
            v: _v,
            lense_radius: lense_diameter / 2.0,
            lower_left_corner: origin - half_width*far*_u -half_height*far*_v - far*_w,
            horizontal: 2.0*half_width*far*_u,
            vertical: 2.0*half_height*far*_v,
            width: _width,
            height: _height,
            samples: _samples
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray
    {
        Ray
        {
            origin: self.pos,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.pos
        }
    }

    pub fn get_random_ray(&self, s: f32, t: f32) -> Ray
    {
        let offset = self.lense_radius * random_in_unit_disk();

        Ray
        {
            origin: self.pos + offset,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.pos - offset
        }
    }
}