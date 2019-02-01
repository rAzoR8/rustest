// use hit;

// pub struct Sphere
// {
//     pos: vec::Vec4,
//     radius: f32
// }

// impl Sphere
// {
//     pub fn new(_pos: vec::Vec4, _radius: f32) -> Sphere
//     {
//         Sphere{pos: _pos, radius: _radius}
//     }
// }

// impl hit::Hitable for Sphere
// {
//     fn(&self, r: &ray::Ray, out: &mut HitInfo, f32 min, f32 max) -> bool
//     {
//         let oc = r.origin() - self.center;
//         true
//     }
// }