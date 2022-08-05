use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketType;
use super::traits::Packet;

#[derive(Debug, PacketBytes)]
pub struct ChangeStagePacket {
    pub id: FixedString<0x10>,
    pub stage: FixedString<0x30>,
    pub scenario: i8,
    pub sub_scenario: u8,
}

impl From<ChangeStagePacket> for PacketType {
    #[inline(always)]
    fn from(packet: ChangeStagePacket) -> Self {
        Self::ChangeStage(packet)
    }
}

impl Packet for ChangeStagePacket {}
