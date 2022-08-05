use smoo_derive::PacketBytes;

use super::header::PacketData;
use super::traits::IntoPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketBytes)]
pub struct ShinePacket {
    pub id: i32,
    pub is_grand: bool,
}

impl From<ShinePacket> for PacketData {
    #[inline(always)]
    fn from(packet: ShinePacket) -> Self {
        Self::Shine(packet)
    }
}

impl IntoPacket for ShinePacket {}
