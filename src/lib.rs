//! A pure Rust implementation of [woltapp/blurhash][1].
//!
//! ### Encoding
//!
//! ```
//! use blurhash::encode;
//! use image::{GenericImageView, EncodableLayout};
//!
//! let img = image::open("octocat.png").unwrap();
//! let (width, height) = img.dimensions();
//! let blurhash = encode(4, 3, width, height, img.to_rgba().as_bytes()).unwrap();
//!
//! assert_eq!(blurhash, "LBAdAqof00WCqZj[PDay0.WB}pof");
//! ```
//!
//! ### Decoding
//!
//! ```no_run
//! use blurhash::decode;
//!
//! let pixels = decode("LBAdAqof00WCqZj[PDay0.WB}pof", 50, 50, 1.0);
//! ```
//! [1]: https://github.com/woltapp/blurhash

pub use decode::decode;
pub use encode::encode;
pub use error::Error;

mod alternating_current;
mod base83;
mod direct_current;
mod error;
mod util;
mod encode;
mod decode;

#[cfg(test)]
mod tests {
    use image::{ColorType::Rgba8, save_buffer};
    use image::{EncodableLayout, GenericImageView};

    use super::{decode, encode};

    #[test]
    fn decode_blurhash() {
        let img = image::open("octocat.png").unwrap();
        let (width, height) = img.dimensions();

        let blurhash =
            encode(
                4,
                3,
                width,
                height,
                img
                    .to_rgba8()
                    .as_bytes(),
            ).unwrap();

        let img =
            decode(
                &blurhash,
                width,
                height,
                1.0,
            ).unwrap();

        save_buffer(
            "out.png",
            &img,
            width,
            height,
            Rgba8,
        ).unwrap();

        assert_eq!(img[0..5], [45, 1, 56, 255, 45]);
    }
}
