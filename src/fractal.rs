use crate::prelude::UniformRandomGen;

//
// Multifractal
//

#[allow(clippy::too_many_arguments)]
pub fn fractal_noise_mul_1d<F: Fn(&mut UniformRandomGen, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    noise_func: F,
    octaves: i32,
    freq_falloff: f32,
    lacunarity: f32,
    offset: f32,
    seed: u32,
) -> f32 {
    let mut rnd_val = 1.0;
    let mut power = 1.0;
    let mut x = x;

    for _i in 0..octaves {
        rnd_val *= (noise_func(rng, x, seed) + offset) * power;
        power *= freq_falloff;
        x *= lacunarity;
    }

    rnd_val
}

#[allow(clippy::too_many_arguments)]
pub fn fractal_noise_mul_2d<F: Fn(&mut UniformRandomGen, f32, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    noise_func: F,
    octaves: i32,
    freq_falloff: f32,
    lacunarity: f32,
    offset: f32,
    seed: u32,
) -> f32 {
    let mut rnd_val = 1.0;
    let mut power = 1.0;
    let mut x = x;
    let mut y = y;

    for _i in 0..octaves {
        rnd_val *= (noise_func(rng, x, y, seed) + offset) * power;
        power *= freq_falloff;
        x *= lacunarity;
        y *= lacunarity;
    }

    rnd_val
}
