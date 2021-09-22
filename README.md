# [G60](https://crates.io/crates/g60)
A G60 format (de)encoder for rust.

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