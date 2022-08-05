use smoo_derive::Packet;

use super::fixed_string::FixedString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Packet)]
#[packet("Costume")]
pub struct CostumePacket {
    pub body: FixedString<0x20>,
    pub cap: FixedString<0x20>,
}
