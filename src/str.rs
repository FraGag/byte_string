use core::{
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

use ref_cast::RefCast;

/// Wraps a byte slice and provides a `Debug` implementation
/// that outputs the slice using the Rust byte string syntax (e.g. `b"abc"`).
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, ref_cast::RefCast)]
pub struct ByteStr(pub [u8]);

impl ByteStr {
    /// Converts an immutable byte slice to an immutable `ByteStr` reference.
    pub fn new(s: &[u8]) -> &ByteStr {
        Self::ref_cast(s)
    }

    /// Converts a mutable byte slice to a mutable `ByteStr` reference.
    pub fn new_mut(s: &mut [u8]) -> &mut ByteStr {
        Self::ref_cast_mut(s)
    }
}

impl<'a> From<&'a [u8]> for &'a ByteStr {
    fn from(s: &[u8]) -> &ByteStr {
        ByteStr::new(s)
    }
}

impl<'a> From<&'a mut [u8]> for &'a mut ByteStr {
    fn from(s: &mut [u8]) -> &mut ByteStr {
        ByteStr::new_mut(s)
    }
}

impl<'a> From<&'a ByteStr> for &'a [u8] {
    fn from(s: &ByteStr) -> &[u8] {
        &s.0
    }
}

impl<'a> From<&'a mut ByteStr> for &'a mut [u8] {
    fn from(s: &mut ByteStr) -> &mut [u8] {
        &mut s.0
    }
}

impl AsRef<[u8]> for ByteStr {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<ByteStr> for [u8] {
    fn as_ref(&self) -> &ByteStr {
        ByteStr::new(self)
    }
}

impl AsMut<[u8]> for ByteStr {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl AsMut<ByteStr> for [u8] {
    fn as_mut(&mut self) -> &mut ByteStr {
        ByteStr::new_mut(self)
    }
}

impl PartialEq<[u8]> for ByteStr {
    fn eq(&self, other: &[u8]) -> bool {
        &self.0 == other
    }
}

impl PartialEq<ByteStr> for [u8] {
    fn eq(&self, other: &ByteStr) -> bool {
        self == &other.0
    }
}

impl Deref for ByteStr {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl DerefMut for ByteStr {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<'a> Default for &'a ByteStr {
    fn default() -> &'a ByteStr {
        ByteStr::new(&[])
    }
}

impl<'a> Default for &'a mut ByteStr {
    fn default() -> &'a mut ByteStr {
        ByteStr::new_mut(&mut [])
    }
}

impl<'a> IntoIterator for &'a ByteStr {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a mut ByteStr {
    type Item = &'a mut u8;
    type IntoIter = core::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl Debug for ByteStr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "b\"")?;

        for &byte in self {
            for ch in core::ascii::escape_default(byte) {
                write!(f, "{}", ch as char)?;
            }
        }

        write!(f, "\"")
    }
}
