use smoo_derive::Packet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Packet)]
#[packet("Shine")]
pub struct ShinePacket {
    pub id: i32,
    pub is_grand: bool,
}
