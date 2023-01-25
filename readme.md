<h1 align="center">blurhash-fast</h1>

<h5 align="center">A pure Rust implementation of <a href="https://github.com/woltapp/blurhash">Blurhash</a> using rayon to achieve improved performance.</h5>

<div align="center">
  <a href="https://crates.io/crates/blurhash-fast">
    crates.io
  </a>
  —
  <a href="https://github.com/styla/blurhash-fast">
    Github
  </a>
</div>

<br />

```shell script
$ cargo add blurhash-fast
```

Blurhash is an algorithm that encodes an image into a short string. When you decode the string back into an image, you get a gradient of colors that represent the original image.

### Encoding
```rust
use blurhash::encode;
use image::GenericImageView;

fn main() {
  let img = image::open("image.png").unwrap();

  let (width, height) = img.dimensions();

  let blurhash =
      encode(
          4, 3,
          width, height,
          &img.to_rgba8(),
      );
}
```

### Decoding
```rust
use blurhash::decode;

let pixels =
    decode(
        "LBAdAqof00WCqZj[PDay0.WB}pof",
        50,
        50,
        1.0,
    );
```

## Notes

This project is based on <a href="https://github.com/Raincal/blurhash-rs">blurhash-rs</a>.

## Licence

MIT License

```
Copyright (c) 2019 Raincal
Copyright (c) 2021 Styla GmbH

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
