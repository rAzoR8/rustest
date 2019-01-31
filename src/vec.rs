use packed_simd::f32x4;
use std::ops::{Add, AddAssign, Sub, SubAssign, Index, IndexMut, Mul, MulAssign, Div, DivAssign};

pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new() -> Vec4 {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn swizzle(&self, idx: [u32; 4]) -> Vec4 {
        Vec4 {
            x: *self.index(idx[0]),
            y: *self.index(idx[1]),
            z: *self.index(idx[2]),
            w: *self.index(idx[3]),
        }
    }

    pub fn get(&self) -> f32x4 {
        f32x4::new(self.x, self.y, self.z, self.w)
    }

    //######################################################################
    // Dot product
    //######################################################################

    pub fn dot_simd(&self, o: &Vec4) -> f32 {
        let l = f32x4::new(self.x, self.y, self.z, self.w);
        let r = f32x4::new(o.x, o.y, o.z, o.w);

        (l * r).sum() // wrapping_sum() ?
    }

    pub fn dot(&self, o: &Vec4) -> f32 {
        &self.x * &o.x + &self.y * &o.y + &self.z * &o.z + &self.w * &o.w
    }

    pub fn dot3(&self, o: &Vec4) -> f32 {
        &self.x * &o.x + &self.y * &o.y + &self.z * &o.z
    }

    pub fn dot3_simd(&self, o: &Vec4) -> f32 {
        let l = f32x4::new(self.x, self.y, self.z, 0.0);
        let r = f32x4::new(o.x, o.y, o.z, 0.0);

        (l * r).sum()
    }

    //######################################################################
    // Square length
    //######################################################################

    pub fn square_length(&self) -> f32 {
        self.dot(&self)
    }

    pub fn square_length_simd(&self) -> f32 {
        let l = f32x4::new(self.x, self.y, self.z, self.w);
        (l * l).sum()
    }

    pub fn square_length3(&self) -> f32 {
        self.dot3(&self)
    }

    pub fn square_length3_simd(&self) -> f32 {
        let l = f32x4::new(self.x, self.y, self.z, 0.0);
        (l * l).sum()
    }

    //######################################################################
    // Length
    //######################################################################

    pub fn length(&self) -> f32 {
        self.dot(&self).sqrt()
    }

    pub fn length_simd(&self) -> f32 {
        self.square_length_simd().sqrt()
    }

    pub fn length3(&self) -> f32 {
        self.dot3(&self).sqrt()
    }

    pub fn length3_simd(&self) -> f32 {
        self.square_length_simd().sqrt()
    }

    //######################################################################
    // Normalize
    //######################################################################

    pub fn normalize(&mut self) -> &mut Self {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
        self.w /= length;

        self
    }

    pub fn normalize_simd(&mut self) -> &mut Self {
        let mut l = f32x4::new(self.x, self.y, self.z, self.w);
        l /= (l * l).sum().sqrt();
        self.x = l.extract(0);
        self.y = l.extract(1);
        self.z = l.extract(2);
        self.w = l.extract(3);

        self
    }

    pub fn normalize3(&mut self) -> &mut Self {
        let length = self.length3();
        self.x /= length;
        self.y /= length;
        self.z /= length;

        self
    }

    pub fn normalize3_simd(&mut self) -> &mut Self {
        let mut l = f32x4::new(self.x, self.y, self.z, 0.0);
        l /= (l * l).sum().sqrt();
        self.x = l.extract(0);
        self.y = l.extract(1);
        self.z = l.extract(2);

        self
    }

    //######################################################################
    // Norm
    //######################################################################

    pub fn norm(&self) -> Self {
        let length = self.length();
        Vec4 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
            w: self.w / length,
        }
    }

    pub fn norm3(&self) -> Self {
        let length = self.length3();
        Vec4 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
            w: 0.0,
        }
    }

    pub fn norm_simd(&self) -> Self {
        let mut v = f32x4::new(self.x, self.y, self.z, self.w);
        v /= (v * v).sum().sqrt();

        Vec4::from(v)
    }

    pub fn norm3_simd(&self) -> Self {
        let mut v = f32x4::new(self.x, self.y, self.z, 0.0);
        v /= (v * v).sum().sqrt();

        Vec4::from(v)
    }

    //######################################################################
    // Cross product
    //######################################################################

    pub fn cross3(&self, o: &Vec4) -> Vec4 {
        Vec4 {
            x: &self.y * o.z - &self.z * o.y,
            y: &self.z * o.x - &self.x * o.z,
            z: &self.x * o.y - &self.y * o.x,
            w: 0.0,
        }
    }

    pub fn cross3_simd(&self, o: &Vec4) -> Vec4 {
        // left
        let mut a = f32x4::new(self.y, self.z, self.x, 0.0);
        let mut b = f32x4::new(o.z, o.x, o.y, 0.0);

        let mut res = a * b;

        // right
        a = f32x4::new(self.z, self.x, self.y, 0.0);
        b = f32x4::new(o.y, o.z, o.x, 0.0);

        res -= a * b;

        Vec4 {
            x: res.extract(0),
            y: res.extract(1),
            z: res.extract(2),
            w: 0.0,
        }
    }
} // impl Vec4

//######################################################################
// Index
//######################################################################

impl Index<u32> for Vec4 {
    type Output = f32;

    fn index(&self, idx: u32) -> &f32 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => &self.x,
        }
    }
}

impl IndexMut<u32> for Vec4 {
    fn index_mut(&mut self, idx: u32) -> &mut f32 {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => &mut self.x,
        }
    }
}

//######################################################################
// From
//######################################################################

impl From<[f32; 4]> for Vec4 {
    fn from(v: [f32; 4]) -> Self {
        Vec4 {
            x: v[0],
            y: v[1],
            z: v[2],
            w: v[3],
        }
    }
}

impl From<[f32; 3]> for Vec4 {
    fn from(v: [f32; 3]) -> Self {
        Vec4 {
            x: v[0],
            y: v[1],
            z: v[2],
            w: 0.0,
        }
    }
}

impl From<[f32; 2]> for Vec4 {
    fn from(v: [f32; 2]) -> Self {
        Vec4 {
            x: v[0],
            y: v[1],
            z: 0.0,
            w: 0.0,
        }
    }
}

//replicate
impl From<f32> for Vec4 {
    fn from(v: f32) -> Self {
        Vec4 {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }
}

impl From<f32x4> for Vec4 {
    fn from(v: f32x4) -> Self {
        Vec4 {
            x: v.extract(0),
            y: v.extract(1),
            z: v.extract(2),
            w: v.extract(3),
        }
    }
}

impl From<&f32x4> for Vec4 {
    fn from(v: &f32x4) -> Self {
        Vec4 {
            x: v.extract(0),
            y: v.extract(1),
            z: v.extract(2),
            w: v.extract(3),
        }
    }
}

//######################################################################
// Into
//######################################################################

impl Into<f32x4> for Vec4 {
    fn into(self) -> f32x4 {
        f32x4::new(self.x, self.y, self.z, self.w)
    }
}

//######################################################################
// Add
//######################################################################

impl Add<&Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, o: &Vec4) -> Vec4 {
        Vec4::from(self.get() + o.get())
    }
}

impl AddAssign<&Vec4> for Vec4 {
    fn add_assign(&mut self, o: &Vec4) {
        *self = Vec4::from(self.get() + o.get())
    }
}

//######################################################################
// Sub
//######################################################################

impl Sub<&Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, o: &Vec4) -> Vec4 {
        Vec4::from(self.get() - o.get())
    }
}

impl SubAssign<&Vec4> for Vec4 {
    fn sub_assign(&mut self, o: &Vec4) {
        *self = Vec4::from(self.get() - o.get())
    }
}

//######################################################################
// Mul
//######################################################################

impl Mul<&Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, o: &Vec4) -> Vec4 {
        Vec4::from(self.get() * o.get())
    }
}

impl MulAssign<&Vec4> for Vec4 {
    fn mul_assign(&mut self, o: &Vec4) {
        *self = Vec4::from(self.get() * o.get())
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, o: f32) -> Vec4 {
        Vec4::from(self.get() * f32x4::splat(o))
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, o: f32) {
        *self = Vec4::from(self.get() * f32x4::splat(o))
    }
}

//######################################################################
// Div
//######################################################################

impl Div<&Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, o: &Vec4) -> Vec4 {
        Vec4::from(self.get() / o.get())
    }
}

impl DivAssign<&Vec4> for Vec4 {
    fn div_assign(&mut self, o: &Vec4) {
        *self = Vec4::from(self.get() / o.get())
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, o: f32) -> Vec4 {
        Vec4::from(self.get() / f32x4::splat(o))
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, o: f32) {
        *self = Vec4::from(self.get() / f32x4::splat(o))
    }
}