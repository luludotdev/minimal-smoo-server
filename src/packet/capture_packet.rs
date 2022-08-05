use smoo_derive::Packet;

use super::fixed_string::FixedString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Packet)]
#[packet("Capture")]
pub struct CapturePacket {
    pub model: FixedString<0x20>,
}
