use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketData;
use super::traits::IntoPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PacketBytes)]
pub struct CapturePacket {
    pub model: FixedString<0x20>,
}

impl From<CapturePacket> for PacketData {
    #[inline(always)]
    fn from(packet: CapturePacket) -> Self {
        Self::Capture(packet)
    }
}

impl IntoPacket for CapturePacket {}
