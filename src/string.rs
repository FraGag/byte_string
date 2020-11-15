use core::{
    borrow::{Borrow, BorrowMut},
    fmt::Display,
    fmt::{self, Debug, Formatter},
    iter::FromIterator,
    ops::{Deref, DerefMut},
};

use crate::ByteStr;

/// Wraps a vector of bytes and provides a `Debug` implementation
/// that outputs the slice using the Rust byte string syntax (e.g. `b"abc"`).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteString(pub Vec<u8>);

impl ByteString {
    /// Moves a vector of bytes to a new `ByteString`.
    pub fn new(s: Vec<u8>) -> ByteString {
        ByteString(s)
    }
}

impl From<Vec<u8>> for ByteString {
    fn from(s: Vec<u8>) -> ByteString {
        ByteString::new(s)
    }
}

impl From<ByteString> for Vec<u8> {
    fn from(s: ByteString) -> Vec<u8> {
        s.0
    }
}

impl AsRef<Vec<u8>> for ByteString {
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl AsRef<[u8]> for ByteString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<Vec<u8>> for ByteString {
    fn as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl AsMut<[u8]> for ByteString {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Borrow<ByteStr> for ByteString {
    fn borrow(&self) -> &ByteStr {
        ByteStr::new(&self.0)
    }
}

impl Borrow<Vec<u8>> for ByteString {
    fn borrow(&self) -> &Vec<u8> {
        &self.0
    }
}

impl Borrow<[u8]> for ByteString {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl BorrowMut<ByteStr> for ByteString {
    fn borrow_mut(&mut self) -> &mut ByteStr {
        ByteStr::new_mut(&mut self.0)
    }
}

impl BorrowMut<Vec<u8>> for ByteString {
    fn borrow_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl BorrowMut<[u8]> for ByteString {
    fn borrow_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl PartialEq<Vec<u8>> for ByteString {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.0 == *other
    }
}

impl PartialEq<[u8]> for ByteString {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == other
    }
}

impl PartialEq<ByteString> for Vec<u8> {
    fn eq(&self, other: &ByteString) -> bool {
        self == &other.0
    }
}

impl PartialEq<ByteString> for [u8] {
    fn eq(&self, other: &ByteString) -> bool {
        self == &other.0[..]
    }
}

impl Deref for ByteString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl DerefMut for ByteString {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl Default for ByteString {
    fn default() -> ByteString {
        ByteString::new(vec![])
    }
}

impl FromIterator<u8> for ByteString {
    fn from_iter<I>(iter: I) -> ByteString
    where
        I: IntoIterator<Item = u8>,
    {
        ByteString::new(Vec::from_iter(iter))
    }
}

impl<'a> IntoIterator for ByteString {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ByteString {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a> IntoIterator for &'a mut ByteString {
    type Item = &'a mut u8;
    type IntoIter = core::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl Debug for ByteString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        // Delegate to ByteStr's implementation
        Debug::fmt(Borrow::<ByteStr>::borrow(self), f)
    }
}

impl Display for ByteString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        // Delegate to ByteStr's implementation
        Display::fmt(Borrow::<ByteStr>::borrow(self), f)
    }
}
