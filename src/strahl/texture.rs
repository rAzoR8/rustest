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

#[derive(Copy, Clone)]
pub enum DynamicTextureType
{
    Linear,
    sRGB, // TODO: add gamma?
    RGBe
}

#[derive(Clone)]
pub struct DynamicTexture
{
    pub img: image::DynamicImage,
    pub load_type: DynamicTextureType
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
    pub fn new<P>(path: &P, _type: DynamicTextureType) -> DynamicTexture
    where P: AsRef<Path>
    {
        DynamicTexture{img: image::open(path).unwrap(), load_type: _type}
    }
}

impl Sample for DynamicTexture
{
    fn sample(&self, hit: &HitInfo) -> Vec4
    {
        let (x, y) = self.img.dimensions();
        let data = self.img.get_pixel(x * hit.u as u32, y * hit.v as u32);
        //let rgba = pixel.channels();

        let in_color = Vec4::new(data[0] as f32 , data[1] as f32,  data[2] as f32, data[3] as f32) / 255.0;

        let out_color = match self.load_type
        {
            DynamicTextureType::Linear => {in_color},
            DynamicTextureType::sRGB  => {in_color.pow3(2.2)},
            DynamicTextureType::RGBe => {in_color.pow3(in_color.a())} 
        };

        out_color
    }
}