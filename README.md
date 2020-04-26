# YYID generator in Rust 

[![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/asaaki/yyid.rs/Rust/master)](https://github.com/asaaki/yyid.rs/actions?query=workflow%3ARust)  [![yyid on crates.io](https://img.shields.io/crates/v/yyid.svg)](https://crates.io/crates/yyid) [![yyid documentation](https://docs.rs/yyid/badge.svg)](https://docs.rs/yyid/)

Generates random tokens that look like [type 4 UUIDs](https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_.28random.29): `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`

In contrast to [RFC 4122](https://tools.ietf.org/rfc/rfc4122.txt), it uses all digits (128bit).

Source of randomness: [getrandom](https://crates.io/crates/getrandom)

## Examples

### Rust

Add `yyid = "*"` to your dependencies section in `Cargo.toml`.

```rust
use yyid::*;

fn main() {
    println!("{}", Yyid::new());
    // => "02e7f0f6-067e-8c92-b25c-12c9180540a9"
}
```

## Documentation

<https://docs.rs/yyid/>

## Notes

* The current implementation is derived from the [uuid](http://doc.rust-lang.org/uuid/uuid/index.html) crate
* It does not implement everything (no parsing, only generation of tokens)
* Functionality will also be reduced down to the reference implementations (see _Also Available As_)
* The default format is with hyphens (more human readable by default, only 4 bytes extra)
* For simplicity only the *Ref types are implemented (no owning structures)

## Also Available As 

- YYID for [Ruby](https://github.com/janlelis/yyid.rb)
- YYID for [JavaScript](https://github.com/janlelis/yyid.js)
- YYID for [Elixir](https://github.com/janlelis/yyid.ex)
- YYID for [Go](https://github.com/janlelis/yyid.go)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
