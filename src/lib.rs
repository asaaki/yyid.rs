//! Generate YYIDs
//!
//! - GitHub: <https://github.com/asaaki/yyid.rs>
//! - crates.io: <https://crates.io/crates/yyid>
//!
//! They are like UUIDs (v4), but using all the bits.
//! Therefore they are not UUID standard compliant,
//! so use with care or only for internal IDs.
//!
//! ### Example
//!
//! ```rust
//! use yyid::*;
//!
//! println!("{}", Yyid::new());
//! // => "02e7f0f6-067e-8c92-b25c-12c9180540a9"
//! ```
//!
//! ### Other libraries for YYIDs
//!
//! - Ruby: <https://github.com/janlelis/yyid.rb>
//!
//!   ```ruby
//!   # Code here, since it is only a require and a one-liner:
//!   require "securerandom"
//!   "%08x-%04x-%04x-%04x-%04x%08x" %
//!     SecureRandom.random_bytes(16).unpack("NnnnnN")
//!   #=> "37ab3494-7e04-ecf1-b99f-259999a44d16"
//!   ```
//!
//! - JavaScript: <https://github.com/janlelis/yyid.js>
//! - Elixir: <https://github.com/janlelis/yyid.ex>
//! - Go: <https://github.com/janlelis/yyid.go>

#![no_std]
#![deny(warnings, missing_debug_implementations, missing_docs)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

use std::fmt;

pub mod fmts;

#[cfg(feature = "uuid")]
pub mod uuid;

/// A 128-bit (16 byte) buffer containing the ID.
pub type Bytes = [u8; 16];

const ZEROES: Bytes = [0; 16];

/// A yniversally ynique identifier (Yyid).
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Yyid(Bytes);

#[inline]
fn bytes() -> Bytes {
    #[cfg(not(feature = "fast-rng"))]
    {
        let mut bytes = ZEROES;

        getrandom::getrandom(&mut bytes).unwrap_or_else(|err| {
            // NB: getrandom::Error has no source; this is adequate display
            panic!("could not retrieve random bytes for uuid: {}", err)
        });

        bytes
    }

    #[cfg(feature = "fast-rng")]
    {
        rand::random()
    }
}

impl Yyid {
    /// Creates a new random YYID
    ///
    /// ### Example
    /// ```rust
    /// use yyid::Yyid;
    ///
    /// let yyid = Yyid::new();
    /// println!("{}", yyid);
    /// // => "c49b79f5-22d4-dc42-f214-f4209c80d048"
    /// ```
    pub fn new() -> Self {
        Yyid(bytes())
    }

    /// Special case: a "nil" YYID
    ///
    /// ### Example
    /// ```rust
    /// use yyid::Yyid;
    ///
    /// let nil = Yyid::nil();
    /// assert_eq!(
    ///     nil.to_string(),
    ///     "00000000-0000-0000-0000-000000000000"
    /// );
    /// assert_eq!(nil.as_bytes(), &[0u8; 16]);
    /// ```
    pub const fn nil() -> Self {
        Yyid(ZEROES)
    }

    /// Tests if the YYID is nil.
    pub fn is_nil(&self) -> bool {
        // self.as_bytes().iter().all(|&b| b == 0)
        self.0 == ZEROES
    }

    /// Return an owned array of 16 octets containing the YYID data
    pub fn bytes(self) -> Bytes {
        self.0
    }

    /// Return a borrowed array of 16 octets containing the YYID data
    pub fn as_bytes(&self) -> &Bytes {
        &self.0
    }

    /// Consume itself and return a 128bit value containing the YYID data.
    pub fn to_u128(self) -> u128 {
        u128::from_be_bytes(self.0)
    }

    /// Return a 128bit value containing the YYID data.
    pub fn as_u128(&self) -> u128 {
        u128::from_be_bytes(self.0)
    }

    ///  Consume itself and return a 128bit little-endian value containing the YYID data.
    pub fn to_u128_le(self) -> u128 {
        u128::from_le_bytes(self.0)
    }

    /// Return a 128bit little-endian value containing the YYID data.
    pub fn as_u128_le(&self) -> u128 {
        u128::from_le_bytes(self.0)
    }
}

impl Default for Yyid {
    #[inline]
    fn default() -> Self {
        Self::nil()
    }
}

impl fmt::Debug for Yyid {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.as_hyphenated(), f)
    }
}

impl fmt::Display for Yyid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.as_hyphenated(), f)
    }
}

impl fmt::LowerHex for Yyid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.as_hyphenated(), f)
    }
}

impl fmt::UpperHex for Yyid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.as_hyphenated(), f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        std::string::{String, ToString},
        *,
    };
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_new() {
        let yyid = Yyid::new();
        let ystr = yyid.to_string();

        assert_eq!(ystr.len(), 36);
        assert!(ystr.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
    }

    #[test]
    fn test_to_hyphenated_string() {
        let yyid = Yyid::new();
        let ystr = yyid.as_hyphenated().to_string();

        assert_eq!(ystr.len(), 36);
        assert!(ystr.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
    }

    #[test]
    fn test_to_simple_string() {
        let yyid = Yyid::new();
        let ystr = yyid.as_simple().to_string();

        assert_eq!(ystr.len(), 32);
        assert!(ystr.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_to_urn_string() {
        let yyid = Yyid::new();
        let yurn = yyid.as_urn().to_string();

        assert!(yurn.starts_with("urn:yyid:"));
        assert_eq!(yurn.len(), 45);
        assert!(yurn[9..].chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
    }

    #[test]
    fn test_to_braced_string() {
        let yyid = Yyid::new();
        let ybraced = yyid.as_braced().to_string();

        assert!(ybraced.starts_with("{"));
        assert!(ybraced.ends_with("}"));
        assert_eq!(ybraced.len(), 38);
        assert!(ybraced[1..36].chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
    }

    #[test]
    fn test_to_simple_string_matching() {
        let yyid = Yyid::new();

        let yhyphen = yyid.to_string();
        let ysimple = yyid.as_simple().to_string();

        let ysimplified = yhyphen.chars().filter(|&c| c != '-').collect::<String>();

        assert_eq!(ysimplified, ysimple);
    }

    #[test]
    fn test_compare() {
        let yyid1 = Yyid::new();
        let yyid2 = Yyid::new();

        assert_eq!(yyid1, yyid1);
        assert_eq!(yyid2, yyid2);

        assert_ne!(yyid1, yyid2);
        assert_ne!(yyid2, yyid1);
    }

    #[test]
    fn test_as_bytes() {
        let yyid = Yyid::new();
        let ybytes = yyid.as_bytes();

        assert_eq!(ybytes.len(), 16);
        assert!(!ybytes.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_operator_eq() {
        let yyid1 = Yyid::new();
        let yyid2 = yyid1;
        let yyid3 = Yyid::new();

        assert_eq!(yyid1, yyid1);
        assert_eq!(yyid1, yyid2);
        assert_eq!(yyid2, yyid1);

        assert_ne!(yyid1, yyid3);
        assert_ne!(yyid3, yyid1);
        assert_ne!(yyid2, yyid3);
        assert_ne!(yyid3, yyid2);
    }

    #[test]
    fn test_iterbytes_impl_for_yyid() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let yyid1 = Yyid::new();
        let yyid2 = Yyid::new();
        set.insert(yyid1);
        assert!(set.contains(&yyid1));
        assert!(!set.contains(&yyid2));
    }
}
