pub struct FRandomStream {
    initial_seed: u32,
    seed: u32,
}

impl FRandomStream {
    pub fn new(seed: u32) -> Self {
        Self {
            initial_seed: seed,
            seed,
        }
    }
    pub fn seed(&self) -> u32 {
        self.seed
    }
    pub fn sed_seed(&mut self, seed: u32) {
        self.seed = seed
    }
    pub fn next_seed(&self) -> u32 {
        self.seed.wrapping_mul(0xbb38435).wrapping_add(0x3619636b)
    }
    pub fn mutate(&mut self) {
        self.seed = self.next_seed();
        //println!("MUTATE SEED {}", self.0);
    }
    pub fn get_fraction(&mut self) -> f32 {
        self.mutate();
        f32::from_bits(0x3f800000 | self.seed as u32 >> 9) - 1.0
    }
    pub fn get_unsigned_int(&mut self) -> u32 {
        //self.mutate();
        self.seed
    }
    pub fn rand_helper(&mut self, max: i32) -> i32 {
        if max > 0 {
            (self.get_fraction() * (max as f32)) as i32
        } else {
            0
        }
    }
    pub fn rand_range(&mut self, min: i32, max: i32) -> i32 {
        min + self.rand_helper(max - min + 1)
    }
    pub fn rand_item<'a, T>(&mut self, slice: &'a [T]) -> &'a T {
        let i = self.rand_helper(slice.len() as i32) as usize;
        &slice[i]
    }
    pub fn rand_swap_remove<'a, T>(&mut self, vec: &mut Vec<T>) -> T {
        let i = self.rand_helper(vec.len() as i32) as usize;
        vec.swap_remove(i)
    }
    pub fn rand_remove<'a, T>(&mut self, vec: &mut Vec<T>) -> T {
        let i = self.rand_helper(vec.len() as i32) as usize;
        vec.remove(i)
    }
}
