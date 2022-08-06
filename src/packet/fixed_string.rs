use std::fmt::Display;
use std::str::{FromStr, Utf8Error};

use bytes::{Buf, BufMut, BytesMut};
use color_eyre::eyre::eyre;
use color_eyre::{Report, Result};

use super::PacketBytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedString<const N: usize> {
    inner: [u8; N],
}

impl<const N: usize> FixedString<N> {
    #[inline]
    pub fn try_as_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.inner).map(|s| s.trim_matches('\0'))
    }

    #[inline]
    pub fn try_to_string(&self) -> Result<String, Utf8Error> {
        Ok(self.try_as_str()?.to_owned())
    }
}

impl<const N: usize> Default for FixedString<N> {
    #[inline]
    fn default() -> Self {
        "".parse().unwrap()
    }
}

impl<const N: usize> Display for FixedString<N> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.try_as_str().unwrap();
        write!(f, "{str}")
    }
}

impl<const N: usize> PacketBytes for FixedString<N> {
    #[inline]
    fn write_bytes(&self, buf: &mut bytes::BytesMut) -> usize {
        self.inner.write_bytes(buf)
    }

    #[inline]
    fn from_bytes<T: Buf>(buf: &mut T) -> Result<Self> {
        let inner = <[u8; N] as PacketBytes>::from_bytes(buf)?;
        Ok(Self { inner })
    }
}

impl<const N: usize> FromStr for FixedString<N> {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string = s.to_owned();
        let mut bytes = BytesMut::from(string.as_bytes());

        let max_len = std::mem::size_of::<[u8; N]>();
        let len = bytes.len();

        if len > max_len {
            Err(eyre!("string is too long"))
        } else {
            // Pad
            let pad_len = max_len - len;

            let padding = vec![0; pad_len];
            bytes.put(&padding[..]);

            let inner = bytes[..].try_into().unwrap();
            Ok(Self { inner })
        }
    }
}

impl<const N: usize> TryFrom<FixedString<N>> for String {
    type Error = Utf8Error;

    #[inline]
    fn try_from(value: FixedString<N>) -> Result<Self, Self::Error> {
        value.try_to_string()
    }
}
