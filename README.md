# cdumay_core ![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue) [![cdumay_core on crates.io](https://img.shields.io/crates/v/cdumay_core)](https://crates.io/crates/cdumay_core) [![cdumay_core on docs.rs](https://docs.rs/cdumay_core/badge.svg)](https://docs.rs/cdumay_core) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/rust-cdumay_core)

cdumay_core is a bundle of tools used by other libraries.

### Quickstart

*Cargo.toml*:

```toml
[dependencies]
cdumay_core = "0.3"
serde_json = "1.0"
```

*main.rs*:

```rust
extern crate cdumay_core;
extern crate serde_json;

use cdumay_core::{Value, Uuid};

fn main() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43
        }"#;
    let v: Value = serde_json::from_str(data).unwrap();
    assert_eq!(v["name"], "John Doe");
    assert_eq!(v["age"], 43);
    let uuid = Uuid::new_v4();
    println!("{}", serde_json::to_string_pretty(&uuid).unwrap());
}
```
