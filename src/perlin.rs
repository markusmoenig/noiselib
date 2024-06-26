use crate::prelude::UniformRandomGen;

pub const N_GRAD: i32 = 16;
pub const N_PERM: i32 = 256;

pub static GRAD1: [f32; N_GRAD as usize] = [
    0.3330, 0.3413, 0.4498, 0.5427, 0.6827, 0.8449, 0.9195, 0.9902, -0.2583, -0.3480, -0.4935,
    -0.6358, -0.7276, -0.8047, -0.8377, -0.9997,
];

pub static NOISE_PERM: [i32; N_PERM as usize * 2] = [
    215, 164, 31, 51, 68, 144, 111, 227, 147, 241, 249, 207, 26, 113, 109, 36, 140, 226, 83, 37,
    158, 143, 27, 145, 177, 40, 53, 170, 20, 25, 138, 206, 55, 222, 77, 1, 23, 42, 197, 174, 169,
    0, 132, 136, 86, 98, 242, 160, 75, 218, 54, 120, 202, 126, 61, 231, 41, 119, 214, 50, 18, 172,
    21, 203, 59, 205, 219, 34, 70, 49, 156, 29, 129, 10, 115, 245, 216, 32, 190, 67, 57, 240, 8,
    95, 151, 189, 228, 102, 246, 125, 38, 212, 234, 43, 24, 139, 171, 103, 200, 196, 150, 194, 71,
    97, 161, 105, 157, 73, 5, 199, 233, 225, 148, 191, 85, 159, 121, 130, 162, 116, 60, 166, 195,
    142, 220, 39, 88, 210, 87, 137, 146, 100, 3, 154, 186, 19, 165, 52, 175, 35, 180, 141, 99, 106,
    9, 78, 149, 223, 244, 182, 47, 173, 237, 22, 56, 183, 110, 79, 64, 123, 209, 76, 250, 236, 124,
    178, 16, 185, 184, 181, 211, 255, 248, 66, 7, 224, 81, 153, 91, 15, 193, 187, 239, 28, 213, 84,
    254, 12, 243, 232, 2, 96, 134, 179, 176, 235, 74, 89, 229, 238, 33, 93, 82, 45, 94, 107, 152,
    127, 72, 108, 247, 128, 44, 90, 6, 117, 168, 131, 80, 17, 65, 253, 30, 198, 201, 4, 192, 204,
    221, 133, 112, 188, 118, 13, 92, 14, 217, 251, 58, 101, 114, 11, 69, 208, 122, 48, 63, 135,
    155, 230, 163, 252, 46, 62, 167, 104, 215, 164, 31, 51, 68, 144, 111, 227, 147, 241, 249, 207,
    26, 113, 109, 36, 140, 226, 83, 37, 158, 143, 27, 145, 177, 40, 53, 170, 20, 25, 138, 206, 55,
    222, 77, 1, 23, 42, 197, 174, 169, 0, 132, 136, 86, 98, 242, 160, 75, 218, 54, 120, 202, 126,
    61, 231, 41, 119, 214, 50, 18, 172, 21, 203, 59, 205, 219, 34, 70, 49, 156, 29, 129, 10, 115,
    245, 216, 32, 190, 67, 57, 240, 8, 95, 151, 189, 228, 102, 246, 125, 38, 212, 234, 43, 24, 139,
    171, 103, 200, 196, 150, 194, 71, 97, 161, 105, 157, 73, 5, 199, 233, 225, 148, 191, 85, 159,
    121, 130, 162, 116, 60, 166, 195, 142, 220, 39, 88, 210, 87, 137, 146, 100, 3, 154, 186, 19,
    165, 52, 175, 35, 180, 141, 99, 106, 9, 78, 149, 223, 244, 182, 47, 173, 237, 22, 56, 183, 110,
    79, 64, 123, 209, 76, 250, 236, 124, 178, 16, 185, 184, 181, 211, 255, 248, 66, 7, 224, 81,
    153, 91, 15, 193, 187, 239, 28, 213, 84, 254, 12, 243, 232, 2, 96, 134, 179, 176, 235, 74, 89,
    229, 238, 33, 93, 82, 45, 94, 107, 152, 127, 72, 108, 247, 128, 44, 90, 6, 117, 168, 131, 80,
    17, 65, 253, 30, 198, 201, 4, 192, 204, 221, 133, 112, 188, 118, 13, 92, 14, 217, 251, 58, 101,
    114, 11, 69, 208, 122, 48, 63, 135, 155, 230, 163, 252, 46, 62, 167, 104,
];

fn smooth_func(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

pub fn perlin_noise_1d(_rng: &mut UniformRandomGen, x: f32, seed: u32) -> f32 {
    let mut ix = x.floor() as i32;
    let fx = x - ix as f32;

    ix += NOISE_PERM[(seed & (N_PERM - 1) as u32) as usize];

    let ixp1 = (ix & (N_PERM - 1)) as usize;
    let ixp2 = ((ix / N_PERM) & (N_PERM - 1)) as usize;

    let h1 = NOISE_PERM[NOISE_PERM[ixp1] as usize + ixp2] & 15;
    let h2 = NOISE_PERM[NOISE_PERM[ixp1 + 1] as usize + ixp2] & 15;

    let wx = smooth_func(fx);

    (1.0 - wx) * GRAD1[h1 as usize] + wx * GRAD1[h2 as usize]
}

#[inline(always)]
pub fn grad2(ix: i32, iy: i32, fx: f32, fy: f32) -> f32 {
    let h = NOISE_PERM[NOISE_PERM[ix as usize] as usize + iy as usize] & 7;

    let u = if h < 4 { fx } else { fy };
    let v = if h < 4 { fy } else { fx };

    let u_val = if h & 1 == 0 { 1.8 * u } else { -u };
    let v_val = if h & 1 == 0 { v } else { -1.8 * v };
    u_val + v_val
}

pub fn perlin_noise_2d(_rng: &mut UniformRandomGen, x: f32, y: f32, mut seed: u32) -> f32 {
    let mut ix = x.floor() as i32;
    let mut iy = y.floor() as i32;

    let fx = x - ix as f32;
    let fy = y - iy as f32;

    seed &= N_PERM as u32 - 1;
    ix += NOISE_PERM[seed as usize];
    iy += NOISE_PERM[seed as usize + 1];

    ix &= N_PERM - 1;
    iy &= N_PERM - 1;

    let w00 = grad2(ix, iy, fx, fy);
    let w01 = grad2(ix, iy + 1, fx, fy - 1.0);
    let w10 = grad2(ix + 1, iy, fx - 1.0, fy);
    let w11 = grad2(ix + 1, iy + 1, fx - 1.0, fy - 1.0);

    let wx = smooth_func(fx);
    let wy = smooth_func(fy);

    (1.0 - wx) * ((1.0 - wy) * w00 + wy * w01) + wx * ((1.0 - wy) * w10 + wy * w11)
}

#[inline(always)]
pub fn grad3(ix: i32, iy: i32, iz: i32, fx: f32, fy: f32, fz: f32) -> f32 {
    let h = NOISE_PERM
        [NOISE_PERM[NOISE_PERM[ix as usize] as usize + iy as usize] as usize + iz as usize]
        & 15;
    let u = if h < 8 || h == 12 || h == 13 { fx } else { fy };
    let v = if h < 4 || h == 12 || h == 13 { fy } else { fz };

    ((h & 1) * 2 - 1) as f32 * u + ((h & 2) * 2 - 1) as f32 * v
}

pub fn perlin_noise_3d(_rng: &mut UniformRandomGen, x: f32, y: f32, z: f32, mut seed: u32) -> f32 {
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let iz = z.floor() as i32;

    let fx = x - ix as f32;
    let fy = y - iy as f32;
    let fz = z - iz as f32;

    seed &= N_PERM as u32 - 1;
    let ix = (ix + NOISE_PERM[seed as usize]) & (N_PERM - 1);
    let iy = (iy + NOISE_PERM[seed as usize + 1]) & (N_PERM - 1);
    let iz = (iz + NOISE_PERM[seed as usize + 2]) & (N_PERM - 1);

    let wz = smooth_func(fz);

    let w000 = grad3(ix, iy, iz, fx, fy, fz) * (1.0 - wz);
    let w001 = grad3(ix, iy, iz + 1, fx, fy, fz - 1.0) * wz;
    let w010 = grad3(ix, iy + 1, iz, fx, fy - 1.0, fz) * (1.0 - wz);
    let w011 = grad3(ix, iy + 1, iz + 1, fx, fy - 1.0, fz - 1.0) * wz;
    let w100 = grad3(ix + 1, iy, iz, fx - 1.0, fy, fz) * (1.0 - wz);
    let w101 = grad3(ix + 1, iy, iz + 1, fx - 1.0, fy, fz - 1.0) * wz;
    let w110 = grad3(ix + 1, iy + 1, iz, fx - 1.0, fy - 1.0, fz) * (1.0 - wz);
    let w111 = grad3(ix + 1, iy + 1, iz + 1, fx - 1.0, fy - 1.0, fz - 1.0) * wz;

    let wx = smooth_func(fx);
    let wy = smooth_func(fy);

    (1.0 - wx) * ((1.0 - wy) * (w000 + w001) + wy * (w010 + w011))
        + wx * ((1.0 - wy) * (w100 + w101) + wy * (w110 + w111))
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub fn grad4(ix: i32, iy: i32, iz: i32, it: i32, fx: f32, fy: f32, fz: f32, ft: f32) -> f32 {
    let h = NOISE_PERM[NOISE_PERM
        [NOISE_PERM[NOISE_PERM[ix as usize] as usize + iy as usize] as usize + iz as usize]
        as usize
        + it as usize];
    let mut sum = 0.0;

    if h & 1 != 0 {
        sum += if h & 2 != 0 { fx } else { -fx };
    }
    if h & 4 != 0 {
        sum += if h & 8 != 0 { fy } else { -fy };
    }
    if h & 16 != 0 {
        sum += if h & 32 != 0 { fz } else { -fz };
    }
    if h & 64 != 0 {
        sum += if h & 128 != 0 { ft } else { -ft };
    }

    sum
}

pub fn perlin_noise_4d(
    _rng: &mut UniformRandomGen,
    x: f32,
    y: f32,
    z: f32,
    t: f32,
    mut seed: u32,
) -> f32 {
    let mut ix = x.floor() as i32;
    let mut iy = y.floor() as i32;
    let mut iz = z.floor() as i32;
    let mut it = t.floor() as i32;

    let fx = x - ix as f32;
    let fy = y - iy as f32;
    let fz = z - iz as f32;
    let ft = t - it as f32;

    seed &= N_PERM as u32 - 1;
    ix += NOISE_PERM[seed as usize];
    iy += NOISE_PERM[seed as usize + 1];
    iz += NOISE_PERM[seed as usize + 2];
    it += NOISE_PERM[seed as usize + 3];

    ix &= N_PERM - 1;
    iy &= N_PERM - 1;
    iz &= N_PERM - 1;
    it &= N_PERM - 1;

    let wt = smooth_func(ft);

    let w0000 = grad4(ix, iy, iz, it, fx, fy, fz, ft) * (1.0 - wt);
    let w0001 = grad4(ix, iy, iz, it + 1, fx, fy, fz, ft - 1.0) * wt;
    let w0010 = grad4(ix, iy, iz + 1, it, fx, fy, fz - 1.0, ft) * (1.0 - wt);
    let w0011 = grad4(ix, iy, iz + 1, it + 1, fx, fy, fz - 1.0, ft - 1.0) * wt;
    let w0100 = grad4(ix, iy + 1, iz, it, fx, fy - 1.0, fz, ft) * (1.0 - wt);
    let w0101 = grad4(ix, iy + 1, iz, it + 1, fx, fy - 1.0, fz, ft - 1.0) * wt;
    let w0110 = grad4(ix, iy + 1, iz + 1, it, fx, fy - 1.0, fz - 1.0, ft) * (1.0 - wt);
    let w0111 = grad4(ix, iy + 1, iz + 1, it + 1, fx, fy - 1.0, fz - 1.0, ft - 1.0) * wt;

    let w1000 = grad4(ix + 1, iy, iz, it, fx - 1.0, fy, fz, ft) * (1.0 - wt);
    let w1001 = grad4(ix + 1, iy, iz, it + 1, fx - 1.0, fy, fz, ft - 1.0) * wt;
    let w1010 = grad4(ix + 1, iy, iz + 1, it, fx - 1.0, fy, fz - 1.0, ft) * (1.0 - wt);
    let w1011 = grad4(ix + 1, iy, iz + 1, it + 1, fx - 1.0, fy, fz - 1.0, ft - 1.0) * wt;
    let w1100 = grad4(ix + 1, iy + 1, iz, it, fx - 1.0, fy - 1.0, fz, ft) * (1.0 - wt);
    let w1101 = grad4(ix + 1, iy + 1, iz, it + 1, fx - 1.0, fy - 1.0, fz, ft - 1.0) * wt;
    let w1110 = grad4(ix + 1, iy + 1, iz + 1, it, fx - 1.0, fy - 1.0, fz - 1.0, ft) * (1.0 - wt);
    let w1111 = grad4(
        ix + 1,
        iy + 1,
        iz + 1,
        it + 1,
        fx - 1.0,
        fy - 1.0,
        fz - 1.0,
        ft - 1.0,
    ) * wt;

    let wx = smooth_func(fx);
    let wy = smooth_func(fy);
    let wz = smooth_func(fz);

    (1.0 - wx)
        * ((1.0 - wy) * ((1.0 - wz) * (w0000 + w0001) + wz * (w0010 + w0011))
            + wy * ((1.0 - wz) * (w0100 + w0101) + wz * (w0110 + w0111)))
        + wx * ((1.0 - wy) * ((1.0 - wz) * (w1000 + w1001) + wz * (w1010 + w1011))
            + wy * ((1.0 - wz) * (w1100 + w1101) + wz * (w1110 + w1111)))
}
