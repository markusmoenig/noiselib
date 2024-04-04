use crate::prelude::UniformRandomGen;

//
// Multifractal
//

pub fn fractal_noise_add_1d<F: Fn(&mut UniformRandomGen, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    noise_func: F,
    octaves: i32,
    freq_falloff: f32,
    lacunarity: f32,
    seed: u32,
) -> f32 {
    let mut rnd_val = 0.0;

    let mut power = 1.0;
    let mut norma = power;

    let mut x = x;

    for _ in 0..octaves {
        rnd_val += noise_func(rng, x, seed) * power;
        norma += power;

        power *= freq_falloff;
        x *= lacunarity;
    }

    rnd_val / norma
}

#[allow(clippy::too_many_arguments)]
pub fn fractal_noise_add_2d<F: Fn(&mut UniformRandomGen, f32, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    noise_func: F,
    octaves: i32,
    freq_falloff: f32,
    lacunarity: f32,
    seed: u32,
) -> f32 {
    let mut rnd_val = 0.0;

    let mut power = 1.0;
    let mut norma = power;

    let mut x = x;
    let mut y = y;

    for _ in 0..octaves {
        rnd_val += noise_func(rng, x, y, seed) * power;
        norma += power;

        power *= freq_falloff;
        x *= lacunarity;
        y *= lacunarity;
    }

    rnd_val / norma
}

#[allow(clippy::too_many_arguments)]
pub fn fractal_noise_add_3d<F: Fn(&mut UniformRandomGen, f32, f32, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
    noise_func: F,
    octaves: i32,
    freq_falloff: f32,
    lacunarity: f32,
    seed: u32,
) -> f32 {
    let mut rnd_val = 0.0;

    let mut power = 1.0;
    let mut norma = power;

    let mut x = x;
    let mut y = y;
    let mut z = z;

    for _ in 0..octaves {
        rnd_val += noise_func(rng, x, y, z, seed) * power;
        norma += power;

        power *= freq_falloff;
        x *= lacunarity;
        y *= lacunarity;
        z *= lacunarity;
    }

    rnd_val / norma
}

#[allow(clippy::too_many_arguments)]
pub fn fractal_noise_add_4d<F: Fn(&mut UniformRandomGen, f32, f32, f32, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
    t: f32,
    noise_func: F,
    octaves: i32,
    freq_falloff: f32,
    lacunarity: f32,
    seed: u32,
) -> f32 {
    let mut rnd_val = 0.0;

    let mut power = 1.0;
    let mut norma = power;

    let mut x = x;
    let mut y = y;
    let mut z = z;
    let mut t = t;

    for _ in 0..octaves {
        rnd_val += noise_func(rng, x, y, z, t, seed) * power;
        norma += power;

        power *= freq_falloff;
        x *= lacunarity;
        y *= lacunarity;
        z *= lacunarity;
        t *= lacunarity;
    }

    rnd_val / norma
}

pub fn fractal_noise_add_abs_1d<F: Fn(&mut UniformRandomGen, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    noise_func: F,
    octaves: i32,
    freq_falloff: f32,
    lacunarity: f32,
    seed: u32,
) -> f32 {
    let mut rnd_val = 0.0;

    let mut power = 1.0;
    let mut norma = power;

    let mut x = x;

    for _ in 0..octaves {
        rnd_val += noise_func(rng, x, seed).abs() * power;
        norma += power;

        power *= freq_falloff;
        x *= lacunarity;
    }

    rnd_val / norma
}

#[allow(clippy::too_many_arguments)]
pub fn fractal_noise_add_abs_2d<F: Fn(&mut UniformRandomGen, f32, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    noise_func: F,
    octaves: i32,
    freq_falloff: f32,
    lacunarity: f32,
    seed: u32,
) -> f32 {
    let mut rnd_val = 0.0;

    let mut power = 1.0;
    let mut norma = power;

    let mut x = x;
    let mut y = y;

    for _ in 0..octaves {
        rnd_val += noise_func(rng, x, y, seed).abs() * power;
        norma += power;

        power *= freq_falloff;
        x *= lacunarity;
        y *= lacunarity;
    }

    rnd_val / norma
}

#[allow(clippy::too_many_arguments)]
pub fn fractal_noise_add_abs_3d<F: Fn(&mut UniformRandomGen, f32, f32, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
    noise_func: F,
    octaves: i32,
    freq_falloff: f32,
    lacunarity: f32,
    seed: u32,
) -> f32 {
    let mut rnd_val = 0.0;

    let mut power = 1.0;
    let mut norma = power;

    let mut x = x;
    let mut y = y;
    let mut z = z;

    for _ in 0..octaves {
        rnd_val += noise_func(rng, x, y, z, seed).abs() * power;
        norma += power;

        power *= freq_falloff;
        x *= lacunarity;
        y *= lacunarity;
        z *= lacunarity;
    }

    rnd_val / norma
}

#[allow(clippy::too_many_arguments)]
pub fn fractal_noise_add_abs_4d<F: Fn(&mut UniformRandomGen, f32, f32, f32, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
    t: f32,
    noise_func: F,
    octaves: i32,
    freq_falloff: f32,
    lacunarity: f32,
    seed: u32,
) -> f32 {
    let mut rnd_val = 0.0;

    let mut power = 1.0;
    let mut norma = power;

    let mut x = x;
    let mut y = y;
    let mut z = z;
    let mut t = t;

    for _ in 0..octaves {
        rnd_val += noise_func(rng, x, y, z, t, seed).abs() * power;
        norma += power;

        power *= freq_falloff;
        x *= lacunarity;
        y *= lacunarity;
        z *= lacunarity;
        t *= lacunarity;
    }

    rnd_val / norma
}

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

#[allow(clippy::too_many_arguments)]
pub fn fractal_noise_mul_3d<F: Fn(&mut UniformRandomGen, f32, f32, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
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
    let mut z = z;

    for _i in 0..octaves {
        rnd_val *= (noise_func(rng, x, y, z, seed) + offset) * power;
        power *= freq_falloff;
        x *= lacunarity;
        y *= lacunarity;
        z *= lacunarity;
    }

    rnd_val
}

#[allow(clippy::too_many_arguments)]
pub fn fractal_noise_mul_4d<F: Fn(&mut UniformRandomGen, f32, f32, f32, f32, u32) -> f32>(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
    t: f32,
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
    let mut z = z;
    let mut t = t;

    for _i in 0..octaves {
        rnd_val *= (noise_func(rng, x, y, z, t, seed) + offset) * power;
        power *= freq_falloff;
        x *= lacunarity;
        y *= lacunarity;
        z *= lacunarity;
        t *= lacunarity;
    }

    rnd_val
}
