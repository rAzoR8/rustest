use super::vec::*;
use super::ray::*;
use super::hit::*;
use super::random::*;
use super::texture::*;

#[derive(Copy, Clone)]
pub struct MaterialInfo
{
    pub attenuation: Vec4,
    pub emission: Vec4
}

impl MaterialInfo
{
    pub fn new() -> MaterialInfo
    {
        MaterialInfo
        {
            attenuation: Vec4::one(),
            emission: Vec4::zero()
        }
    }
}

#[derive(Clone)]
pub struct Lambertian
{
    pub albedo: Texture
}

#[derive(Clone)]
pub struct Emissive
{
    pub emissive: Texture
}

#[derive(Clone)]
pub struct Metal
{
    pub albedo: Texture,
    pub roughness: f32
}

#[derive(Clone)]
pub struct Background
{
    pub color: Texture,
    pub strength: f32
}

pub trait Scatter
{
    fn scatter(&self, _r: &mut Ray, _hit: &HitInfo, _out_mat: &mut MaterialInfo) -> bool;
} 

//######################################################################
// Material
//######################################################################

#[derive(Clone)]
pub enum Material
{
    Lambertian {mat: Lambertian},
    Emissive {mat: Emissive},
    Metal {mat: Metal},
    Background {mat: Background}
}

impl Scatter for Material
{
    fn scatter(&self, _r: &mut Ray, _hit: &HitInfo, _out_mat: &mut MaterialInfo) -> bool
    {
        let scattered = match self
        {
            Material::Lambertian {mat} => {mat.scatter(_r, &_hit, _out_mat)},
            Material::Emissive {mat} => {mat.scatter(_r, &_hit, _out_mat)},
            Material::Metal {mat} => {mat.scatter(_r, &_hit, _out_mat)},
            Material::Background {mat} => {mat.scatter(_r, &_hit, _out_mat)}
        };

        scattered
    }
}

//######################################################################
// Lambertian
//######################################################################

impl Lambertian
{
    pub fn new(r: f32, g: f32, b: f32) -> Material
    {
        Material::Lambertian{mat: Lambertian{albedo: ConstantTexture::from(r, g, b).texture()}}
    }

    pub fn from_path<P>(path: P, _type: DynamicTextureType) -> Material
    where P: AsRef<std::path::Path>
    {
        Material::Lambertian{mat: Lambertian{albedo: Texture::DynamicTexture{tex: DynamicTexture::new(path, _type)}}}
    }
}

impl Scatter for Lambertian
{
    fn scatter(&self, _r: &mut Ray, _hit: &HitInfo, _out_mat: &mut MaterialInfo) -> bool
    {
        let target = _hit.point + _hit.normal + random_in_unit_sphere();

        *_r = Ray::new(_hit.point, target - _hit.point);

        _out_mat.attenuation = self.albedo.sample(_hit);
        _out_mat.emission = Vec4::zero();

        true
    }
}

//######################################################################
// Emissive
//######################################################################

impl Emissive
{
    pub fn new(r: f32, g: f32, b: f32) -> Material
    {
        Material::Emissive{mat: Emissive{emissive: ConstantTexture::from(r, g, b).texture()}}
    }

    pub fn from_path<P>(path: P, _type: DynamicTextureType) -> Material
    where P: AsRef<std::path::Path>
    {
        Material::Emissive{mat: Emissive{emissive: Texture::DynamicTexture{tex: DynamicTexture::new(path, _type)}}}
    }
}

impl Scatter for Emissive
{
    fn scatter(&self, _r: &mut Ray, _hit: &HitInfo, _out_mat: &mut MaterialInfo) -> bool
    {
        _out_mat.attenuation = Vec4::one();
        _out_mat.emission = self.emissive.sample(_hit);

        false
    }
}

//######################################################################
// Metal
//######################################################################

impl Metal
{
    pub fn new(r: f32, g: f32, b: f32, _roughness: f32) -> Material
    {
        Material::Metal{mat: Metal{albedo: ConstantTexture::from(r, g, b).texture(), roughness: _roughness}}
    }

    pub fn from_path<P>(path: P, _type: DynamicTextureType, _roughness: f32) -> Material
    where P: AsRef<std::path::Path>
    {
        Material::Metal{mat: Metal{albedo: Texture::DynamicTexture{tex: DynamicTexture::new(path, _type)}, roughness: _roughness}}
    }
}

impl Scatter for Metal
{
    fn scatter(&self, _r: &mut Ray, _hit: &HitInfo, _out_mat: &mut MaterialInfo) -> bool
    {
        let mut target = _r.direction.reflect(&_hit.normal);
        
        if self.roughness > 0.0
        {
            target += self.roughness * random_in_unit_sphere();
        }  

        *_r = Ray::new(_hit.point, target);

        _out_mat.attenuation = self.albedo.sample(_hit);
        _out_mat.emission = Vec4::zero();

        _r.direction.dot(&_hit.normal) > 0.0
    }
}

//######################################################################
// Background Grad
//######################################################################

impl Background
{
    pub fn new(_color: Vec4, _strength: f32) -> Material
    {
        Material::Background{mat: Background{strength: _strength, color: ConstantTexture::new(&_color).texture()}}
    }

    pub fn from_path<P>(path: P, _type: DynamicTextureType, _strength: f32) -> Material
    where P: AsRef<std::path::Path>
    {
        Material::Background{mat: Background{color: Texture::DynamicTexture{tex: DynamicTexture::new(path, _type)}, strength: _strength}}
    }
}

impl Scatter for Background
{
    fn scatter(&self, _r: &mut Ray, _hit: &HitInfo, _out_mat: &mut MaterialInfo) -> bool
    {
        let t = _r.direction.norm().y() + 1.0;
        _out_mat.emission = (Vec4::from(1.0-t) + t * self.color.sample(_hit)) * self.strength;
        _out_mat.attenuation = Vec4::one();
        false
    }
}