use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketType;
use super::traits::{Packet, PacketBytes};

#[derive(Debug, PacketBytes)]
pub struct ConnectPacket {
    connection_type: ConnectionType,
    max_players: u16,
    nickname: FixedString<0x20>,
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
    fn from_bytes(buf: &mut bytes::Bytes) -> Self {
        match u16::from_bytes(buf) {
            0 => ConnectionType::Init,
            1 => ConnectionType::Reconnect,

            // TODO: Fallible
            _ => panic!("invalid connection typr"),
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
