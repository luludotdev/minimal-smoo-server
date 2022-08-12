use smoo_derive::Packet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Packet)]
#[packet("Tag")]
pub struct TagPacket {
    update_bits: u8,
    is_it: bool,
    seconds: u8,
    minutes: u16,
}
