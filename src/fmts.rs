//! Implementation for the reference types
// A lot is copied from <https://github.com/uuid-rs/uuid/blob/master/src/adapter/mod.rs>

use crate::{
    std::{borrow::Borrow, fmt, ptr, str},
    Yyid,
};

const LOWER: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

const UPPER: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
];

const URN_PREFIX: &[u8; 9] = b"urn:yyid:";

/// Format a [`Yyid`] as a hyphenated string, like
/// `c49b79f5-22d4-dc42-f214-f4209c80d048`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Hyphenated(Yyid);

/// Format a [`Yyid`] as a simple string, like
/// `c49b79f522d4dc42f214f4209c80d048`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Simple(Yyid);

/// Format a [`Yyid`] as a URN string, like
/// `urn:yyid:c49b79f5-22d4-dc42-f214-f4209c80d048`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Urn(Yyid);

/// Format a [`Yyid`] as a braced hyphenated string, like
/// `{c49b79f5-22d4-dc42-f214-f4209c80d048}`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Braced(Yyid);

#[inline]
const fn format_simple(src: &[u8; 16], upper: bool) -> [u8; 32] {
    let lut = if upper { &UPPER } else { &LOWER };
    let mut dst = [0; 32];
    let mut i = 0;
    while i < 16 {
        let x = src[i];
        dst[i * 2] = lut[(x >> 4) as usize];
        dst[i * 2 + 1] = lut[(x & 0x0f) as usize];
        i += 1;
    }
    dst
}

#[inline]
const fn format_hyphenated(src: &[u8; 16], upper: bool) -> [u8; 36] {
    let lut = if upper { &UPPER } else { &LOWER };
    let groups = [(0, 8), (9, 13), (14, 18), (19, 23), (24, 36)];
    let mut dst = [0; 36];

    let mut group_idx = 0;
    let mut i = 0;
    while group_idx < 5 {
        let (start, end) = groups[group_idx];
        let mut j = start;
        while j < end {
            let x = src[i];
            i += 1;

            dst[j] = lut[(x >> 4) as usize];
            dst[j + 1] = lut[(x & 0x0f) as usize];
            j += 2;
        }
        if group_idx < 4 {
            dst[end] = b'-';
        }
        group_idx += 1;
    }
    dst
}

#[inline]
fn encode_simple<'b>(src: &[u8; 16], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
    let buf = &mut buffer[..Simple::LENGTH];
    let dst = buf.as_mut_ptr();

    // SAFETY: `buf` is guaranteed to be at least `LEN` bytes
    // SAFETY: The encoded buffer is ASCII encoded
    unsafe {
        ptr::write(dst.cast(), format_simple(src, upper));
        str::from_utf8_unchecked_mut(buf)
    }
}

#[inline]
fn encode_hyphenated<'b>(src: &[u8; 16], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
    let buf = &mut buffer[..Hyphenated::LENGTH];
    let dst = buf.as_mut_ptr();

    // SAFETY: `buf` is guaranteed to be at least `LEN` bytes
    // SAFETY: The encoded buffer is ASCII encoded
    unsafe {
        ptr::write(dst.cast(), format_hyphenated(src, upper));
        str::from_utf8_unchecked_mut(buf)
    }
}

#[inline]
fn encode_braced<'b>(src: &[u8; 16], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
    let buf = &mut buffer[..Braced::LENGTH];
    buf[0] = b'{';
    buf[Braced::LENGTH - 1] = b'}';

    // SAFETY: `buf` is guaranteed to be at least `LEN` bytes
    // SAFETY: The encoded buffer is ASCII encoded
    unsafe {
        let dst = buf.as_mut_ptr().add(1);

        ptr::write(dst.cast(), format_hyphenated(src, upper));
        str::from_utf8_unchecked_mut(buf)
    }
}

#[inline]
fn encode_urn<'b>(src: &[u8; 16], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
    let buf = &mut buffer[..Urn::LENGTH];
    buf[..9].copy_from_slice(URN_PREFIX);

    // SAFETY: `buf` is guaranteed to be at least `LEN` bytes
    // SAFETY: The encoded buffer is ASCII encoded
    unsafe {
        let dst = buf.as_mut_ptr().add(9);

        ptr::write(dst.cast(), format_hyphenated(src, upper));
        str::from_utf8_unchecked_mut(buf)
    }
}

// === impls ===

impl Yyid {
    /// Get an owned [`Hyphenated`] from a [`Yyid`]
    #[inline]
    pub const fn hyphenated(self) -> Hyphenated {
        Hyphenated(self)
    }

    /// Get a borrowed [`Hyphenated`] from a [`Yyid`]
    #[inline]
    pub const fn as_hyphenated(&self) -> &Hyphenated {
        // SAFETY: `Yyid` and `Hyphenated` have the same ABI
        unsafe { &*(self as *const Yyid as *const Hyphenated) }
    }

    /// Get an owned [`Simple`] from a [`Yyid`]
    #[inline]
    pub const fn simple(self) -> Simple {
        Simple(self)
    }

    /// Get a borrowed [`Simple`] from a [`Yyid`]
    #[inline]
    pub const fn as_simple(&self) -> &Simple {
        // SAFETY: `Yyid` and `Simple` have the same ABI
        unsafe { &*(self as *const Yyid as *const Simple) }
    }

    /// Get an owned [`Urn`] from a [`Yyid`]
    #[inline]
    pub const fn urn(self) -> Urn {
        Urn(self)
    }

    /// Get a borrowed [`Urn`] from a [`Yyid`]
    #[inline]
    pub const fn as_urn(&self) -> &Urn {
        // SAFETY: `Yyid` and `Urn` have the same ABI
        unsafe { &*(self as *const Yyid as *const Urn) }
    }

    /// Get an owned [`Braced`] from a [`Yyid`]
    #[inline]
    pub const fn braced(self) -> Braced {
        Braced(self)
    }

    /// Get a borrowed [`Braced`] from a [`Yyid`]
    #[inline]
    pub const fn as_braced(&self) -> &Braced {
        // SAFETY: `Yyid` and `Braced` have the same ABI
        unsafe { &*(self as *const Yyid as *const Braced) }
    }
}

impl Hyphenated {
    /// Hyphenated string length
    pub const LENGTH: usize = 36;

    /// Wraps a [`Yyid`] into a [`Hyphenated`]
    pub const fn from_yyid(yyid: Yyid) -> Self {
        Self(yyid)
    }

    /// Get a reference to the underlying [`Yyid`].
    pub const fn as_yyid(&self) -> &Yyid {
        &self.0
    }

    /// Consumes the [`Hyphenated`], returning the underlying [`Yyid`].
    pub const fn into_yyid(self) -> Yyid {
        self.0
    }

    /// Writes the [`Yyid`] as a lower-case hyphenated string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded YYID.
    #[inline]
    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_hyphenated(self.0.as_bytes(), buffer, false)
    }

    /// Writes the [`Yyid`] as a upper-case hyphenated string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded YYID.
    #[inline]
    pub fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_hyphenated(self.0.as_bytes(), buffer, true)
    }
}

impl Simple {
    /// Simple string length
    const LENGTH: usize = 32;

    /// Wraps a [`Yyid`] into a [`Simple`]
    pub const fn from_yyid(yyid: Yyid) -> Self {
        Self(yyid)
    }

    /// Get a reference to the underlying [`Yyid`].
    pub const fn as_yyid(&self) -> &Yyid {
        &self.0
    }

    /// Consumes the [`Simple`], returning the underlying [`Yyid`].
    pub const fn into_yyid(self) -> Yyid {
        self.0
    }

    /// Writes the [`Yyid`] as a lower-case simple string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded YYID.
    #[inline]
    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_simple(self.0.as_bytes(), buffer, false)
    }

    /// Writes the [`Yyid`] as a upper-case simple string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded YYID.
    #[inline]
    pub fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_simple(self.0.as_bytes(), buffer, true)
    }
}

impl Urn {
    /// URN string length
    const LENGTH: usize = 45;

    /// Wraps a [`Yyid`] into a [`Urn`]
    pub const fn from_yyid(yyid: Yyid) -> Self {
        Self(yyid)
    }

    /// Get a reference to the underlying [`Yyid`].
    pub const fn as_yyid(&self) -> &Yyid {
        &self.0
    }

    /// Consumes the [`Urn`], returning the underlying [`Yyid`].
    pub const fn into_yyid(self) -> Yyid {
        self.0
    }

    /// Writes the [`Yyid`] as a lower-case URN string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded YYID.
    #[inline]
    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_urn(self.0.as_bytes(), buffer, false)
    }

    /// Writes the [`Yyid`] as a upper-case URN string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded YYID.
    #[inline]
    pub fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_urn(self.0.as_bytes(), buffer, true)
    }
}

impl Braced {
    /// Braced string length
    const LENGTH: usize = 38;

    /// Wraps a [`Yyid`] into a [`Braced`]
    pub const fn from_yyid(yyid: Yyid) -> Self {
        Self(yyid)
    }

    /// Get a reference to the underlying [`Yyid`].
    pub const fn as_yyid(&self) -> &Yyid {
        &self.0
    }

    /// Consumes the [`Braced`], returning the underlying [`Yyid`].
    pub const fn into_yyid(self) -> Yyid {
        self.0
    }

    /// Writes the [`Yyid`] as a lower-case braced string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded YYID.
    #[inline]
    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_braced(self.0.as_bytes(), buffer, false)
    }

    /// Writes the [`Yyid`] as a upper-case braced string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded YYID.
    #[inline]
    pub fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_braced(self.0.as_bytes(), buffer, true)
    }
}

// === Formatters ===

macro_rules! impl_fmt_traits {
    ($($T:ident<$($a:lifetime),*>),+) => {$(
        impl<$($a),*> fmt::Display for $T<$($a),*> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(self, f)
            }
        }

        impl<$($a),*> fmt::LowerHex for $T<$($a),*> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.encode_lower(&mut [0; Self::LENGTH]))
            }
        }

        impl<$($a),*> fmt::UpperHex for $T<$($a),*> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.encode_upper(&mut [0; Self::LENGTH]))
            }
        }

        impl_fmt_from!($T<$($a),*>);
    )+}
}

macro_rules! impl_fmt_from {
    ($T:ident<>) => {
        impl From<Yyid> for $T {
            #[inline]
            fn from(f: Yyid) -> Self {
                $T(f)
            }
        }

        impl From<$T> for Yyid {
            #[inline]
            fn from(f: $T) -> Self {
                f.into_yyid()
            }
        }

        impl AsRef<Yyid> for $T {
            #[inline]
            fn as_ref(&self) -> &Yyid {
                &self.0
            }
        }

        impl Borrow<Yyid> for $T {
            #[inline]
            fn borrow(&self) -> &Yyid {
                &self.0
            }
        }
    };
    ($T:ident<$a:lifetime>) => {
        impl<$a> From<&$a Yyid> for $T<$a> {
            #[inline]
            fn from(f: &$a Yyid) -> Self {
                $T::from_yyid_ref(f)
            }
        }

        impl<$a> From<$T<$a>> for &$a Yyid {
            #[inline]
            fn from(f: $T<$a>) -> &$a Yyid {
                f.0
            }
        }

        impl<$a> AsRef<Yyid> for $T<$a> {
            #[inline]
            fn as_ref(&self) -> &Yyid {
                self.0
            }
        }

        impl<$a> Borrow<Yyid> for $T<$a> {
            #[inline]
            fn borrow(&self) -> &Yyid {
                self.0
            }
        }
    };
}

impl_fmt_traits! {
    Hyphenated<>,
    Simple<>,
    Urn<>,
    Braced<>
}
