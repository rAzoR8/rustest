use super::hit::*;
use super::vec::*;
use super::ray::*;

pub struct Sphere
{
    pos: Vec4,
    radius: f32
}

pub struct Plane
{
    pos: Vec4,
    normal: Vec4
}

pub struct AABB
{
    min: Vec4,
    max: Vec4
}

pub enum Primitive
{
    SphereT {obj: Sphere, mat: u32},
    PlaneT {obj: Plane, mat: u32},
    AABBT {obj: AABB, mat: u32}
}

//######################################################################
// Hitable Impls
//######################################################################

impl Sphere
{
    pub fn new(_pos: Vec4, _radius: f32) -> Sphere
    {
        Sphere{pos: _pos, radius: _radius}
    }
}

impl Hitable for Sphere
{
    fn hit(&self, r: &Ray, out: &mut HitInfo, min: f32, max: f32) -> bool
    {
        let op = r.origin - self.pos;
        let a = r.direction.square_length();
        let b = op.dot(&r.direction);
        let c = op.square_length() - self.radius*self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0
        {
            let sq_discriminant = discriminant.sqrt();

            // solution 1
            let depth1 = (-b - sq_discriminant) / a;

            if depth1 > min && depth1 < max
            {
                out.depth = depth1;
                out.point = r.point_at(depth1);
                out.normal = ((out.point - self.pos) / a).norm();
                return true;
            }

            let depth2 = (-b + sq_discriminant) / a;

            if depth2 > min && depth2 < max
            {
                out.depth = depth2;
                out.point = r.point_at(depth2);
                out.normal = ((out.point - self.pos) / a).norm();
                return true;
            }            
        }

        false
    }
}