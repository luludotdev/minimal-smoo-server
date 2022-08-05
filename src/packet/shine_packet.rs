use smoo_derive::PacketBytes;

use super::header::PacketType;
use super::traits::Packet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketBytes)]
pub struct ShinePacket {
    pub id: i32,
    pub is_grand: bool,
}

impl From<ShinePacket> for PacketType {
    #[inline(always)]
    fn from(packet: ShinePacket) -> Self {
        Self::Shine(packet)
    }
}

impl Packet for ShinePacket {}
