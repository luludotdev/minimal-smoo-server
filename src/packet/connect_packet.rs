use bytes::Buf;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use smoo_derive::Packet;

use super::fixed_string::FixedString;
use super::traits::PacketBytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Packet)]
#[packet("Connect")]
pub struct ConnectPacket {
    pub connection_type: ConnectionType,
    pub max_players: u16,
    pub nickname: FixedString<0x20>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ConnectionType {
    Init = 0,
    Reconnect = 1,
}

impl PacketBytes for ConnectionType {
    fn write_bytes(&self, buf: &mut bytes::BytesMut) -> usize {
        let u32 = *self as u32;
        u32.write_bytes(buf)
    }

    fn from_bytes<T: Buf>(buf: &mut T) -> Result<Self> {
        let id = u32::from_bytes(buf)?;
        match id {
            0 => Ok(ConnectionType::Init),
            1 => Ok(ConnectionType::Reconnect),

            _ => Err(eyre!("invalid connection type: {id}")),
        }
    }
}
