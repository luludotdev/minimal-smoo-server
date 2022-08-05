use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketData;
use super::traits::IntoPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketBytes)]
pub struct ChangeStagePacket {
    pub id: FixedString<0x10>,
    pub stage: FixedString<0x30>,
    pub scenario: i8,
    pub sub_scenario: u8,
}

impl From<ChangeStagePacket> for PacketData {
    #[inline(always)]
    fn from(packet: ChangeStagePacket) -> Self {
        Self::ChangeStage(packet)
    }
}

impl IntoPacket for ChangeStagePacket {}
