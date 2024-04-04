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

/*
const F3: f32 = 1.0 / 3.0;
const G3: f32 = 1.0 / 6.0;

pub fn musgrave_noise_3d(rng: &mut UniformRandomGen, x: f32, y: f32, z: f32, seed: u32) -> f32 {
    let skew = (x + y + z) * F3;
    let mut ix = (x + skew).floor() as u32;
    let mut iy = RT_FLOOR(y + skew);
    let mut iz = RT_FLOOR(z + skew);

    ieeef unskew = (ix + iy + iz) * G3;

    ieeef fx[4], fy[4], fz[4];

    fx[0] = x - (ix - unskew);
    fy[0] = y - (iy - unskew);
    fz[0] = z - (iz - unskew);

    int i[4], j[4], k[4];
    i[0] = j[0] = k[0] = 0;
    i[3] = j[3] = k[3] = 1;

    if (fx[0] >= fy[0])
    {
        if (fy[0] >= fz[0])
        {
            i[1] = i[2] = j[2] = 1;
            j[1] = k[1] = k[2] = 0;
        }
        else if (fx[0] >= fz[0])
        {
            i[1] = i[2] = k[2] = 1;
            j[1] = j[2] = k[1] = 0;
        }
        else
        {
            i[2] = k[1] = k[2] = 1;
            i[1] = j[1] = j[2] = 0;
        }
    }
    else
    {
        if (fy[0] < fz[0])
        {
            j[2] = k[1] = k[2] = 1;
            i[1] = i[2] = j[1] = 0;
        }
        else if (fx[0] < fz[0])
        {
            j[1] = j[2] = k[2] = 1;
            i[1] = i[2] = k[1] = 0;
        }
        else
        {
            i[2] = j[1] = j[2] = 1;
            i[1] = k[1] = k[2] = 0;
        }
    }

    for (int idx = 1; idx < 4; idx++)
    {
        fx[idx] = fx[0] - i[idx] + idx * G3;
        fy[idx] = fy[0] - j[idx] + idx * G3;
        fz[idx] = fz[0] - k[idx] + idx * G3;
    }

    seed &= (N_PERM - 1);
    ix += noise_perm[seed];
    iy += noise_perm[seed + 1];
    iz += noise_perm[seed + 2];

    ix &= (N_PERM - 1);
    iy &= (N_PERM - 1);
    iz &= (N_PERM - 1);

    ieeef sum = 0;

    for (int idx = 0; idx < 4; idx++)
    {
        ieeef t = 0.6f - fx[idx] * fx[idx] - fy[idx] * fy[idx] - fz[idx] * fz[idx];

        if (t > 0)
        {
            t *= t;
            sum += t * t * grad3(ix + i[idx], iy + j[idx], iz + k[idx], fx[idx], fy[idx], fz[idx]);
        }
    }

    return sum * 32.5f;
}
*/
