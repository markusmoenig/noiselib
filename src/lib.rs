pub mod fractal;
pub mod musgrave;
pub mod perlin;
pub mod random;
pub mod simplex;
pub mod uniform;

pub mod prelude {
    pub use crate::fractal::*;
    pub use crate::musgrave::*;
    pub use crate::perlin::*;
    pub use crate::random::*;
    pub use crate::simplex::*;
    pub use crate::uniform::UniformRandomGen;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use image::{ImageBuffer, Luma};

    #[test]
    fn generate_random_image() {
        let seed = 1;
        let mut rng = UniformRandomGen::new(seed);

        let width = 256;
        let height = 256;
        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let noise_val = random_noise_2d(&mut rng, x as f32, y as f32, seed);
            let normalized_val = ((noise_val + 1.0) / 2.0 * 255.0) as u8;
            Luma([normalized_val])
        });

        img.save("images/random.png").expect("Failed to save image");
    }
    #[test]
    fn generate_random_filtered_image() {
        let seed = 1;
        let mut rng = UniformRandomGen::new(seed);

        let width = 256;
        let height = 256;
        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let noise_val = random_noise_filtered_2d(&mut rng, x as f32, y as f32, seed);
            let normalized_val = ((noise_val + 1.0) / 2.0 * 255.0) as u8;
            Luma([normalized_val])
        });

        img.save("images/random_filtered.png")
            .expect("Failed to save image");
    }
    #[test]
    fn generate_perlin_image() {
        let seed = 1;
        let mut rng = UniformRandomGen::new(seed);

        let width = 256;
        let height = 256;
        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let noise_val = perlin_noise_2d(
                &mut rng,
                x as f32 / width as f32 * 10.0,
                y as f32 / height as f32 * 10.0,
                seed,
            );
            let normalized_val = ((noise_val + 1.0) / 2.0 * 255.0) as u8;
            Luma([normalized_val])
        });

        img.save("images/perlin.png").expect("Failed to save image");
    }
    #[test]
    fn generate_musgrave_image() {
        let seed = 1;
        let mut rng = UniformRandomGen::new(seed);

        let width = 256;
        let height = 256;
        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let noise_val = musgrave_noise_2d(
                &mut rng,
                x as f32 / width as f32 * 10.0,
                y as f32 / height as f32 * 10.0,
                seed,
            );
            let normalized_val = ((noise_val + 1.0) / 2.0 * 255.0) as u8;
            Luma([normalized_val])
        });

        img.save("images/musgrave.png")
            .expect("Failed to save image");
    }
    #[test]
    fn generate_simplex_image() {
        let seed = 1;
        let mut rng = UniformRandomGen::new(seed);

        let width = 256;
        let height = 256;
        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let noise_val = simplex_noise_2d(
                &mut rng,
                x as f32 / width as f32 * 10.0,
                y as f32 / height as f32 * 10.0,
                seed,
            );
            let normalized_val = ((noise_val + 1.0) / 2.0 * 255.0) as u8;
            Luma([normalized_val])
        });

        img.save("images/simplex.png")
            .expect("Failed to save image");
    }
    #[test]
    fn generate_fractal_image() {
        let octaves = 4;
        let freq_falloff = 0.5;
        let lacunarity = 2.0;
        let seed = 1;
        let offset = 2.5;
        let mut rng = UniformRandomGen::new(seed);

        let width = 256;
        let height = 256;
        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let noise_val = fractal_noise_mul_2d(
                &mut rng,
                x as f32 / width as f32 * 10.0,
                y as f32 / height as f32 * 10.0,
                perlin_noise_2d,
                octaves,
                freq_falloff,
                lacunarity,
                offset,
                seed,
            );
            let normalized_val = ((noise_val + 1.0) / 2.0 * 255.0) as u8;
            Luma([normalized_val])
        });

        img.save("images/fractal_mul.png")
            .expect("Failed to save image");
    }
}
