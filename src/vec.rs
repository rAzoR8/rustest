pub mod vec {
    use packed_simd::f32x4;
    use std::ops::{Index,IndexMut};
    
    pub struct Vec4
    {
        pub d: f32x4
    }

    impl Vec4 {
        pub fn new() -> Vec4
        {
            Vec4{d: f32x4::splat(0.)}
        }

        pub fn from_elements(x: f32, y: f32, z: f32, w: f32) -> Vec4
        {
            Vec4{d: f32x4::new(x, y, z, w)}
        }

        pub fn get(&self) -> &f32x4
        {
            &self.d
        }

        pub fn get_mut(&mut self) -> &mut f32x4
        {
            &mut self.d
        }
        
        pub fn elem(&self, idx: usize) -> f32
        {
            self.d.extract(idx)
        }

        pub fn set(&mut self, v: &f32x4)
        {
            self.d = *v;
        }

        pub fn set_elem(&mut self, idx: usize, elem: f32)
        {
            self.d.replace(idx, elem);
        }

        pub fn x(&self) -> f32 {
            self.d.extract(0)
        }
        pub fn y(&self) -> f32 {
            self.d.extract(1)
        }
        pub fn z(&self) -> f32 {
            self.d.extract(2)
        }
        pub fn w(&self) -> f32 {
            self.d.extract(3)
        }

        pub fn r(&self) -> f32 {
            self.d.extract(0)
        }
        pub fn g(&self) -> f32 {
            self.d.extract(1)
        }
        pub fn b(&self) -> f32 {
            self.d.extract(2)
        }
        pub fn a(&self) -> f32 {
            self.d.extract(3)
        }

        pub fn shuffle(&self, indices: packed_simd::Simd<[u32; 4]>) -> Vec4
        {
            Vec4{d: self.d.shuffle1_dyn(indices)}
        }

        pub fn dot(&self, o: &Vec4) -> f32
        {
            (self.d * o.d).sum() // wrapping_sum() ?
        }

        //http://threadlocalmutex.com/?p=8
        // pub fn cross3(&self, o: &Vec4) -> Vec4
        // {
        //     let a_yzx = self.shuffle([3u, 0u, 2u, 1u]);
        //     //let a_yzx = self.shuffle([3, 0, 2, 1]);
        //     //let a_yzx = self.shuffle([3, 0, 2, 1]);
        //     Vec4{}
        // }
    } // impl Vec4

    // Traits
} // vec
