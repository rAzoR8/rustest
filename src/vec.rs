pub mod vec {
    use packed_simd::f32x4;
    use std::ops::{Index,IndexMut};
    
    pub struct Vec4
    {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32
    }

    impl Vec4 {
        pub fn new() -> Vec4
        {
            Vec4{x: 0.0, y: 0.0, z :0.0, w : 0.0}
            //Vec4{d: f32x4::splat(0.)}
        }

        // pub fn from_elements(x: f32, y: f32, z: f32, w: f32) -> Vec4
        // {
        //     Vec4{d: f32x4::new(x, y, z, w)}
        // }

        // pub fn get(&self) -> &f32x4
        // {
        //     &self.d
        // }

        // pub fn get_mut(&mut self) -> &mut f32x4
        // {
        //     &mut self.d
        // }
        
        // pub fn shuffle(&self, indices: packed_simd::Simd<[u32; 4]>) -> Vec4
        // {
        //     Vec4{d: self.d.shuffle1_dyn(indices)}
        // }

        pub fn dot_simd(&self, o: &Vec4) -> f32
        {
            let l = f32x4::new(self.x, self.y, self.z, self.w);
            let r = f32x4::new(o.x, o.y, o.z, o.w);

            (l * r).sum() // wrapping_sum() ?
        }

        pub fn dot(&self, o: &Vec4) -> f32
        {
            &self.x * &o.x + &self.y * &o.y + &self.z * &o.z + &self.w * &o.w
        }

        pub fn cross3(&self, o: &Vec4) -> Vec4
        {
            Vec4
            {
                x: &self.y * o.z - &self.z * o.y,
                y: &self.z * o.x - &self.x * o.z,
                z: &self.x * o.y - &self.y * o.x,
                w: 0.0
            }
        }

        pub fn cross3_simd(&self, o: &Vec4) -> Vec4
        {
            // left
            let mut a = f32x4::new(self.y, self.z, self.x, 0.0);
            let mut b = f32x4::new(o.z, o.x, o.y, 0.0);

            let mut res = a * b;

            // right
            a = f32x4::new(self.z, self.x, self.y, 0.0);
            b = f32x4::new(o.y, o.z, o.x, 0.0);

            res -= a * b;

            Vec4
            {
                x: res.extract(0),
                y: res.extract(1),
                z: res.extract(2),
                w: 0.0,
            }
        }
    } // impl Vec4

    impl Index<u32> for Vec4
    {
        type Output = f32;

        fn index(&self, idx: u32) -> &f32
        {
            match idx
            {
                0 => &self.x,
                1 => &self.y,
                2 => &self.z,
                3 => &self.w,
                _ => &self.x
            }
        }
    }

    impl IndexMut<u32> for Vec4
    {
        fn index_mut(&mut self, idx: u32) -> &mut f32
        {
            match idx
            {
                0 => &mut self.x,
                1 => &mut self.y,
                2 => &mut self.z,
                3 => &mut self.w,
                _ => &mut self.x
            }
        }
    }

    impl From<[f32; 4]> for Vec4
    {
        fn from(v: [f32; 4]) -> Self
        {
            Vec4{x: v[0], y: v[1], z:v[2], w: v[3] }
        }
    }

    impl From<[f32; 3]> for Vec4
    {
        fn from(v: [f32; 3]) -> Self
        {
            Vec4{x: v[0], y: v[1], z:v[2], w: 0.0 }
        }
    }

    impl From<[f32; 2]> for Vec4
    {
        fn from(v: [f32; 2]) -> Self
        {
            Vec4{x: v[0], y: v[1], z: 0.0, w: 0.0 }
        }
    }

    //replicate
    impl From<f32> for Vec4
    {
        fn from(v: f32) -> Self
        {
            Vec4{x: v, y: v, z: v, w: v }
        }
    }

    // Traits
} // vec
