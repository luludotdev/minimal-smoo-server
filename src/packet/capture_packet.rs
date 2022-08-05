use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketType;
use super::traits::Packet;

#[derive(Debug, PacketBytes)]
pub struct CapturePacket {
    pub model: FixedString<0x20>,
}

impl From<CapturePacket> for PacketType {
    #[inline(always)]
    fn from(packet: CapturePacket) -> Self {
        Self::Capture(packet)
    }
}

impl Packet for CapturePacket {}
