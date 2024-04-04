use crate::prelude::*;

pub fn musgrave_noise_1d(rng: &mut UniformRandomGen, x: f32, seed: u32) -> f32 {
    let x1 = perlin_noise_1d(rng, x + 0.5, seed);
    perlin_noise_1d(rng, x1, seed)
}

pub fn musgrave_noise_2d(rng: &mut UniformRandomGen, x: f32, y: f32, seed: u32) -> f32 {
    let x1 = perlin_noise_2d(rng, x + 0.5, y + 0.5, seed);
    let y1 = perlin_noise_2d(rng, x + 3.83, y + 3.83, seed);

    perlin_noise_2d(rng, x1, y1, seed)
}

pub fn musgrave_noise_3d(rng: &mut UniformRandomGen, x: f32, y: f32, z: f32, seed: u32) -> f32 {
    let x1 = perlin_noise_3d(rng, x + 0.5, y + 0.5, z + 0.5, seed);
    let y1 = perlin_noise_3d(rng, x + 3.83, y + 3.83, z + 3.83, seed);
    let z1 = perlin_noise_3d(rng, x + 8.27, y + 8.27, z + 8.27, seed);

    perlin_noise_3d(rng, x1, y1, z1, seed)
}

pub fn musgrave_noise_4d(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
    t: f32,
    seed: u32,
) -> f32 {
    let x1 = perlin_noise_4d(rng, x + 0.5, y + 0.5, z + 0.5, t + 0.5, seed);
    let y1 = perlin_noise_4d(rng, x + 3.83, y + 3.83, z + 3.83, t + 3.83, seed);
    let z1 = perlin_noise_4d(rng, x + 8.27, y + 8.27, z + 8.27, t + 8.27, seed);
    let t1 = perlin_noise_4d(rng, x + 13.82, y + 13.82, z + 13.82, t + 13.82, seed);

    perlin_noise_4d(rng, x1, y1, z1, t1, seed)
}
