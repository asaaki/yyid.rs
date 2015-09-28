//! Generate YYIDs
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

// TODO: #![doc(html_root_url = "https://<some-url-to>/yyid")]

#![cfg_attr(test, deny(warnings))]

extern crate rand;

use std::fmt;
use std::hash;
use std::iter::repeat;
use std::mem::transmute_copy;
use rand::Rng;

pub type YYIDBytes = [u8; 16];

#[derive(Copy, Clone)]
pub struct YYID {
    bytes: YYIDBytes
}

impl hash::Hash for YYID {
    fn hash<S: hash::Hasher>(&self, state: &mut S) {
        self.bytes.hash(state)
    }
}

#[derive(Copy, Clone)]
struct YYIDSections {
    section1:  u32,
    section2:  u16,
    section3:  u16,
    section4:  u16,
    section5a: u16,
    section5b: u32,
}

impl YYID {
    // Creates a new random YYID
    pub fn new() -> YYID {
        let ybytes = rand::thread_rng().gen_iter::<u8>().take(16).collect::<Vec<_>>();
        let mut yyid = YYID{ bytes: [0; 16] };
        copy_memory(&mut yyid.bytes, &ybytes);
        yyid
    }

    // Return an array of 16 octets containing the YYID data
    pub fn as_bytes<'a>(&'a self) -> &'a [u8] { &self.bytes }

    // Returns the YYID as a string of 32 hexadecimal digits
    //
    // Example: `2ff0b694960e88a4693a66cff98fc56c`
    pub fn to_simple_string(&self) -> String {
        let mut ystr = repeat(0u8).take(32).collect::<Vec<_>>();
        for i in 0..16 {
            let digit = format!("{:02x}", self.bytes[i] as usize);
            ystr[i*2+0] = digit.as_bytes()[0];
            ystr[i*2+1] = digit.as_bytes()[1];
        }
        String::from_utf8(ystr).unwrap()
    }

    // Returns a string of hexadecimal digits, separated into groups with a hyphen.
    //
    // Example: `02e7f0f6-067e-8c92-b25c-12c9180540a9`
    pub fn to_hyphenated_string(&self) -> String {
        let mut ys: YYIDSections;
        unsafe {
            ys = transmute_copy(&self.bytes);
        }
        ys.section1  = ys.section1.to_be();
        ys.section2  = ys.section2.to_be();
        ys.section3  = ys.section3.to_be();
        ys.section4  = ys.section4.to_be();
        ys.section5a = ys.section5a.to_be();
        ys.section5b = ys.section5b.to_be();
        let ystr = format!("{:08x}-{:04x}-{:04x}-{:04x}-{:04x}{:08x}",
            ys.section1,
            ys.section2, ys.section3, ys.section4,
            ys.section5a, ys.section5b);
        ystr
    }

    /// Returns the YYID formatted as a full URN string
    ///
    /// This is the same as the hyphenated format, but with the "urn:yyid:" prefix.
    ///
    /// Example: `urn:yyid:05f7d6d3-1727-ce2d-6cf2-3b73ad48ff73`
    pub fn to_urn_string(&self) -> String {
        format!("urn:yyid:{}", self.to_hyphenated_string())
    }
}

fn copy_memory(dst: &mut [u8], src: &[u8]) {
    for (slot, val) in dst.iter_mut().zip(src.iter()) {
        *slot = *val;
    }
}

/// Convert the YYID to a hexadecimal-based string representation wrapped in `YYID()`
impl fmt::Debug for YYID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "YYID(\"{}\")", self.to_hyphenated_string())
    }
}

/// Convert the YYID to a hexadecimal-based string representation
impl fmt::Display for YYID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hyphenated_string())
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
        let ybytes = rng.gen_iter::<u8>().take(16).collect::<Vec<_>>();
        let mut yyid = YYID{ bytes: [0; 16] };
        copy_memory(&mut yyid.bytes, &ybytes);
        yyid
    }
}

#[cfg(test)]
mod tests {
    use super::YYID;
    use rand;

    #[test]
    fn test_new() {
        let yyid = YYID::new();
        let ystr = yyid.to_simple_string();

        assert!(ystr.len() == 32);
    }

    #[test]
    fn test_to_simple_string() {
        let yyid = YYID::new();
        let ystr = yyid.to_simple_string();

        assert!(ystr.len() == 32);
        assert!(ystr.chars().all(|c| c.is_digit(16)));
    }

    #[test]
    fn test_to_string() {
        let yyid = YYID::new();
        let ystr = yyid.to_string();

        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_to_hyphenated_string() {
        let yyid = YYID::new();
        let ystr = yyid.to_hyphenated_string();

        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
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

        let yhyphen = yyid.to_hyphenated_string();
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
    fn test_iterbytes_impl_for_uuid() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let yyid1 = YYID::new();
        let yyid2 = YYID::new();
        set.insert(yyid1.clone());
        assert!(set.contains(&yyid1));
        assert!(!set.contains(&yyid2));
    }
}
