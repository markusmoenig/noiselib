pub struct UniformRandomGen {
    seed: u32,
}

impl UniformRandomGen {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn reset(&mut self, seed: u32) {
        self.seed = seed;
    }

    pub fn get_(&mut self) -> f32 {
        self.seed = self
            .seed
            .wrapping_mul(987654323)
            .wrapping_add(self.seed ^ 0x78e98b58);

        // Using bitwise operations to construct the float
        let bitval = 0x3f800000 | (self.seed & 0x7fffff);
        // Safe conversion from u32 to f32 in Rust
        let result = f32::from_bits(bitval);
        result - 1.0
    }

    pub fn get(&mut self, seed: u32) -> f32 {
        self.seed = seed
            .wrapping_mul(987654323)
            .wrapping_add(self.seed ^ 0x78e98b58);

        // Using bitwise operations to construct the float
        let bitval = 0x3f800000 | (self.seed & 0x7fffff);
        // Safe conversion from u32 to f32 in Rust
        let result = f32::from_bits(bitval);
        result - 1.0
    }
}
