use std::fmt::Display;
use std::str::Utf8Error;

use color_eyre::Result;

use super::PacketBytes;

#[derive(Debug)]
pub struct FixedString<const N: usize> {
    inner: [u8; N],
}

impl<const N: usize> FixedString<N> {
    #[inline]
    pub fn try_to_string(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.inner)
    }
}

impl<const N: usize> Display for FixedString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.try_to_string().unwrap();
        write!(f, "{str}")
    }
}

impl<const N: usize> PacketBytes for FixedString<N> {
    #[inline]
    fn write_bytes(&self, buf: &mut bytes::BytesMut) -> usize {
        self.inner.write_bytes(buf)
    }

    #[inline]
    fn from_bytes(buf: &mut bytes::Bytes) -> Result<Self> {
        let inner = <[u8; N] as PacketBytes>::from_bytes(buf)?;
        Ok(Self { inner })
    }
}
