use bytes::{Buf, BytesMut};
use color_eyre::Result;
use uuid::Uuid;

use super::header::{Packet, PacketData};

pub trait PacketBytes
where
    Self: Sized,
{
    fn write_bytes(&self, buf: &mut BytesMut) -> usize;
    fn from_bytes<T: Buf>(buf: &mut T) -> Result<Self>;
}

pub trait IntoPacket: PacketBytes + Into<PacketData> {
    #[inline]
    fn into_packet(self, id: Uuid) -> Packet {
        Packet {
            id,
            data: self.into(),
        }
    }
}
