use bytes::{Buf, BytesMut};
use color_eyre::Result;
use uuid::Uuid;

use super::header::{PacketHeader, PacketType};

pub trait PacketBytes
where
    Self: Sized,
{
    fn write_bytes(&self, buf: &mut BytesMut) -> usize;
    fn from_bytes<T: Buf>(buf: &mut T) -> Result<Self>;
}

pub trait Packet: PacketBytes + Into<PacketType> {
    #[inline]
    fn into_header(self, id: Uuid) -> PacketHeader {
        PacketHeader {
            id,
            packet: self.into(),
        }
    }
}
