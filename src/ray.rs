mod vec;

pub struct Ray {
    pub origin: vec::Vec4;
    pub direction: vec::Vec4;
}

impl Ray {
    pub fn new(_origin: vec::Vec4, _dir: vec::Vec4) -> Ray {
        Ray {
            origin: _origin),
            direction: _dir)
        }
    }

    pub fn point_at(dist: &f32) -> vec::Vec4
    {
        vec::Vec4::from(origin + dist * direction)
    }
}