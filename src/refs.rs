//! Implementation for the reference types
// A lot is copied from <https://github.com/uuid-rs/uuid/blob/master/src/adapter/mod.rs>

use crate::{
    std::{fmt, str},
    Yyid,
};

/// The segments of a UUID's [u8; 16] corresponding to each group.
/// Marks indices and needs to be read as pairs.
const BYTE_POSITIONS: [usize; 6] = [0, 4, 6, 8, 10, 16];

/// Where the hyphens need to appear
const HYPHEN_POSITIONS: [usize; 4] = [8, 13, 18, 23];

const LOWER: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

const URN_PREFIX: &[u8; 9] = b"urn:yyid:";

/// Used for formatting a [`Yyid`] as a hyphenated string
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HyphenatedRef<'a>(&'a Yyid);

/// Used for formatting a [`Yyid`] as a simple string
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SimpleRef<'a>(&'a Yyid);

/// Used for formatting a [`Yyid`] as a URN string
/// URN prefix will be: `urn:yyid:`
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UrnRef<'a>(&'a Yyid);

// === generic encoder function ===

#[allow(clippy::needless_range_loop)]
fn encode<'a>(full_buffer: &'a mut [u8], start: usize, yyid: &Yyid, hyphens: bool) -> &'a mut str {
    let len = if hyphens { 36 } else { 32 };

    {
        let buffer = &mut full_buffer[start..start + len];
        let bytes = yyid.as_bytes();
        let hex = &LOWER;

        for group in 0..5 {
            let hyphens_before = if hyphens { group } else { 0 };
            for idx in BYTE_POSITIONS[group]..BYTE_POSITIONS[group + 1] {
                let b = bytes[idx];
                let out_idx = hyphens_before + 2 * idx;

                buffer[out_idx] = hex[(b >> 4) as usize];
                buffer[out_idx + 1] = hex[(b & 0b1111) as usize];
            }

            if group != 4 && hyphens {
                buffer[HYPHEN_POSITIONS[group]] = b'-';
            }
        }
    }

    str::from_utf8_mut(&mut full_buffer[..start + len])
        .expect("found non-ASCII output characters while encoding a UUID")
}

// === impls ===

impl<'a> Yyid {
    /// Creates a [`HyphenatedRef`] from a [`Yyid`]
    #[inline]
    pub const fn to_hyphenated_ref(&self) -> HyphenatedRef<'_> {
        HyphenatedRef::from_yyid_ref(self)
    }

    /// Creates a [`SimpleRef`] from a [`Yyid`]
    #[inline]
    pub const fn to_simple_ref(&self) -> SimpleRef<'_> {
        SimpleRef::from_yyid_ref(self)
    }

    /// Creates a [`UrnRef`] from a [`Yyid`]
    #[inline]
    pub const fn to_urn_ref(&self) -> UrnRef<'_> {
        UrnRef::from_yyid_ref(self)
    }
}

impl<'a> HyphenatedRef<'a> {
    /// Hyphenated string length
    pub const LENGTH: usize = 36;

    /// Wraps a [`Yyid`] into a [`HyphenatedRef`]
    pub const fn from_yyid_ref(yyid: &'a Yyid) -> Self {
        Self(yyid)
    }

    fn encode<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode(buffer, 0, &self.0, true)
    }
}

impl<'a> SimpleRef<'a> {
    /// Simple string length
    pub const LENGTH: usize = 32;

    /// Wraps a [`Yyid`] into a [`SimpleRef`]
    pub const fn from_yyid_ref(yyid: &'a Yyid) -> Self {
        Self(yyid)
    }

    fn encode<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode(buffer, 0, &self.0, false)
    }
}

impl<'a> UrnRef<'a> {
    /// URN string length
    pub const LENGTH: usize = 45;

    /// Wraps a [`Yyid`] into a [`UrnRef`]
    pub const fn from_yyid_ref(yyid: &'a Yyid) -> Self {
        Self(yyid)
    }

    fn encode<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        buffer[..9].copy_from_slice(URN_PREFIX);
        encode(buffer, 9, &self.0, true)
    }
}

// === Formatters ===

macro_rules! impl_adapter_traits {
    ($($T:ident<$($a:lifetime),*>),+) => {$(
        impl<$($a),*> fmt::Display for $T<$($a),*> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(self, f)
            }
        }

        impl<$($a),*> fmt::LowerHex for $T<$($a),*> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.encode(&mut [0; $T::LENGTH]))
            }
        }

        impl_adapter_from!($T<$($a),*>);
    )+}
}

macro_rules! impl_adapter_from {
    ($T:ident<$a:lifetime>) => {
        impl<$a> From<&$a Yyid> for $T<$a> {
            #[inline]
            fn from(f: &$a Yyid) -> Self {
                $T::from_yyid_ref(f)
            }
        }
    };
}

impl_adapter_traits! {
    HyphenatedRef<'a>,
    SimpleRef<'a>,
    UrnRef<'a>
}
