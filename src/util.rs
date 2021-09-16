use std::f32::consts::PI;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use super::base83;
use super::Error;

/// linear 0.0-1.0 floating point to srgb 0-255 integer conversion.
#[inline(always)]
pub fn linear_to_srgb(value: f32) -> u32 {
    let v = f32::max(0., f32::min(1., value));

    if v <= 0.003_130_8 {
        (v * 12.92 * 255. + 0.5).round() as u32
    } else {
        ((1.055 * f32::powf(v, 1. / 2.4) - 0.055) * 255. + 0.5).round() as u32
    }
}

/// srgb 0-255 integer to linear 0.0-1.0 floating point conversion.
#[inline(always)]
pub fn srgb_to_linear(value: u32) -> f32 {
    let v = value as f32 / 255.;

    if v <= 0.04045 {
        v / 12.92
    } else {
        f32::powf((v + 0.055) / 1.055, 2.4)
    }
}

#[inline(always)]
fn sign(n: f32) -> f32 {
    if n < 0. {
        -1.
    } else {
        1.
    }
}

#[inline(always)]
pub fn sign_pow(val: f32, exp: f32) -> f32 {
    sign(val) * f32::powf(val.abs(), exp)
}

#[inline(always)]
pub fn extract_hash_dimensions(blurhash: &str) -> Result<(usize, usize), Error> {
    if blurhash.len() < 6 {
        return Err(Error::HashTooShort);
    }

    let size_flag = base83::decode(&blurhash[0..1])?;

    let num_y =
        (f32::floor(size_flag as f32 / 9.) + 1.) as usize;

    let num_x =
        (size_flag % 9) + 1;

    let expected = 4 + 2 * num_x * num_y;

    if blurhash.len() != expected {
        return Err(
            Error::LengthMismatch {
                expected,
                actual: blurhash.len(),
            },
        );
    }

    Ok((num_x, num_y))
}

#[inline(always)]
pub fn multiply_basis_function(
    component_x: u32,
    component_y: u32,
    width: u32,
    height: u32,
    rgb: &[u8],
) -> [f32; 3] {
    let normalisation =
        match (component_x, component_y) {
            (0, 0) => 1.,
            _ => 2.,
        };

    let bytes_per_row = width * 4;

    let (mut r, mut g, mut b) =
        (0..height)
            .into_par_iter()
            .map(
                |row| {
                    (0..width)
                        .into_par_iter()
                        .map(
                            |column| {
                                let basis =
                                    f32::cos(PI * component_x as f32 * column as f32 / width as f32)
                                        * f32::cos(PI * component_y as f32 * row as f32 / height as f32);

                                (
                                    basis * srgb_to_linear(u32::from(rgb[(4 * column + row * bytes_per_row) as usize])),
                                    basis * srgb_to_linear(u32::from(rgb[(4 * column + 1 + row * bytes_per_row) as usize])),
                                    basis * srgb_to_linear(u32::from(rgb[(4 * column + 2 + row * bytes_per_row) as usize])),
                                )
                            },
                        )
                        .reduce_with(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2))
                        .unwrap()
                },
            )
            .reduce_with(|a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2))
            .unwrap();

    let scale =
        normalisation / (width * height) as f32;

    [
        r * scale,
        g * scale,
        b * scale,
    ]
}
