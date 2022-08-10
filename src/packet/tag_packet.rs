use bytes::Buf;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use smoo_derive::Packet;

use super::PacketBytes;

#[derive(Debug, Clone, Copy, PartialEq, Packet)]
#[packet("Tag")]
pub struct TagPacket {
    update_type: UpdateType,
    is_it: bool,
    seconds: u8,
    minutes: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UpdateType {
    Time = 1 << 0,
    State = 1 << 1,
}

impl PacketBytes for UpdateType {
    fn write_bytes(&self, buf: &mut bytes::BytesMut) -> usize {
        let u8 = *self as u8;
        u8.write_bytes(buf)
    }

    fn from_bytes<T: Buf>(buf: &mut T) -> Result<Self> {
        let id = u8::from_bytes(buf)?;
        match id {
            1 => Ok(UpdateType::Time),
            2 => Ok(UpdateType::State),

            _ => Err(eyre!("invalid tag update type: {id}")),
        }
    }
}
