use super::vec::*;
use super::ray::*;
use super::hit::*;
use super::random::*;

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

#[derive(Copy, Clone)]
pub struct Lambertian
{
    pub albedo: Vec4
}

#[derive(Copy, Clone)]
pub struct Emissive
{
    pub emissive: Vec4
}

#[derive(Copy, Clone)]
pub struct Metal
{
    pub albedo: Vec4,
    pub roughness: f32
}

pub trait Scatter
{
    fn scatter(&self, _r: &Ray, _hit: &HitInfo, _out_mat: &mut MaterialInfo, _out_ray: &mut Ray) -> bool;
} 

#[derive(Copy, Clone)]
pub enum Material
{
    Lambertian {mat: Lambertian},
    Emissive {mat: Emissive},
    Metal {mat: Metal}
}

//######################################################################
// Lambertian
//######################################################################

impl Lambertian
{
    pub fn new(r: f32, g: f32, b: f32) -> Lambertian
    {
        Lambertian{albedo: Vec4::from3(r, g, b)}
    }

    pub fn material(&self) -> Material
    {
        Material::Lambertian {mat: *self}
    }
}

impl Scatter for Lambertian
{
    fn scatter(&self, _r: &Ray, _hit: &HitInfo, _out_mat: &mut MaterialInfo, _out_ray: &mut Ray) -> bool
    {
        let target = _hit.point + _hit.normal + random_in_unit_sphere();

        *_out_ray = Ray::new(_hit.point, (target - _hit.point).norm());

        _out_mat.attenuation = self.albedo;
        _out_mat.emission = Vec4::zero();

        true
    }
}

//######################################################################
// Emissive
//######################################################################

impl Emissive
{
    pub fn new(r: f32, g: f32, b: f32) -> Emissive
    {
        Emissive{emissive: Vec4::from3(r, g, b)}
    }

    pub fn material(&self) -> Material
    {
        Material::Emissive {mat: *self}
    }
}

impl Scatter for Emissive
{
    fn scatter(&self, _r: &Ray, _hit: &HitInfo, _out_mat: &mut MaterialInfo, _out_ray: &mut Ray) -> bool
    {
        _out_mat.attenuation = Vec4::one();
        _out_mat.emission = self.emissive;

        false
    }
}

//######################################################################
// Metal
//######################################################################

impl Metal
{
    pub fn new(r: f32, g: f32, b: f32, _roughness: f32) -> Metal
    {
        Metal{albedo: Vec4::from3(r, g, b), roughness: _roughness}
    }

    pub fn material(&self) -> Material
    {
        Material::Metal {mat: *self}
    }
}

impl Scatter for Metal
{
    fn scatter(&self, _r: &Ray, _hit: &HitInfo, _out_mat: &mut MaterialInfo, _out_ray: &mut Ray) -> bool
    {
        let target = _r.direction.reflect(&_hit.normal) + self.roughness * random_in_unit_sphere();

        *_out_ray = Ray::new(_hit.point, (target - _hit.point).norm());

        _out_mat.attenuation = self.albedo;
        _out_mat.emission = Vec4::zero();

        _out_ray.direction.dot(&_hit.normal) > 0.0
    }
}