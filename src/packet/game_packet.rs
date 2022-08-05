use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketData;
use super::traits::IntoPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketBytes)]
pub struct GamePacket {
    pub is_2d: bool,
    pub scenario: u8,
    pub stage: FixedString<0x20>,
}

impl From<GamePacket> for PacketData {
    #[inline(always)]
    fn from(packet: GamePacket) -> Self {
        Self::Game(packet)
    }
}

impl IntoPacket for GamePacket {}
