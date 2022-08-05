use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketType;
use super::traits::Packet;

#[derive(Debug, PacketBytes)]
pub struct GamePacket {
    is_2d: bool,
    scenario: u8,
    stage: FixedString<0x20>,
}

impl From<GamePacket> for PacketType {
    #[inline(always)]
    fn from(packet: GamePacket) -> Self {
        Self::Game(packet)
    }
}

impl Packet for GamePacket {}
