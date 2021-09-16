use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use super::{alternating_current, base83, direct_current};
use super::Error;
use super::util::multiply_basis_function;

/// Calculates the blurhash for an image using the given x and y component counts.
pub fn encode(
    components_x: u32,
    components_y: u32,
    width: u32,
    height: u32,
    rgba_image: &[u8],
) -> Result<String, Error> {
    if !(1..=9).contains(&components_x)
        || !(1..=9).contains(&components_y)
    {
        return Err(Error::ComponentsOutOfRange);
    }

    let factors: Vec<[f32; 3]> = (0..components_y)
        .into_par_iter()
        .map(
            |y| {
                (0..components_x)
                    .into_par_iter()
                    .map(
                        |x| {
                            multiply_basis_function(
                                x,
                                y,
                                width,
                                height,
                                rgba_image,
                            )
                        }
                    )
                    .collect::<Vec<[f32; 3]>>()
            }
        )
        .flatten()
        .collect();

    let dc = factors[0];
    let ac = &factors[1..];

    let mut blurhash = String::new();

    let size_flag =
        (components_x - 1)
            + (components_y - 1)
            * 9;

    blurhash
        .push_str(
            &base83::encode(
                size_flag,
                1,
            ),
        );

    let maximum_value: f32;

    if !ac.is_empty() {
        let mut actualmaximum_value = 0.0;

        for i in 0..components_y * components_x - 1 {
            actualmaximum_value = f32::max(ac[i as usize][0], actualmaximum_value);
            actualmaximum_value = f32::max(ac[i as usize][1], actualmaximum_value);
            actualmaximum_value = f32::max(ac[i as usize][2], actualmaximum_value);
        }

        let quantised_maximum_value =
            f32::max(
                0.,
                f32::min(
                    82.,
                    f32::floor(
                        actualmaximum_value
                            * 166.
                            - 0.5,
                    ),
                ),
            ) as u32;

        maximum_value = (quantised_maximum_value + 1) as f32 / 166.;

        blurhash.push_str(&base83::encode(quantised_maximum_value, 1));
    } else {
        maximum_value = 1.;

        blurhash.push_str(&base83::encode(0, 1));
    }

    blurhash.push_str(&base83::encode(direct_current::encode(dc), 4));

    for i in 0..components_y * components_x - 1 {
        blurhash.push_str(&base83::encode(
            alternating_current::encode(ac[i as usize], maximum_value),
            2,
        ));
    }

    Ok(blurhash)
}
