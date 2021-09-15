use std::f32::consts::PI;

use crate::base83;

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

fn sign(n: f32) -> f32 {
    if n < 0. {
        -1.
    } else {
        1.
    }
}

pub fn sign_pow(val: f32, exp: f32) -> f32 {
    sign(val) * f32::powf(val.abs(), exp)
}

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
    let mut r = 0.;
    let mut g = 0.;
    let mut b = 0.;

    let normalisation =
        match (component_x, component_y) {
            (0, 0) => 1.,
            _ => 2.,
        };

    let bytes_per_row = width * 4;

    for y in 0..height {
        for x in 0..width {
            let basis = f32::cos(PI * component_x as f32 * x as f32 / width as f32)
                * f32::cos(PI * component_y as f32 * y as f32 / height as f32);

            r += basis * srgb_to_linear(u32::from(rgb[(4 * x + y * bytes_per_row) as usize]));
            g += basis * srgb_to_linear(u32::from(rgb[(4 * x + 1 + y * bytes_per_row) as usize]));
            b += basis * srgb_to_linear(u32::from(rgb[(4 * x + 2 + y * bytes_per_row) as usize]));
        }
    }

    let scale =
        normalisation / (width * height) as f32;

    [
        r * scale,
        g * scale,
        b * scale,
    ]
}
