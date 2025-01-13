# wsq-rs

`wsq-rs` is a Rust library that extends the functionality of the [`image`](https://crates.io/crates/image) crate to support reading and writing WSQ (Wavelet Scalar Quantization) files. This library is primarily an exercise in learning and experimentation, heavily inspired by the [`python-wsq`](https://github.com/idemia/python-wsq) repository.

WSQ is a compression format commonly used for fingerprint images, providing high compression ratios while preserving the fidelity required for biometric applications. The library leverages NIST's NBIS C source code for the core WSQ compression and decompression functionality.

---

## Features

- Decode WSQ files into raw grayscale image data.
- Encode raw grayscale images into WSQ format.
- Integration with the `image` crate, implementing traits like `ImageDecoder`.

---

## Limitations

- **Thread Safety**: The NIST NBIS C source code used for WSQ compression and decompression is not thread-safe. Concurrent calls to the compression or decompression functions may result in undefined behavior.
- **Experimental**: This library is not production-ready and serves as a learning exercise.

---

## Getting Started

### Prerequisites

To build this library, ensure you have the following installed:

- Rust (stable or nightly)
- A C compiler (e.g., GCC or Clang)

### Installation

Add `wsq-rs` to your `Cargo.toml`:

```toml
[dependencies]
wsq-rs = { path = "/path/to/wsq-rs" }
```

### Usage

#### Compress a Raw Grayscale Image

```rust
  use wsq_rs::compress_wsq;

let raw_image = vec![255; 327 * 443]; // Example raw grayscale image data
let wsq_data = compress_wsq(&raw_image, 327, 443, 0.75).expect("Failed to compress WSQ");

println!("WSQ data size: {}", wsq_data.len());
```

#### Decompress a WSQ File

```rust
use wsq_rs::decode_wsq_file;

let decoder = decode_wsq_file("example.wsq").expect("Failed to decode WSQ");

println!("Image dimensions: {}x{}", decoder.width, decoder.height);
```

---

## Testing

Run the tests using the following command:

```sh
cargo test -- --test-threads=1
```

The `--test-threads=1` flag is required due to the thread-safety limitations of the underlying NIST NBIS C code.

---

## Inspiration

This project is heavily inspired by [idemia/python-wsq](https://github.com/idemia/python-wsq), which offers similar functionality for Python.

---

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

The NIST NBIS C source code is distributed under its own license. Refer to their documentation for details.

---
