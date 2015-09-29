//! Generate YYIDs
//!
//! - GitHub: <https://github.com/asaaki/yyid.rs>
//! - crates.io: <https://crates.io/crates/yyid>
//!
//! Based on following micro libraries:
//!
//! - Ruby: <https://github.com/janlelis/yyid.rb>
//!
//!   ```ruby
//!   # Code here, since it is only a require and a one-liner:
//!   require "securerandom"
//!   "%08x-%04x-%04x-%04x-%04x%08x" % SecureRandom.random_bytes(16).unpack("NnnnnN")
//!   #=> "37ab3494-7e04-ecf1-b99f-259999a44d16"
//!   ```
//!
//! - JavaScript: <https://github.com/janlelis/yyid.js>
//! - Elixir: <https://github.com/janlelis/yyid.ex>
//! - Go: <https://github.com/janlelis/yyid.go>
//!
//! TODO: Reduce/simplify code to meet the API of the other libraries (a hyphenated string only)

// NOTE: Most of this code is currently based on uuid crate
//       (<https://github.com/rust-lang-nursery/uuid>).

#![doc(html_root_url = "http://asaaki.github.io/yyid.rs/yyid/index.html")]

#![cfg_attr(test, deny(warnings))]

extern crate libc;
extern crate rand;

use std::fmt;
use std::hash;
use std::ffi::CString;
use rand::Rng;

pub type YYIDBytes = [u8; 16];

#[derive(Copy, Clone)]
pub struct YYID {
    bytes: YYIDBytes
}

/// Creates a new random YYID as String
///
/// See: [YYID's to_string](../yyid/struct.YYID.html#method.to_string)
pub fn yyid_string() -> String {
    YYID::new().to_string()
}

/// Creates a new random YYID as a C-compatible char*
#[no_mangle]
pub extern "C" fn yyid_c_string() -> *const i8 {
    let yyid   = yyid_string();
    let c_yyid = CString::new(yyid).unwrap();
    c_yyid.as_ptr()
}

impl YYID {
    /// Creates a new random YYID
    pub fn new() -> YYID {
        let mut ybytes = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut ybytes);
        YYID{ bytes: ybytes }
    }

    /// Return an array of 16 octets containing the YYID data
    pub fn as_bytes<'a>(&'a self) -> &'a [u8] {
        &self.bytes
    }

    /// Returns a string of hexadecimal digits, separated into groups with a hyphen.
    ///
    /// Example: `02e7f0f6-067e-8c92-b25c-12c9180540a9`
    pub fn to_string(&self) -> String {
        let b = &self.bytes;
        format!("{:02x}{:02x}{:02x}{:02x}-\
                 {:02x}{:02x}-\
                 {:02x}{:02x}-\
                 {:02x}{:02x}-\
                 {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                 b[0], b[1], b[2], b[3],
                 b[4], b[5],
                 b[6], b[7],
                 b[8], b[9],
                 b[10], b[11], b[12], b[13], b[14], b[15])
    }

    /// Returns the YYID as a string of 32 hexadecimal digits
    ///
    /// Example: `2ff0b694960e88a4693a66cff98fc56c`
    pub fn to_simple_string(&self) -> String {
        let b = &self.bytes;
        format!("{:02x}{:02x}{:02x}{:02x}\
                 {:02x}{:02x}\
                 {:02x}{:02x}\
                 {:02x}{:02x}\
                 {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                 b[0], b[1], b[2], b[3],
                 b[4], b[5],
                 b[6], b[7],
                 b[8], b[9],
                 b[10], b[11], b[12], b[13], b[14], b[15])
    }

    /// Returns the YYID formatted as a full URN string
    ///
    /// This is the same as the hyphenated format, but with the "urn:yyid:" prefix.
    ///
    /// Example: `urn:yyid:05f7d6d3-1727-ce2d-6cf2-3b73ad48ff73`
    pub fn to_urn_string(&self) -> String {
        format!("urn:yyid:{}", self.to_string())
    }
}

/// Convert the YYID to a hexadecimal-based string representation wrapped in `YYID()`
impl fmt::Debug for YYID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "YYID(\"{}\")", self.to_string())
    }
}

/// Convert the YYID to a hexadecimal-based string representation
impl fmt::Display for YYID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Test two YYIDs for equality
///
/// YYIDs are equal only when they are byte-for-byte identical
impl PartialEq for YYID {
    fn eq(&self, other: &YYID) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for YYID {}

/// Generates a random instance of YYID (V4 conformant)
impl rand::Rand for YYID {
    #[inline]
    fn rand<R: rand::Rng>(rng: &mut R) -> YYID {
        let mut ybytes = [0u8; 16];
        rng.fill_bytes(&mut ybytes);
        YYID{ bytes: ybytes }
    }
}

impl hash::Hash for YYID {
    fn hash<S: hash::Hasher>(&self, state: &mut S) {
        self.bytes.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::YYID;
    use super::yyid_string;
    use super::yyid_c_string;
    use std::ffi::CStr;
    use std::str;
    use rand;

    #[test]
    fn test_new() {
        let yyid = YYID::new();
        let ystr = yyid.to_simple_string();

        assert!(ystr.len() == 32);
    }

    #[test]
    fn test_to_string() {
        let yyid = YYID::new();
        let ystr = yyid.to_string();

        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_exported_yyid_c_string() {
        let yyid_chars = yyid_c_string(); // *const i8|c_char (pointer)
        let c_yyid     = unsafe { CStr::from_ptr(yyid_chars) }; // &CStr
        let yyid       = str::from_utf8(c_yyid.to_bytes()).unwrap();
        //               -(to_bytes)-> &[u8] -(str::from_utf8)-> Result<&str, Utf8Error> -(unwrap)-> &str

        assert!(yyid.len() == 36);
        assert!(yyid.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_yyid_string() {
        let ystr = yyid_string();

        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_to_simple_string() {
        let yyid = YYID::new();
        let ystr = yyid.to_simple_string();

        assert!(ystr.len() == 32);
        assert!(ystr.chars().all(|c| c.is_digit(16)));
    }

    #[test]
    fn test_to_urn_string() {
        let yyid = YYID::new();
        let yurn = yyid.to_urn_string();
        let ystr = &yurn[9..];

        assert!(yurn.starts_with("urn:yyid:"));
        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_to_simple_string_matching() {
        let yyid = YYID::new();

        let yhyphen = yyid.to_string();
        let ysimple = yyid.to_simple_string();

        let ysimplified = yhyphen.chars().filter(|&c| c != '-').collect::<String>();

        assert!(ysimplified == ysimple);
    }

    #[test]
    fn test_compare() {
        let yyid1 = YYID::new();
        let yyid2 = YYID::new();

        assert!(yyid1 == yyid1);
        assert!(yyid2 == yyid2);
        assert!(yyid1 != yyid2);
        assert!(yyid2 != yyid1);
    }

    #[test]
    fn test_as_bytes() {
        let yyid = YYID::new();
        let ybytes = yyid.as_bytes();

        assert!(ybytes.len() == 16);
        assert!(! ybytes.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_operator_eq() {
        let yyid1 = YYID::new();
        let yyid2 = yyid1.clone();
        let yyid3 = YYID::new();

        assert!(yyid1 == yyid1);
        assert!(yyid1 == yyid2);
        assert!(yyid2 == yyid1);

        assert!(yyid1 != yyid3);
        assert!(yyid3 != yyid1);
        assert!(yyid2 != yyid3);
        assert!(yyid3 != yyid2);
    }

    #[test]
    fn test_rand_rand() {
        let mut rng = rand::thread_rng();
        let yyid: YYID = rand::Rand::rand(&mut rng);
        let ybytes = yyid.as_bytes();

        assert!(ybytes.len() == 16);
        assert!(! ybytes.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_iterbytes_impl_for_yyid() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let yyid1 = YYID::new();
        let yyid2 = YYID::new();
        set.insert(yyid1.clone());
        assert!(set.contains(&yyid1));
        assert!(!set.contains(&yyid2));
    }
}
