# YYID generator in Rust

Generates random tokens that look like [type 4 UUIDs](https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_.28random.29): `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`

In contrast to [RFC 4122](https://tools.ietf.org/rfc/rfc4122.txt), it uses all digits (128bit).

Source of randomness: [rand](https://doc.rust-lang.org/rand/rand/index.html)

## Example

```rust
extern crate yyid;

use yyid::YYID;

fn main() {
    let yyid = YYID::new();
    println!("{}", yyid.to_hyphenated_string());
    // => "02e7f0f6-067e-8c92-b25c-12c9180540a9"
}
```

The current implementation is derived from the [uuid](http://doc.rust-lang.org/uuid/uuid/index.html) crate.

It does not implement everything (no parsing, only generation of tokens).

Functionality will also be reduced down to the reference implementations (see below).

## Also Available

- YYID for [Ruby](https://github.com/janlelis/yyid.rb)
- YYID for [JavaScript](https://github.com/janlelis/yyid.js)
- YYID for [Elixir](https://github.com/janlelis/yyid.ex)
- YYID for [Go](https://github.com/janlelis/yyid.go)
