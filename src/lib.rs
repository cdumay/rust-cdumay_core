//! cdumay_core is a bundle of tools used by other libraries.
//!
//! ## Quickstart
//!
//! _Cargo.toml_:
//! ```toml
//! [dependencies]
//! cdumay_core = "0.3"
//! serde_json = "1.0"
//! ```
//!
//! _main.rs_:
//! ```rust
//! extern crate cdumay_core;
//! extern crate serde_json;
//!
//! use cdumay_core::{Value, Uuid};
//!
//! fn main() {
//!     let data = r#"
//!         {
//!             "name": "John Doe",
//!             "age": 43
//!         }"#;
//!     let v: Value = serde_json::from_str(data).unwrap();
//!     assert_eq!(v["name"], "John Doe");
//!     assert_eq!(v["age"], 43);
//!     let uuid = Uuid::new_v4();
//!     println!("{}", serde_json::to_string_pretty(&uuid).unwrap());
//! }
extern crate serde_json;
extern crate uuid;

pub use serde_json::Value;
pub use uuid::Uuid;