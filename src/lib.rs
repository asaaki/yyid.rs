//! Generate YYIDs
//!
//! - GitHub: <https://github.com/asaaki/yyid.rs>
//! - crates.io: <https://crates.io/crates/yyid>
//!
//! ### Example
//!
//! #### Rust
//!
//! ```rust
//! use yyid::yyid_string;
//!
//! println!("{}", yyid_string());
//! // => "02e7f0f6-067e-8c92-b25c-12c9180540a9"
//! ```
//!
//! #### C
//!
//! ```c
//! #include "path/to/yyid.rs/include/libyyid.h"
//! const char* my_yyid = yyid_c_string();
//! ```
//!
//! ### Other libraries for YYID
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

#![deny(warnings)]

use {
    getrandom::getrandom,
    libc::c_char,
    std::{
        ffi::CString,
        fmt::{self, Debug, Display, Formatter},
        hash::{Hash, Hasher},
    },
};

pub type YYIDBytes = [u8; 16];

#[derive(Copy, Clone)]
pub struct YYID {
    bytes: YYIDBytes,
}

/// Creates a new random YYID as String
///
/// ### Example
/// ```rust
/// use yyid::yyid_string;
///
/// println!("{}", yyid_string());
/// // => "02e7f0f6-067e-8c92-b25c-12c9180540a9"
/// ```
pub fn yyid_string() -> String {
    YYID::new().to_hyphenated_string()
}

/// Creates a new random YYID as a C-compatible char*
///
/// ### Example
/// ```rust
/// use yyid::yyid_c_string;
/// use std::ffi::CString;
///
/// let yyid_c_string_ptr = yyid_c_string();
/// unsafe {
///     assert_eq!(b'-',  *yyid_c_string_ptr.offset(8) as u8);
///     let _ = CString::from_raw(yyid_c_string_ptr);
/// }
/// ```
#[no_mangle]
pub extern "C" fn yyid_c_string() -> *mut c_char {
    let yyid = YYID::new().to_hyphenated_string();
    let c_yyid = CString::new(yyid).expect("CString::new failed");
    c_yyid.into_raw()
}

impl YYID {
    /// Creates a new random YYID
    ///
    /// ### Example
    /// ```rust
    /// use yyid::YYID;
    ///
    /// let yyid = YYID::new();
    /// println!("{}", yyid);
    /// // => "02e7f0f6-067e-8c92-b25c-12c9180540a9"
    /// ```
    pub fn new() -> YYID {
        let mut bytes = [0u8; 16];
        // TODO: in a next version this should be bubbled up
        getrandom(&mut bytes).expect("getrandom could not safely generate random data");
        YYID { bytes }
    }

    /// Return an array of 16 octets containing the YYID data
    pub fn as_bytes(&self) -> &'_ [u8] {
        &self.bytes
    }

    /// Returns a string of hexadecimal digits, separated into groups with a
    /// hyphen.
    ///
    /// Example: `02e7f0f6-067e-8c92-b25c-12c9180540a9`
    pub fn to_hyphenated_string(&self) -> String {
        let b = &self.bytes;
        format!(
            "{:02x}{:02x}{:02x}{:02x}-\
                 {:02x}{:02x}-\
                 {:02x}{:02x}-\
                 {:02x}{:02x}-\
                 {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
        )
    }

    /// Returns the YYID as a string of 32 hexadecimal digits
    ///
    /// Example: `2ff0b694960e88a4693a66cff98fc56c`
    pub fn to_simple_string(&self) -> String {
        let b = &self.bytes;
        format!(
            "{:02x}{:02x}{:02x}{:02x}\
                 {:02x}{:02x}\
                 {:02x}{:02x}\
                 {:02x}{:02x}\
                 {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
        )
    }

    /// Returns the YYID formatted as a full URN string
    ///
    /// This is the same as the hyphenated format, but with the "urn:yyid:"
    /// prefix.
    ///
    /// Example: `urn:yyid:05f7d6d3-1727-ce2d-6cf2-3b73ad48ff73`
    pub fn to_urn_string(&self) -> String {
        format!("urn:yyid:{}", self.to_string())
    }
}

impl Default for YYID {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert the YYID to a hexadecimal-based string representation wrapped in
/// `YYID()`
/// ### Example
/// ```rust
/// use yyid::YYID;
///
/// let yyid = YYID::new();
/// println!("{:?}", yyid);
/// ```

impl Debug for YYID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "YYID(\"{}\")", self.to_hyphenated_string())
    }
}

/// Convert the YYID to a hexadecimal-based string representation
///
/// ### Example
/// ```rust
/// use yyid::YYID;
///
/// let yyid = YYID::new();
/// println!("{}", yyid);
/// ```
impl Display for YYID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hyphenated_string())
    }
}

/// Test two YYIDs for equality
///
/// YYIDs are equal only when they are byte-for-byte identical
///
/// ### Example
/// ```rust
/// use yyid::YYID;
///
/// let yyid1 = YYID::new();
/// let yyid2 = YYID::new();
/// assert_ne!(yyid1, yyid2);
/// ```
impl PartialEq for YYID {
    fn eq(&self, other: &YYID) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for YYID {}

impl Hash for YYID {
    fn hash<S: Hasher>(&self, state: &mut S) {
        self.bytes.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::yyid_c_string;
    use super::yyid_string;
    use super::YYID;
    use std::ffi::CStr;
    use std::str;

    #[test]
    fn test_new() {
        let yyid = YYID::new();
        let ystr = yyid.to_simple_string();

        assert!(ystr.len() == 32);
    }

    #[test]
    fn test_to_hyphenated_string() {
        let yyid = YYID::new();
        let ystr = yyid.to_hyphenated_string();

        assert!(ystr.len() == 36);
        assert!(ystr.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_exported_yyid_c_string() {
        let yyid_cptr = yyid_c_string();
        let yyid_cstr = unsafe { CStr::from_ptr(yyid_cptr) };
        let yyid = str::from_utf8(yyid_cstr.to_bytes()).unwrap();

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
        assert!(!ybytes.iter().all(|&b| b == 0));
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
