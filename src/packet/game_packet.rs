use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketType;
use super::traits::Packet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketBytes)]
pub struct GamePacket {
    pub is_2d: bool,
    pub scenario: u8,
    pub stage: FixedString<0x20>,
}

impl From<GamePacket> for PacketType {
    #[inline(always)]
    fn from(packet: GamePacket) -> Self {
        Self::Game(packet)
    }
}

impl Packet for GamePacket {}
