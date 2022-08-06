use smoo_derive::Packet;

use super::fixed_string::FixedString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Packet)]
#[packet("ChangeStage")]
pub struct ChangeStagePacket {
    pub stage: FixedString<0x30>,
    pub id: FixedString<0x10>,
    pub scenario: i8,
    pub sub_scenario: u8,
}
