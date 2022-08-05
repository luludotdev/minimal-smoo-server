use smoo_derive::Packet;

use super::fixed_string::FixedString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Packet)]
#[packet("Game")]
pub struct GamePacket {
    pub is_2d: bool,
    pub scenario: u8,
    pub stage: FixedString<0x20>,
}
