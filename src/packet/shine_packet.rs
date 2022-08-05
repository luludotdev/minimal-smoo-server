use smoo_derive::PacketBytes;

use super::header::PacketType;
use super::traits::Packet;

#[derive(Debug, PacketBytes)]
pub struct ShinePacket {
    id: i32,
    is_grand: bool,
}

impl From<ShinePacket> for PacketType {
    #[inline(always)]
    fn from(packet: ShinePacket) -> Self {
        Self::Shine(packet)
    }
}

impl Packet for ShinePacket {}
