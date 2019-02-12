use super::vec::*;
use super::ray::*;
use super::random::*;
use std::f32::consts::PI;

use std::marker::{Send, Sync};
pub trait Camera: Send + Sync
{
    //pub fn get_ray(&self, s: f32, t: f32) -> Ray

    fn sample_count(&self) -> u32;
    fn width(&self) -> u32;
    fn height(&self) -> u32;

    // floating pixel coordinates in [widthxheight]
    fn get_ray(&self, x: f32, y: f32) -> Ray;
}

#[derive(Copy, Clone)]
pub struct PerspectiveCamera
{
    pos: Vec4,
    w: Vec4, // look dir
    u: Vec4, 
    v: Vec4,
    lense_radius: f32,
    lower_left_corner: Vec4,
    horizontal: Vec4,
    vertical: Vec4,
    width: u32,
    height: u32,
    samples: u32
}

impl PerspectiveCamera {
    pub fn new(origin: Vec4, target: Vec4, up: Vec4, fovy: f32, _width: u32, _height: u32,  lense_diameter: f32, far: f32, _samples: u32) -> PerspectiveCamera
    {
        let half_height = (fovy*PI/360.0).tan();
        let half_width = ((_width as f32) / (_height as f32)) * half_height;
        let _w = (origin - target).norm();
        let _u = (up.cross3(&_w)).norm();
        let _v = _u.cross3(&_w);

        PerspectiveCamera
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

    // pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    //     Ray {
    //         origin: self.pos,
    //         direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.pos,
    //     }
    // }

    // pub fn get_ray2(&self, x: u32, y: u32) -> Ray {
    //     let s = (x as f32) / (self.width as f32);
    //     let t = (y as f32) / (self.height as f32);

    //     Ray {
    //         origin: self.pos,
    //         direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.pos,
    //     }
    // }

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

impl Camera for PerspectiveCamera
{
    fn get_ray(&self, x: f32, y: f32) -> Ray
    {
        let s = (x as f32) / (self.width as f32);
        let t = (y as f32) / (self.height as f32);

        Ray
        {
            origin: self.pos,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.pos
        }
    }

    fn sample_count(&self) -> u32 {self.samples}
    fn width(&self) -> u32 {self.width}
    fn height(&self) -> u32 {self.height}
}

unsafe impl Sync for PerspectiveCamera{}
