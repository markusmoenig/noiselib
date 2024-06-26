use crate::prelude::UniformRandomGen;

//
// Worley's F1 and F2 cellular noises
//

// generated by Knuth's algo

static POISSON_COUNT: [i32; 256] = [
    1, 0, 2, 5, 5, 5, 2, 2, 3, 5, 3, 0, 2, 3, 3, 2, 2, 3, 3, 1, 0, 3, 2, 1, 4, 4, 3, 2, 2, 3, 1, 1,
    3, 8, 2, 3, 2, 3, 2, 4, 2, 4, 2, 4, 3, 3, 3, 2, 3, 0, 0, 5, 1, 1, 1, 2, 3, 0, 2, 3, 5, 1, 4, 1,
    3, 3, 2, 2, 6, 6, 1, 2, 4, 4, 5, 1, 3, 2, 2, 4, 1, 2, 1, 3, 2, 1, 2, 1, 4, 3, 5, 4, 1, 2, 0, 1,
    2, 2, 0, 1, 4, 3, 2, 2, 1, 3, 3, 3, 4, 4, 1, 3, 3, 3, 3, 4, 1, 3, 2, 2, 2, 2, 1, 3, 5, 2, 5, 5,
    2, 3, 0, 3, 3, 4, 1, 3, 3, 1, 0, 1, 2, 1, 4, 2, 3, 3, 2, 6, 1, 2, 0, 1, 1, 1, 3, 4, 0, 2, 3, 2,
    1, 1, 1, 3, 5, 1, 5, 2, 0, 2, 0, 2, 1, 3, 3, 3, 1, 3, 3, 2, 1, 1, 3, 1, 1, 0, 0, 2, 2, 2, 7, 2,
    2, 0, 8, 2, 1, 3, 3, 1, 3, 4, 4, 0, 2, 6, 3, 6, 2, 2, 4, 3, 4, 2, 3, 2, 3, 3, 3, 3, 2, 3, 4, 1,
    2, 2, 4, 4, 5, 6, 2, 6, 3, 1, 5, 0, 3, 3, 2, 5, 0, 4, 2, 1, 2, 6, 0, 3, 1, 1, 1, 3, 3, 2, 2, 2,
];

pub fn worley_f1_add_points_2d(
    rng: &mut UniformRandomGen,
    dist2: &mut f32,
    ix: i32,
    iy: i32,
    x: f32,
    y: f32,
    seed: u32,
) {
    let rseed = 702395077u64
        .wrapping_mul(ix as u64)
        .wrapping_add(915488749u64.wrapping_mul(iy as u64))
        .wrapping_add(seed as u64) as u32;
    let n_points = POISSON_COUNT[(rseed >> 24) as usize];

    for i in 0..n_points {
        let dx = rng.get(rseed + i as u32 * 16) + ix as f32 - x;
        let dy = rng.get(rseed + i as u32 * 16 + 1) + iy as f32 - y;

        let d2 = dx * dx + dy * dy;

        if *dist2 > d2 {
            *dist2 = d2;
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn worley_f1_add_points_3d(
    rng: &mut UniformRandomGen,
    dist2: &mut f32,
    ix: i32,
    iy: i32,
    iz: i32,
    x: f32,
    y: f32,
    z: f32,
    seed: u32,
) {
    let rseed = 702395077u64
        .wrapping_mul(ix as u64)
        .wrapping_add(915488749u64.wrapping_mul(iy as u64))
        .wrapping_add(2120969693u64.wrapping_mul(iz as u64))
        .wrapping_add(seed as u64) as u32;
    let n_points = POISSON_COUNT[(rseed >> 24) as usize];

    for i in 0..n_points {
        let dx = rng.get(rseed + i as u32 * 16) + ix as f32 - x;
        let dy = rng.get(rseed + i as u32 * 16 + 1) + iy as f32 - y;
        let dz = rng.get(rseed + i as u32 * 16 + 2) + iz as f32 - z;

        let d2 = dx * dx + dy * dy + dz * dz;

        if *dist2 > d2 {
            *dist2 = d2;
        }
    }
}

#[derive(Clone, Copy)]
struct CellDist2D {
    dist: f32,
    dx: i32,
    dy: i32,
}

pub fn worley_f1_noise_2d(rng: &mut UniformRandomGen, x: f32, y: f32, seed: u32) -> f32 {
    let x = x.rem_euclid(1073741824.0);
    let y = y.rem_euclid(1073741824.0);

    let ix = x.floor() as i32;
    let iy = y.floor() as i32;

    let fx = x - ix as f32;
    let fy = y - iy as f32;

    let mut dist2 = std::f32::MAX;

    worley_f1_add_points_2d(rng, &mut dist2, ix, iy, x, y, seed);

    let mut cells = [CellDist2D {
        dist: 0.0,
        dx: 0,
        dy: 0,
    }; 8];

    let mut cell_idx = 0;

    // left cell
    let dist2_left = fx * fx;
    if dist2_left < dist2 {
        cells[cell_idx].dist = dist2_left;
        cells[cell_idx].dx = -1;
        cells[cell_idx].dy = 0;
        cell_idx += 1;
    }

    // right cell
    let dist2_right = dist2_left - 2.0 * fx + 1.0;
    if dist2_right < dist2 {
        cells[cell_idx].dist = dist2_right;
        cells[cell_idx].dx = 1;
        cells[cell_idx].dy = 0;

        cell_idx += 1;
    }

    // down cell
    let dist2_down = fy * fy;
    if dist2_down < dist2 {
        cells[cell_idx].dist = dist2_down;
        cells[cell_idx].dx = 0;
        cells[cell_idx].dy = -1;

        cell_idx += 1;
    }

    // up cell
    let dist2_up = dist2_down - 2.0 * fy + 1.0;
    if dist2_up < dist2 {
        cells[cell_idx].dist = dist2_up;
        cells[cell_idx].dx = 0;
        cells[cell_idx].dy = 1;

        cell_idx += 1;
    }

    // left-up cell
    if (dist2_up + dist2_left) < dist2 {
        cells[cell_idx].dist = dist2_up + dist2_left;
        cells[cell_idx].dx = -1;
        cells[cell_idx].dy = 1;

        cell_idx += 1;
    }

    // right-up cell
    if (dist2_up + dist2_right) < dist2 {
        cells[cell_idx].dist = dist2_up + dist2_right;
        cells[cell_idx].dx = 1;
        cells[cell_idx].dy = 1;

        cell_idx += 1;
    }

    // left-down cell
    if (dist2_down + dist2_left) < dist2 {
        cells[cell_idx].dist = dist2_down + dist2_left;
        cells[cell_idx].dx = -1;
        cells[cell_idx].dy = -1;

        cell_idx += 1;
    }

    // right-down cell
    if (dist2_down + dist2_right) < dist2 {
        cells[cell_idx].dist = dist2_down + dist2_right;
        cells[cell_idx].dx = 1;
        cells[cell_idx].dy = -1;

        cell_idx += 1;
    }

    // Sorting cells by distance
    cells[..cell_idx].sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

    worley_f1_add_points_2d(
        rng,
        &mut dist2,
        ix + cells[0].dx,
        iy + cells[0].dy,
        x,
        y,
        seed,
    );

    #[allow(clippy::needless_range_loop)]
    for i in 1..cell_idx {
        // If the distance to the closest point in this cell is already larger than the current dist2,
        // further cells won't contain a closer point, so we can break the loop early.
        if cells[i].dist >= dist2 {
            break;
        }

        let new_ix = ix + cells[i].dx;
        let new_iy = iy + cells[i].dy;

        worley_f1_add_points_2d(rng, &mut dist2, new_ix, new_iy, x, y, seed);
    }

    dist2 - 0.4
}
