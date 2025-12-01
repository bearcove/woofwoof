# woofwoof

WOFF2 font compression and decompression for Rust.

Uses Google's [woff2](https://github.com/google/woff2) C++ library for font-specific transforms, with compression provided by the pure Rust [brotli](https://crates.io/crates/brotli) crate.

## Usage

```rust
use woofwoof::{compress, decompress};

// Compress TTF/OTF to WOFF2
let ttf_data = std::fs::read("font.ttf")?;
let woff2_data = compress(&ttf_data, "", 8, true)
    .expect("compression failed");

// Decompress WOFF2 back to TTF/OTF
let roundtripped = decompress(&woff2_data)
    .expect("decompression failed");
```

## API

### `compress(data, metadata, quality, transform) -> Option<Vec<u8>>`

Compress a TTF/OTF font to WOFF2 format.

- `data`: The TTF or OTF font data
- `metadata`: Optional extended metadata (XML string, usually empty `""`)
- `quality`: Brotli compression quality (0-11, recommended: 8-11)
- `transform`: Whether to apply font-specific transforms (recommended: `true`)

### `decompress(data) -> Option<Vec<u8>>`

Decompress a WOFF2 font back to TTF/OTF format.

## Why "woofwoof"?

Because it does WOFF2. Get it? WOFF... woof... woofwoof?

## Architecture

```
Rust API (compress/decompress)
         │
         ▼
    C wrapper (extern "C")
         │
         ▼
    woff2 C++ (font transforms)
         │
         ▼
    Rust brotli shim (#[no_mangle])
         │
         ▼
    brotli crate (pure Rust)
```

The C++ woff2 library handles the complex font table transformations that make WOFF2 compress so well. Instead of linking to C brotli, we provide stub headers and implement the brotli functions in Rust, forwarding to the pure Rust brotli crate.

## Sponsors

Thanks to all individual sponsors:

<p> <a href="https://github.com/sponsors/fasterthanlime">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://github.com/fasterthanlime/woofwoof/raw/main/static/sponsors/github-dark.svg">
<img src="https://github.com/fasterthanlime/woofwoof/raw/main/static/sponsors/github-light.svg" height="40" alt="GitHub Sponsors">
</picture>
</a> <a href="https://patreon.com/fasterthanlime">
    <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/fasterthanlime/woofwoof/raw/main/static/sponsors/patreon-dark.svg">
    <img src="https://github.com/fasterthanlime/woofwoof/raw/main/static/sponsors/patreon-light.svg" height="40" alt="Patreon">
    </picture>
</a> </p>

...along with corporate sponsors:

<p> <a href="https://zed.dev">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://github.com/fasterthanlime/woofwoof/raw/main/static/sponsors/zed-dark.svg">
<img src="https://github.com/fasterthanlime/woofwoof/raw/main/static/sponsors/zed-light.svg" height="40" alt="Zed">
</picture>
</a> <a href="https://depot.dev?utm_source=woofwoof">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://github.com/fasterthanlime/woofwoof/raw/main/static/sponsors/depot-dark.svg">
<img src="https://github.com/fasterthanlime/woofwoof/raw/main/static/sponsors/depot-light.svg" height="40" alt="Depot">
</picture>
</a> </p>

...without whom this work could not exist.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

The bundled woff2 library is licensed under the MIT license.
