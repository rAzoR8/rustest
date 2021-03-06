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

    pub fn from(r: f32, g: f32, b: f32) -> ConstantTexture
    {
        ConstantTexture{color: Vec4::from3(r,g,b)}
    }

    pub fn texture(&self) -> Texture
    {
        Texture::ConstantTexture{tex: *self}
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
    pub fn new<P>(path: P, _type: DynamicTextureType) -> DynamicTexture
    where P: AsRef<Path>
    {
        let img_file = match image::open(path)
        {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{}", e);
                image::DynamicImage::new_rgb8(1, 1)
            } 
        };

        DynamicTexture{img: img_file, load_type: _type}
    }    

    pub fn texture<P>(path: P, _type: DynamicTextureType) -> Texture
    where P: AsRef<Path>
    {
        let img_file = match image::open(path)
        {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{}", e);
                image::DynamicImage::new_rgb8(1, 1)
            } 
        };

        Texture::DynamicTexture{tex: DynamicTexture{img: img_file, load_type: _type}}
    }
}

impl Sample for DynamicTexture
{
    fn sample(&self, hit: &HitInfo) -> Vec4
    {
        let (x, y) = self.img.dimensions();
        let u = (x as f32 * hit.u) as u32;
        let v = (y as f32 * hit.v) as u32;
        let data = self.img.get_pixel(u.min(x-1), v.min(y-1));

        let in_color = Vec4::new(data[0] as f32 , data[1] as f32,  data[2] as f32, data[3] as f32);

        let out_color = match self.load_type
        {
            DynamicTextureType::Linear => {in_color / 255.0},
            DynamicTextureType::sRGB  => {(in_color / 255.0).pow3(2.2)},
            DynamicTextureType::RGBe => { (in_color / 255.0) * (2.0 as f32).powf(in_color.a() - 128.0)} 
        };

        // https://github.com/opencv/opencv/blob/master/modules/imgcodecs/src/rgbe.cpp

        out_color
    }
}