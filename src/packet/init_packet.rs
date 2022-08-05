use smoo_derive::Packet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Packet)]
#[packet("Init")]
pub struct InitPacket {
    pub max_players: u16,
}
