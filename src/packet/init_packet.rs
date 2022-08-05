use smoo_derive::PacketBytes;

use super::header::PacketData;
use super::traits::IntoPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketBytes)]
pub struct InitPacket {
    pub max_players: u16,
}

impl From<InitPacket> for PacketData {
    #[inline(always)]
    fn from(packet: InitPacket) -> Self {
        Self::Init(packet)
    }
}

impl IntoPacket for InitPacket {}
