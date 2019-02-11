use super::hit::*;
use super::vec::*;
use super::ray::*;

#[derive(Copy, Clone)]
pub struct Sphere
{
    pos: Vec4,
    radius: f32,
    compute_uv: bool
}

#[derive(Copy, Clone)]
pub struct Plane
{
    pos: Vec4,
    normal: Vec4,
    //compute_uv: bool
}

#[derive(Copy, Clone)]
pub struct BBox
{
    center: Vec4,
    dimensions: Vec4,
    inv_dimensions: Vec4
}

#[derive(Copy, Clone)]
pub enum Primitive
{
    Sphere {obj: Sphere, mat: u32},
    Plane {obj: Plane, mat: u32},
    BBox {obj: BBox, mat: u32}
}

//######################################################################
// Sphere
//######################################################################

impl Sphere
{
    pub fn new(_pos: Vec4, _radius: f32) -> Sphere
    {
        Sphere{pos: _pos, radius: _radius, compute_uv: false}
    }

    pub fn new_with_uv(_pos: Vec4, _radius: f32) -> Sphere
    {
        Sphere{pos: _pos, radius: _radius, compute_uv: true}
    }

    pub fn primitive(&self, _mat: u32) -> Primitive
    {
        Primitive::Sphere{obj: *self, mat: _mat}
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, out: &mut HitInfo, min: f32, max: f32) -> bool {
        let op = r.origin - self.pos;
        let a = r.direction.square_length();
        let b = op.dot(&r.direction);
        let c = op.square_length() - self.radius*self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let sq_discriminant = discriminant.sqrt();            
            let depth1 = (-b - sq_discriminant) / a;
            let mut valid_hit = depth1 > min && depth1 < max;

            if valid_hit { // solution 1
                out.depth = depth1;
                out.point = r.point_at(depth1);
            } else {
                let depth2 = (-b + sq_discriminant) / a;
                valid_hit = depth2 > min && depth2 < max;

                if valid_hit { // solution 2
                    out.depth = depth2;
                    out.point = r.point_at(depth2);
                }
            }

            if valid_hit {
                out.normal = (out.point - self.pos) / self.radius;

                if self.compute_uv
                {
                    //out.u = out.normal.x().atan2(out.normal.z()) / (std::f32::consts::PI * 2.0) + 0.5;
                    //out.v = out.normal.z() * 0.5 + 0.5;

                    let p = out.normal;
                    let phi = p.z().atan2(p.x());
                    let theta = p.y().asin();
                    out.u = (phi + std::f32::consts::PI) / (std::f32::consts::PI * 2.0);
                    out.v = 1.0 - (theta + std::f32::consts::PI * 0.5) * std::f32::consts::FRAC_1_PI;
                }
            }

            return valid_hit;
        }

        false
    }
}

//######################################################################
// Plane
//######################################################################

impl Plane
{
    pub fn new(_pos: Vec4, _normal: Vec4) -> Plane
    {
        Plane{pos: _pos, normal: _normal}
    }

    pub fn primitive(&self, _mat: u32) -> Primitive
    {
        Primitive::Plane{obj: *self, mat: _mat}
    }
}

impl Hitable for Plane
{
    fn hit(&self, r: &Ray, out: &mut HitInfo, min: f32, max: f32) -> bool
    {
        let denom = self.normal.dot(&r.direction);

        if denom > 0.0 
        {
            let depth = (self.pos - r.origin).dot(&self.normal) / denom;

            if depth > min && depth < max
            {
                out.depth = depth;
                out.normal = self.normal;
                out.point = r.point_at(out.depth);
                return true;
            }
        }

        false
    }
}

//######################################################################
// BBox
//######################################################################

impl BBox
{
    pub fn new(_center: Vec4, _dimensions: Vec4) -> BBox
    {
        BBox{center: _center, dimensions: _dimensions, inv_dimensions: 1.0 / _dimensions}
    }

    pub fn primitive(&self, _mat: u32) -> Primitive
    {
        Primitive::BBox{obj: *self, mat: _mat}
    }
}

impl Hitable for BBox
{
    // http://www.jcgt.org/published/0007/03/04/paper-lowres.pdf
    fn hit(&self, r: &Ray, out: &mut HitInfo, min: f32, max: f32) -> bool
    {
        let ray_origin = r.origin - self.center;

        let winding = if (ray_origin.abs() * self.inv_dimensions).max_elem3() < 1.0 {-1.0} else {1.0};

        let sign = -r.direction.sign();

        let dist = (self.dimensions * sign * winding - ray_origin) * self.inv_dimensions;

        // TODO:
        //# defineTEST(U, VW) (d.U >= 0.0) && \all(lessThan(abs(ray.origin.VW + ray.dir.VW*d.U), box.radius.VW))bvec3 test = bvec3(TEST(x, yz), TEST(y, zx), TEST(z, xy));sgn = test.x ? vec3(sgn.x,0,0) : (test.y ? vec3(0,sgn.y,0) :vec3(0,0,test.z ? sgn.z:0));# undefTEST

        let (x, y, z) = dist.extract3();

        out.depth = if x != 0.0 {x} else { if y != 0.0 {y} else {z} };
        out.normal = sign;

        out.depth > min && out.depth < max
    }
}