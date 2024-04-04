use crate::prelude::*;

//
// Simplex noise
//

pub fn simplex_noise_1d(_rng: &mut UniformRandomGen, x: f32, seed: u32) -> f32 {
    let mut ix = x.floor() as i32;
    let fx = x - ix as f32;

    ix += NOISE_PERM[(seed & (N_PERM as u32 - 1)) as usize];

    let ixp1 = ix & (N_PERM - 1);
    let ixp2 = (ix / N_PERM) & (N_PERM - 1);

    let h1 = NOISE_PERM[NOISE_PERM[ixp1 as usize] as usize + ixp2 as usize] & 15;
    let h2 = NOISE_PERM[NOISE_PERM[ixp1 as usize + 1] as usize + ixp2 as usize] & 15;

    let mut t0 = 1.0 - fx * fx;
    t0 *= t0;

    let mut t1 = fx * (2.0 - fx);
    t1 *= t1;

    t0 * t0 * GRAD1[h1 as usize] + t1 * t1 * GRAD1[h2 as usize]
}

const F2: f32 = 0.3660254;
const G2: f32 = 0.211325;

pub fn simplex_noise_2d(_rng: &mut UniformRandomGen, x: f32, y: f32, mut seed: u32) -> f32 {
    let skew = (x + y) * F2;
    let mut ix = (x + skew).floor() as i32;
    let mut iy = (y + skew).floor() as i32;

    let unskew = (ix + iy) as f32 * G2;

    let fx = x - (ix as f32 - unskew);
    let fy = y - (iy as f32 - unskew);

    let off = if fx > fy { 1.0 } else { 0.0 };

    let x1 = fx - off + G2;
    let y1 = fy - (1.0 - off) + G2;
    let x2 = fx - 1.0 + 2.0 * G2;
    let y2 = fy - 1.0 + 2.0 * G2;

    seed &= N_PERM as u32 - 1;
    ix += NOISE_PERM[seed as usize];
    iy += NOISE_PERM[seed as usize + 1];

    ix &= N_PERM - 1;
    iy &= N_PERM - 1;

    let mut sum = 0.0;

    let mut t = 0.5 - fx * fx - fy * fy;

    if t > 0.0 {
        t *= t;
        sum += t * t * grad2(ix, iy, fx, fy);
    }

    t = 0.5 - x1 * x1 - y1 * y1;

    if t > 0.0 {
        t *= t;
        sum += t * t * grad2(ix + off as i32, iy + (1.0 - off) as i32, x1, y1);
    }

    t = 0.5 - x2 * x2 - y2 * y2;

    if t > 0.0 {
        t *= t;
        sum += t * t * grad2(ix + 1, iy + 1, x2, y2);
    }

    sum * 49.5
}

const F3: f32 = 1.0 / 3.0;
const G3: f32 = 1.0 / 6.0;

pub fn simplex_noise_3d(_rng: &mut UniformRandomGen, x: f32, y: f32, z: f32, mut seed: u32) -> f32 {
    let skew = (x + y + z) * F3;
    let mut ix = (x + skew).floor() as i32;
    let mut iy = (y + skew).floor() as i32;
    let mut iz = (z + skew).floor() as i32;

    let unskew = (ix + iy + iz) as f32 * G3;

    let mut fx: [f32; 4] = [0.0; 4];
    let mut fy: [f32; 4] = [0.0; 4];
    let mut fz: [f32; 4] = [0.0; 4];

    fx[0] = x - (ix as f32 - unskew);
    fy[0] = y - (iy as f32 - unskew);
    fz[0] = z - (iz as f32 - unskew);

    let mut i: [i32; 4] = [0; 4];
    let mut j: [i32; 4] = [0; 4];
    let mut k: [i32; 4] = [0; 4];

    i[0] = 0;
    j[0] = 0;
    k[0] = 0;
    i[3] = 1;
    j[3] = 1;
    k[3] = 1;

    if fx[0] >= fy[0] {
        if fy[0] >= fz[0] {
            i[1] = 1;
            i[2] = 1;
            j[2] = 1;
            j[1] = 0;
            k[1] = 0;
            k[2] = 0;
        } else if fx[0] >= fz[0] {
            i[1] = 1;
            i[2] = 1;
            k[2] = 1;
            j[1] = 0;
            j[2] = 0;
            k[1] = 0;
        } else {
            i[2] = 1;
            k[1] = 1;
            k[2] = 1;
            i[1] = 0;
            j[1] = 0;
            j[2] = 0;
        }
    } else if fy[0] < fz[0] {
        j[2] = 1;
        k[1] = 1;
        k[2] = 1;
        i[1] = 0;
        i[2] = 0;
        j[1] = 0;
    } else if fx[0] < fz[0] {
        j[1] = 1;
        j[2] = 1;
        k[2] = 1;
        i[1] = 0;
        i[2] = 0;
        k[1] = 0;
    } else {
        i[2] = 1;
        j[1] = 1;
        j[2] = 1;
        i[1] = 0;
        k[1] = 0;
        k[2] = 0;
    }

    for idx in 1..4 {
        fx[idx] = fx[0] - i[idx] as f32 + idx as f32 * G3;
        fy[idx] = fy[0] - j[idx] as f32 + idx as f32 * G3;
        fz[idx] = fz[0] - k[idx] as f32 + idx as f32 * G3;
    }

    seed &= N_PERM as u32 - 1;
    ix += NOISE_PERM[seed as usize];
    iy += NOISE_PERM[seed as usize + 1];
    iz += NOISE_PERM[seed as usize + 2];

    ix &= N_PERM - 1;
    iy &= N_PERM - 1;
    iz &= N_PERM - 1;

    let mut sum = 0.0;

    for idx in 0..4 {
        let mut t = 0.6 - fx[idx] * fx[idx] - fy[idx] * fy[idx] - fz[idx] * fz[idx];

        if t > 0.0 {
            t *= t;
            sum += t
                * t
                * grad3(
                    ix + i[idx],
                    iy + j[idx],
                    iz + k[idx],
                    fx[idx],
                    fy[idx],
                    fz[idx],
                );
        }
    }

    sum * 32.5
}

static SIMPLEX: [[u8; 4]; 64] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 0, 0, 0],
    [0, 2, 3, 1],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [1, 2, 3, 0],
    [0, 2, 1, 3],
    [0, 0, 0, 0],
    [0, 3, 1, 2],
    [0, 3, 2, 1],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [1, 3, 2, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [1, 2, 0, 3],
    [0, 0, 0, 0],
    [1, 3, 0, 2],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [2, 3, 0, 1],
    [2, 3, 1, 0],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [2, 0, 3, 1],
    [0, 0, 0, 0],
    [2, 1, 3, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [2, 0, 1, 3],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [3, 0, 1, 2],
    [3, 0, 2, 1],
    [0, 0, 0, 0],
    [3, 1, 2, 0],
    [2, 1, 0, 3],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [3, 1, 0, 2],
    [0, 0, 0, 0],
    [3, 2, 0, 1],
    [3, 2, 1, 0],
];

const F4: f32 = 0.309017;
const G4: f32 = 0.1381966;

pub fn simplex_noise_4d(
    _rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
    t: f32,
    mut seed: u32,
) -> f32 {
    let skew = (x + y + z + t) * F4;
    let mut ix = (x + skew).floor() as i32;
    let mut iy = (y + skew).floor() as i32;
    let mut iz = (z + skew).floor() as i32;
    let mut it = (t + skew).floor() as i32;

    let unskew = (ix + iy + iz + it) as f32 * G4;

    let mut fx: [f32; 5] = [0.0; 5];
    let mut fy: [f32; 5] = [0.0; 5];
    let mut fz: [f32; 5] = [0.0; 5];
    let mut ft: [f32; 5] = [0.0; 5];

    fx[0] = x - (ix as f32 - unskew);
    fy[0] = y - (iy as f32 - unskew);
    fz[0] = z - (iz as f32 - unskew);
    ft[0] = t - (it as f32 - unskew);

    let mut i: [i32; 5] = [0; 5];
    let mut j: [i32; 5] = [0; 5];
    let mut k: [i32; 5] = [0; 5];
    let mut l: [i32; 5] = [0; 5];

    i[0] = 0;
    j[0] = 0;
    k[0] = 0;
    l[0] = 0;
    i[4] = 1;
    j[4] = 1;
    k[4] = 1;
    l[4] = 1;

    let c = if fx > fy { 32 } else { 0 }
        | if fx > fz { 16 } else { 0 }
        | if fy > fz { 8 } else { 0 }
        | if fx > ft { 4 } else { 0 }
        | if fy > ft { 2 } else { 0 }
        | if fz > ft { 1 } else { 0 };

    for idx in 1..5 {
        if idx < 4 {
            i[idx] = if SIMPLEX[c][0] as usize >= 4 - idx {
                1
            } else {
                0
            };
            j[idx] = if SIMPLEX[c][1] as usize >= 4 - idx {
                1
            } else {
                0
            };
            k[idx] = if SIMPLEX[c][2] as usize >= 4 - idx {
                1
            } else {
                0
            };
            l[idx] = if SIMPLEX[c][3] as usize >= 4 - idx {
                1
            } else {
                0
            };
        }

        fx[idx] = fx[0] - i[idx] as f32 + idx as f32 * G4;
        fy[idx] = fy[0] - j[idx] as f32 + idx as f32 * G4;
        fz[idx] = fz[0] - k[idx] as f32 + idx as f32 * G4;
        ft[idx] = ft[0] - l[idx] as f32 + idx as f32 * G4;
    }

    seed &= N_PERM as u32 - 1;
    ix += NOISE_PERM[seed as usize];
    iy += NOISE_PERM[seed as usize + 1];
    iz += NOISE_PERM[seed as usize + 2];
    it += NOISE_PERM[seed as usize + 3];

    ix &= N_PERM - 1;
    iy &= N_PERM - 1;
    iz &= N_PERM - 1;
    it &= N_PERM - 1;

    let mut sum = 0.0;

    for idx in 0..5 {
        let mut w =
            0.6 - fx[idx] * fx[idx] - fy[idx] * fy[idx] - fz[idx] * fz[idx] - ft[idx] * ft[idx];

        if w > 0.0 {
            w *= w;
            sum += w
                * w
                * grad4(
                    ix + i[idx],
                    iy + j[idx],
                    iz + k[idx],
                    it + l[idx],
                    fx[idx],
                    fy[idx],
                    fz[idx],
                    ft[idx],
                );
        }
    }

    sum * 23.0
}
