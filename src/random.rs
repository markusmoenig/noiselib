use crate::prelude::UniformRandomGen;

//
// Random uncorrelated cell noise
//

pub fn random_noise_1d(rng: &mut UniformRandomGen, x: f32, seed: u32) -> f32 {
    let x = x + 0.00137;
    rng.get(x.floor() as u32 * 3290387 + seed) * 2.0 - 1.0
}

pub fn random_noise_2d(rng: &mut UniformRandomGen, x: f32, y: f32, seed: u32) -> f32 {
    let x = x + 0.00137;
    let y = y + 0.00137;
    rng.get(x.floor() as u32 * 3290387 + y.floor() as u32 * 4433105 + seed) * 2.0 - 1.0
}

pub fn random_noise_3d(rng: &mut UniformRandomGen, x: f32, y: f32, z: f32, seed: u32) -> f32 {
    let x = x + 0.00137;
    let y = y + 0.00137;
    let z = z + 0.00137;
    rng.get(
        x.floor() as u32 * 3290387 + y.floor() as u32 * 4433105 + z.floor() as u32 * 6876199 + seed,
    ) * 2.0
        - 1.0
}

pub fn random_noise_4d(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
    t: f32,
    seed: u32,
) -> f32 {
    let x = x + 0.00137;
    let y = y + 0.00137;
    let z = z + 0.00137;
    let t = t + 0.00137;
    rng.get(
        x.floor() as u32 * 3290387
            + y.floor() as u32 * 4433105
            + z.floor() as u32 * 6876199
            + t.floor() as u32 * 9968761
            + seed,
    ) * 2.0
        - 1.0
}

//
// Linearly filtered random noise 1D
//

pub fn random_noise_filtered_1d(rng: &mut UniformRandomGen, x: f32, seed: u32) -> f32 {
    let x = x + 0.00137;
    let xi = x.floor() as i32;
    let xp = 1.0 + xi as f32 - x;

    let xi = xi as u32 * 3290387;

    let val = rng.get(xi + seed) * xp + rng.get(xi + 3290387 + seed) * (1.0 - xp);
    val * 2.0 - 1.0
}

pub fn random_noise_filtered_2d(rng: &mut UniformRandomGen, x: f32, y: f32, seed: u32) -> f32 {
    let x = x + 0.00137;
    let y = y + 0.00137;

    let xi = x.floor() as i32;
    let yi = y.floor() as i32;

    let xp = 1.0 + xi as f32 - x;
    let xm = 1.0 - xp;

    let yp = 1.0 + yi as f32 - y;
    let ym = 1.0 - yp;

    let xi = xi as u32 * 3290387;
    let yi = yi as u32 * 4433105;

    let xi1 = xi + 3290387;
    let yi1 = yi + 4433105;

    let mut val = rng.get(xi + yi + seed) * xp * yp;
    val += rng.get(xi + yi1 + seed) * xp * ym;
    val += rng.get(xi1 + yi + seed) * xm * yp;
    val += rng.get(xi1 + yi1 + seed) * xm * ym;

    val * 2.0 - 1.0
}

// Linearly filtered random noise 3D
pub fn random_noise_filtered_3d(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
    seed: u32,
) -> f32 {
    let x = x + 0.00137;
    let y = y + 0.00137;
    let z = z + 0.00137;

    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let zi = z.floor() as i32;

    let xp = 1.0 + xi as f32 - x;
    let xm = 1.0 - xp;
    let yp = 1.0 + yi as f32 - y;
    let ym = 1.0 - yp;
    let zp = 1.0 + zi as f32 - z;
    let zm = 1.0 - zp;

    let xi = xi as u32 * 3290387;
    let yi = yi as u32 * 4433105;
    let zi = zi as u32 * 6876199;

    let xi1 = xi + 3290387;
    let yi1 = yi + 4433105;
    let zi1 = zi + 6876199;

    let mut val = rng.get(xi + yi + zi + seed) * xp * yp * zp;
    val += rng.get(xi + yi + zi1 + seed) * xp * yp * zm;
    val += rng.get(xi + yi1 + zi + seed) * xp * ym * zp;
    val += rng.get(xi + yi1 + zi1 + seed) * xp * ym * zm;
    val += rng.get(xi1 + yi + zi + seed) * xm * yp * zp;
    val += rng.get(xi1 + yi + zi1 + seed) * xm * yp * zm;
    val += rng.get(xi1 + yi1 + zi + seed) * xm * ym * zp;
    val += rng.get(xi1 + yi1 + zi1 + seed) * xm * ym * zm;

    val * 2.0 - 1.0
}

// Linearly filtered random noise 4D
pub fn random_noise_filtered_4d(
    rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
    t: f32,
    seed: u32,
) -> f32 {
    let x = x + 0.00137;
    let y = y + 0.00137;
    let z = z + 0.00137;
    let t = t + 0.00137;

    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let zi = z.floor() as i32;
    let ti = t.floor() as i32;

    let xp = 1.0 + xi as f32 - x;
    let xm = 1.0 - xp;
    let yp = 1.0 + yi as f32 - y;
    let ym = 1.0 - yp;
    let zp = 1.0 + zi as f32 - z;
    let zm = 1.0 - zp;
    let tp = 1.0 + ti as f32 - t;
    let tm = 1.0 - tp;

    let xi = xi as u32 * 3290387;
    let yi = yi as u32 * 4433105;
    let zi = zi as u32 * 6876199;
    let ti = ti as u32 * 9968761;

    let xi1 = xi + 3290387;
    let yi1 = yi + 4433105;
    let zi1 = zi + 6876199;
    let ti1 = ti + 9968761;

    let mut val = rng.get(xi + yi + zi + ti + seed) * xp * yp * zp * tp;
    val += rng.get(xi + yi + zi + ti1 + seed) * xp * yp * zp * tm;
    val += rng.get(xi + yi + zi1 + ti + seed) * xp * yp * zm * tp;
    val += rng.get(xi + yi + zi1 + ti1 + seed) * xp * yp * zm * tm;
    val += rng.get(xi + yi1 + zi + ti + seed) * xp * ym * zp * tp;
    val += rng.get(xi + yi1 + zi + ti1 + seed) * xp * ym * zp * tm;
    val += rng.get(xi + yi1 + zi1 + ti + seed) * xp * ym * zm * tp;
    val += rng.get(xi + yi1 + zi1 + ti1 + seed) * xp * ym * zm * tm;
    val += rng.get(xi1 + yi + zi + ti + seed) * xm * yp * zp * tp;
    val += rng.get(xi1 + yi + zi + ti1 + seed) * xm * yp * zp * tm;
    val += rng.get(xi1 + yi + zi1 + ti + seed) * xm * yp * zm * tp;
    val += rng.get(xi1 + yi + zi1 + ti1 + seed) * xm * yp * zm * tm;
    val += rng.get(xi1 + yi1 + zi + ti + seed) * xm * ym * zp * tp;
    val += rng.get(xi1 + yi1 + zi + ti1 + seed) * xm * ym * zp * tm;
    val += rng.get(xi1 + yi1 + zi1 + ti + seed) * xm * ym * zm * tp;
    val += rng.get(xi1 + yi1 + zi1 + ti1 + seed) * xm * ym * zm * tm;

    val * 2.0 - 1.0
}
