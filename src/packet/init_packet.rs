use smoo_derive::PacketBytes;

use super::header::PacketType;
use super::traits::Packet;

#[derive(Debug, PacketBytes)]
pub struct InitPacket {
    pub max_players: u16,
}

impl From<InitPacket> for PacketType {
    #[inline(always)]
    fn from(packet: InitPacket) -> Self {
        Self::Init(packet)
    }
}

impl Packet for InitPacket {}
