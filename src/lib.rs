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
//! ### Other libraries for Yyid
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
//! - Ruby: <https://github.com/janlelis/yyid.rb>
//! - JavaScript: <https://github.com/janlelis/yyid.js>
//! - Elixir: <https://github.com/janlelis/yyid.ex>
//! - Go: <https://github.com/janlelis/yyid.go>

#![deny(warnings, missing_debug_implementations, missing_docs)]

use {
    core::{
        fmt::{self, Debug, Display, Formatter},
        hash::{Hash, Hasher},
    },
    getrandom::getrandom,
};

/// A 128-bit (16 byte) buffer containing the ID.
pub type YyidBytes = [u8; 16];

/// A yniversally ynique identifier (Yyid).
#[derive(Copy, Clone)]
pub struct Yyid(YyidBytes);

/// Creates a new random Yyid as String
///
/// ### Example
/// ```rust
/// use yyid::yyid_string;
///
/// println!("{}", yyid_string());
/// // => "02e7f0f6-067e-8c92-b25c-12c9180540a9"
/// ```
pub fn yyid_string() -> String {
    Yyid::new().to_hyphenated_string()
}

impl Yyid {
    /// Creates a new random Yyid
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
    /// assert_eq!(nil.as_bytes(), [0u8; 16]);
    /// ```
    pub fn nil() -> Self {
        Yyid([0u8; 16])
    }

    /// Return an array of 16 octets containing the Yyid data
    pub fn as_bytes(&self) -> &'_ [u8] {
        &self.0
    }

    /// Returns a string of hexadecimal digits, separated into groups with a
    /// hyphen.
    ///
    /// Example: `02e7f0f6-067e-8c92-b25c-12c9180540a9`
    pub fn to_hyphenated_string(&self) -> String {
        let b = &self.0;
        format!(
            "{:02x}{:02x}{:02x}{:02x}-\
                 {:02x}{:02x}-\
                 {:02x}{:02x}-\
                 {:02x}{:02x}-\
                 {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
        )
    }

    /// Returns the Yyid as a string of 32 hexadecimal digits
    ///
    /// Example: `2ff0b694960e88a4693a66cff98fc56c`
    pub fn to_simple_string(&self) -> String {
        let b = &self.0;
        format!(
            "{:02x}{:02x}{:02x}{:02x}\
                 {:02x}{:02x}\
                 {:02x}{:02x}\
                 {:02x}{:02x}\
                 {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
        )
    }

    /// Returns the Yyid formatted as a full URN string
    ///
    /// This is the same as the hyphenated format, but with the "urn:yyid:"
    /// prefix.
    ///
    /// Example: `urn:yyid:05f7d6d3-1727-ce2d-6cf2-3b73ad48ff73`
    pub fn to_urn_string(&self) -> String {
        format!("urn:yyid:{}", self.to_string())
    }
}

impl Default for Yyid {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert the Yyid to a hexadecimal-based string representation wrapped in
/// `Yyid()`
/// ### Example
/// ```rust
/// use yyid::Yyid;
///
/// let yyid = Yyid::new();
/// println!("{:?}", yyid);
/// ```

impl Debug for Yyid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Yyid(\"{}\")", self.to_hyphenated_string())
    }
}

/// Convert the Yyid to a hexadecimal-based string representation
///
/// ### Example
/// ```rust
/// use yyid::Yyid;
///
/// let yyid = Yyid::new();
/// println!("{}", yyid);
/// ```
impl Display for Yyid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hyphenated_string())
    }
}

/// Test two Yyids for equality
///
/// Yyids are equal only when they are byte-for-byte identical
///
/// ### Example
/// ```rust
/// use yyid::Yyid;
///
/// let yyid1 = Yyid::new();
/// let yyid2 = Yyid::new();
/// assert_ne!(yyid1, yyid2);
/// ```
impl PartialEq for Yyid {
    fn eq(&self, other: &Yyid) -> bool {
        self.0 == other.0
    }
}

impl Eq for Yyid {}

impl Hash for Yyid {
    fn hash<S: Hasher>(&self, state: &mut S) {
        self.0.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let yyid = Yyid::new();
        let ystr = yyid.to_simple_string();

        assert!(ystr.len() == 32);
    }

    #[test]
    fn test_to_hyphenated_string() {
        let yyid = Yyid::new();
        let ystr = yyid.to_hyphenated_string();

        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_yyid_string() {
        let ystr = yyid_string();

        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_to_simple_string() {
        let yyid = Yyid::new();
        let ystr = yyid.to_simple_string();

        assert!(ystr.len() == 32);
        assert!(ystr.chars().all(|c| c.is_digit(16)));
    }

    #[test]
    fn test_to_urn_string() {
        let yyid = Yyid::new();
        let yurn = yyid.to_urn_string();
        let ystr = &yurn[9..];

        assert!(yurn.starts_with("urn:yyid:"));
        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_to_simple_string_matching() {
        let yyid = Yyid::new();

        let yhyphen = yyid.to_string();
        let ysimple = yyid.to_simple_string();

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
