use color_eyre::eyre::eyre;
use color_eyre::Result;
use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketType;
use super::traits::{Packet, PacketBytes};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketBytes)]
pub struct ConnectPacket {
    pub connection_type: ConnectionType,
    pub max_players: u16,
    pub nickname: FixedString<0x20>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ConnectionType {
    Init = 0,
    Reconnect = 1,
}

impl PacketBytes for ConnectionType {
    #[inline]
    fn write_bytes(&self, buf: &mut bytes::BytesMut) -> usize {
        let u16 = *self as u16;
        u16.write_bytes(buf)
    }

    #[inline]
    fn from_bytes(buf: &mut bytes::Bytes) -> Result<Self> {
        let id = u16::from_bytes(buf)?;
        match id {
            0 => Ok(ConnectionType::Init),
            1 => Ok(ConnectionType::Reconnect),

            _ => Err(eyre!("invalid connection type: {id}")),
        }
    }
}

impl From<ConnectPacket> for PacketType {
    #[inline(always)]
    fn from(packet: ConnectPacket) -> Self {
        Self::Connect(packet)
    }
}

impl Packet for ConnectPacket {}
