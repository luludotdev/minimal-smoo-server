use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketData;
use super::traits::IntoPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketBytes)]
pub struct CostumePacket {
    pub body: FixedString<0x20>,
    pub cap: FixedString<0x20>,
}

impl From<CostumePacket> for PacketData {
    #[inline(always)]
    fn from(packet: CostumePacket) -> Self {
        Self::Costume(packet)
    }
}

impl IntoPacket for CostumePacket {}
