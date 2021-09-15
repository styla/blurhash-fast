use std::f32::consts::PI;

use super::{alternating_current, base83, direct_current};
use super::Error;
use super::util::extract_hash_dimensions;
use super::util::linear_to_srgb;

/// Decodes the given blurhash to an image of the specified size.
///
/// The punch parameter can be used to de- or increase the contrast of the
/// resulting image.
pub fn decode(blurhash: &str, width: u32, height: u32, punch: f32) -> Result<Vec<u8>, Error> {
    let (num_x, num_y) =
        extract_hash_dimensions(
            blurhash,
        )?;

    let quantised_maximum_value =
        base83::decode(
            &blurhash[1..2],
        )?;

    let maximum_value =
        (quantised_maximum_value + 1) as f32 / 166.;

    let mut colors =
        vec![[0.; 3]; num_x * num_y];

    for i in 0..colors.len() {
        if i == 0 {
            let value =
                base83::decode(
                    &blurhash[2..6],
                )?;

            colors[i as usize] =
                direct_current::decode(
                    value as u32,
                );

            continue;
        }

        let value =
            base83::decode(
                &blurhash[4 + i * 2..6 + i * 2]
            )?;

        colors[i as usize] =
            alternating_current::decode(
                value as u32,
                maximum_value * punch,
            );
    }

    let bytes_per_row = width * 4;
    let mut pixels = vec![0; (bytes_per_row * height) as usize];

    for y in 0..height {
        for x in 0..width {
            let mut pixel = [0.; 3];

            for j in 0..num_y {
                for i in 0..num_x {
                    let basis =
                        f32::cos((PI * x as f32 * i as f32) / width as f32)
                            * f32::cos((PI * y as f32 * j as f32) / height as f32);

                    let color =
                        &colors[i + j * num_x as usize];

                    pixel[0] += color[0] * basis;
                    pixel[1] += color[1] * basis;
                    pixel[2] += color[2] * basis;
                }
            }

            let int_r = linear_to_srgb(pixel[0]);
            let int_g = linear_to_srgb(pixel[1]);
            let int_b = linear_to_srgb(pixel[2]);

            pixels[(4 * x + y * bytes_per_row) as usize] = int_r as u8;
            pixels[(4 * x + 1 + y * bytes_per_row) as usize] = int_g as u8;
            pixels[(4 * x + 2 + y * bytes_per_row) as usize] = int_b as u8;
            pixels[(4 * x + 3 + y * bytes_per_row) as usize] = 255u8;
        }
    }
    Ok(pixels)
}
