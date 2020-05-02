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
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
#[allow(unused_imports)]
#[macro_use]
extern crate core as std;

use {crate::std::fmt, getrandom::getrandom};

pub mod refs;

/// A 128-bit (16 byte) buffer containing the ID.
pub type Bytes = [u8; 16];

/// A yniversally ynique identifier (Yyid).
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Yyid(Bytes);

impl<'a> Yyid {
    /// Creates a new random YYID
    ///
    /// ### Example
    /// ```rust
    /// use yyid::Yyid;
    ///
    /// let yyid = Yyid::new();
    /// println!("{}", yyid);
    /// // => "02e7f0f6-067e-8c92-b25c-12c9180540a9"
    /// ```
    pub fn new() -> Self {
        let mut bytes = [0u8; 16];
        // TODO: in a next version this should be bubbled up
        getrandom(&mut bytes).expect("getrandom could not safely generate random data");
        Yyid(bytes)
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
        Yyid([0u8; 16])
    }

    /// Tests if the YYID is nil.
    pub fn is_nil(&self) -> bool {
        self.as_bytes().iter().all(|&b| b == 0)
    }

    /// Return an array of 16 octets containing the YYID data
    pub fn as_bytes(&self) -> &Bytes {
        &self.0
    }

    /// Returns a 128bit value containing the YYID data.
    // TODO: Add example once a parse_str or From is implemented
    pub fn to_u128(&self) -> u128 {
        u128::from(self.as_bytes()[0]) << 120
            | u128::from(self.as_bytes()[1]) << 112
            | u128::from(self.as_bytes()[2]) << 104
            | u128::from(self.as_bytes()[3]) << 96
            | u128::from(self.as_bytes()[4]) << 88
            | u128::from(self.as_bytes()[5]) << 80
            | u128::from(self.as_bytes()[6]) << 72
            | u128::from(self.as_bytes()[7]) << 64
            | u128::from(self.as_bytes()[8]) << 56
            | u128::from(self.as_bytes()[9]) << 48
            | u128::from(self.as_bytes()[10]) << 40
            | u128::from(self.as_bytes()[11]) << 32
            | u128::from(self.as_bytes()[12]) << 24
            | u128::from(self.as_bytes()[13]) << 16
            | u128::from(self.as_bytes()[14]) << 8
            | u128::from(self.as_bytes()[15])
    }

    /// Returns a 128bit little-endian value containing the YYID data.
    // TODO: Add example once a parse_str or From is implemented
    pub fn to_u128_le(&self) -> u128 {
        u128::from(self.as_bytes()[0])
            | u128::from(self.as_bytes()[1]) << 8
            | u128::from(self.as_bytes()[2]) << 16
            | u128::from(self.as_bytes()[3]) << 24
            | u128::from(self.as_bytes()[4]) << 32
            | u128::from(self.as_bytes()[5]) << 40
            | u128::from(self.as_bytes()[6]) << 48
            | u128::from(self.as_bytes()[7]) << 56
            | u128::from(self.as_bytes()[8]) << 64
            | u128::from(self.as_bytes()[9]) << 72
            | u128::from(self.as_bytes()[10]) << 80
            | u128::from(self.as_bytes()[11]) << 88
            | u128::from(self.as_bytes()[12]) << 96
            | u128::from(self.as_bytes()[13]) << 104
            | u128::from(self.as_bytes()[14]) << 112
            | u128::from(self.as_bytes()[15]) << 120
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
        fmt::LowerHex::fmt(&self.to_hyphenated_ref(), f)
    }
}

impl fmt::Display for Yyid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.to_hyphenated_ref(), f)
    }
}

impl fmt::LowerHex for Yyid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.to_hyphenated_ref(), f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        std::string::{String, ToString},
        *,
    };

    #[test]
    fn test_new() {
        let yyid = Yyid::new();
        let ystr = yyid.to_string();

        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_to_hyphenated_string() {
        let yyid = Yyid::new();
        let ystr = yyid.to_hyphenated_ref().to_string();

        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_to_simple_string() {
        let yyid = Yyid::new();
        let ystr = yyid.to_simple_ref().to_string();

        assert!(ystr.len() == 32);
        assert!(ystr.chars().all(|c| c.is_digit(16)));
    }

    #[test]
    fn test_to_urn_string() {
        let yyid = Yyid::new();
        let yurn = yyid.to_urn_ref().to_string();
        let ystr = &yurn[9..];

        assert!(yurn.starts_with("urn:yyid:"));
        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_to_simple_string_matching() {
        let yyid = Yyid::new();

        let yhyphen = yyid.to_string();
        let ysimple = yyid.to_simple_ref().to_string();

        let ysimplified = yhyphen.chars().filter(|&c| c != '-').collect::<String>();

        assert!(ysimplified == ysimple);
    }

    #[test]
    fn test_compare() {
        let yyid1 = Yyid::new();
        let yyid2 = Yyid::new();

        assert!(yyid1 == yyid1);
        assert!(yyid2 == yyid2);
        assert!(yyid1 != yyid2);
        assert!(yyid2 != yyid1);
    }

    #[test]
    fn test_as_bytes() {
        let yyid = Yyid::new();
        let ybytes = yyid.as_bytes();

        assert!(ybytes.len() == 16);
        assert!(!ybytes.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_operator_eq() {
        let yyid1 = Yyid::new();
        let yyid2 = yyid1.clone();
        let yyid3 = Yyid::new();

        assert!(yyid1 == yyid1);
        assert!(yyid1 == yyid2);
        assert!(yyid2 == yyid1);

        assert!(yyid1 != yyid3);
        assert!(yyid3 != yyid1);
        assert!(yyid2 != yyid3);
        assert!(yyid3 != yyid2);
    }

    #[test]
    fn test_iterbytes_impl_for_yyid() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let yyid1 = Yyid::new();
        let yyid2 = Yyid::new();
        set.insert(yyid1.clone());
        assert!(set.contains(&yyid1));
        assert!(!set.contains(&yyid2));
    }
}
