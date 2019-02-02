use super::vec::*;
use super::ray::*;
use rand::prelude::*;
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
    pub vertical: Vec4
}

impl Camera 
{
    pub fn new(origin: Vec4, target: Vec4, up: Vec4, fovy: f32, aspect: f32,  lense_diameter: f32, far: f32) -> Camera
    {
        let half_height = (fovy*PI/360.0).tan();
        let half_width = aspect * half_height;
        let _w = (origin - target).norm();
        let _u = (up.cross3(&_w)).norm();
        let _v = _w.cross3(&_u);

        Camera
        {
            pos: origin,
            w: _w,
            u: _u,
            v: _v,
            lense_radius: lense_diameter / 2.0,
            lower_left_corner: origin - half_width*far*_u -half_height*far*_v - far*_w,
            horizontal: 2.0*half_width*far*_u,
            vertical: 2.0*half_height*far*_v
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

    pub fn random_in_unit_disk() -> Vec4
    {
        let mut rng = rand::thread_rng();

        loop 
        {
            let p = 2.0 * Vec4::from3(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 0.0) - Vec4::from3(1.0, 1.0, 0.0);
            if p.square_length() <= 1.0 { return p;}
        }
    }

    pub fn random_in_unit_sphere() -> Vec4
    {
        let mut rng = rand::thread_rng();

        loop 
        {
            let p = 2.0 * Vec4::from3(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)) - Vec4::one();
            if p.square_length() <= 1.0 { return p;}
        }
    }

    pub fn get_random_ray(&self, s: f32, t: f32) -> Ray
    {
        let offset = self.lense_radius * Camera::random_in_unit_disk();

        Ray
        {
            origin: self.pos + offset,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.pos - offset
        }
    }
}