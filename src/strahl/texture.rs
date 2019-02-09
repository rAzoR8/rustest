use super::hit::*;
use super::vec::*;

extern crate image;
use image::{GenericImageView, Pixel};
use std::path::Path;

pub trait Sample
{
    fn sample(&self, hit: &HitInfo) -> Vec4;
}

#[derive(Copy, Clone)]
pub struct ConstantTexture
{
    pub color: Vec4
}

#[derive(Clone)]
pub struct DynamicTexture
{
    pub img: image::DynamicImage
}

#[derive(Clone)]
pub enum Texture
{
    ConstantTexture {tex: ConstantTexture},
    DynamicTexture {tex: DynamicTexture},
}

impl Sample for Texture
{
    fn sample(&self, hit: &HitInfo) -> Vec4
    {
        let pixel = match self
        {
            Texture::ConstantTexture {tex} => {tex.sample(&hit)},
            Texture::DynamicTexture {tex} => {tex.sample(&hit)}
        };

        pixel
    }
}

//######################################################################
// ConstantTexture
//######################################################################

impl ConstantTexture
{
    pub fn new(_color: &Vec4) -> ConstantTexture
    {
        ConstantTexture{color: *_color}
    }
}

impl Sample for ConstantTexture
{
    fn sample(&self, hit: &HitInfo) -> Vec4
    {
        self.color
    }
}

//######################################################################
// DynamicTexture
//######################################################################

impl DynamicTexture
{
    pub fn new<P>(path: &P) -> DynamicTexture
    where P: AsRef<Path>
    {
        DynamicTexture{img: image::open(path).unwrap()}
    }
}

impl Sample for DynamicTexture
{
    fn sample(&self, hit: &HitInfo) -> Vec4
    {
        let (x, y) = self.img.dimensions();
        let pixel = self.img.get_pixel(x * hit.u as u32, y * hit.v as u32);
        //let rgba = pixel.channels();
        Vec4::new(pixel[0] as f32 , pixel[1] as f32,  pixel[2] as f32, pixel[3] as f32) / 255.0
    }
}