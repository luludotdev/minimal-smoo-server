use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketType;
use super::traits::Packet;

#[derive(Debug, PacketBytes)]
pub struct CostumePacket {
    pub body: FixedString<0x20>,
    pub cap: FixedString<0x20>,
}

impl From<CostumePacket> for PacketType {
    #[inline(always)]
    fn from(packet: CostumePacket) -> Self {
        Self::Costume(packet)
    }
}

impl Packet for CostumePacket {}
