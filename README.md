# [G60](https://crates.io/crates/g60)
A G60 format (de)encoder for rust.

G60 is an encoding that uses 60 distinct ASCII characters, specifically all letters and digits except for capital I and O.
G60 encodes 8 bytes to 11 characters, increasing the length in bytes by 37.5%, barely more than the 33⅓% for base64, and much less than the 60% for base-32.

G60 was developed by [Galen Huntington](https://github.com/galenhuntington). This is just a Rust implementation of his work. See the whole definition of the format in his [repo](https://github.com/galenhuntington/g60).

[![](https://img.shields.io/crates/v/g60.svg)](https://crates.io/crates/g60)
[![Docs](https://docs.rs/g60/badge.svg)](https://docs.rs/g60)

## Examples

Using slices:

```rust
use g60::{encode, decode};

fn main() {
    let origin = b"Hello, world!";
    let encoded = "Gt4CGFiHehzRzjCF16";

    assert_eq!(encode(origin), encoded);
    assert_eq!(origin, &decode(&encoded).unwrap()[..]);
}
```

or using strings:

```rust
use g60::{encode_str, decode_to_string};

fn main() {
    let origin = "Hello, world!";
    let encoded = "Gt4CGFiHehzRzjCF16";

    assert_eq!(encode_str(origin), encoded);
    assert_eq!(origin, &decode_to_string(encoded).unwrap());
}
```

## License

This project is licensed under MIT.