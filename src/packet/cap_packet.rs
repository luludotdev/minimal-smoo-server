use glam::{Quat, Vec3};
use smoo_derive::PacketBytes;

use super::fixed_string::FixedString;
use super::header::PacketData;
use super::traits::IntoPacket;

#[derive(Debug, Clone, Copy, PartialEq, PacketBytes)]
pub struct CapPacket {
    pub position: Vec3,
    pub quaternion: Quat,
    pub cap_out: bool,
    pub cap_anim: FixedString<0x30>,
}

impl From<CapPacket> for PacketData {
    #[inline(always)]
    fn from(packet: CapPacket) -> Self {
        Self::Cap(packet)
    }
}

impl IntoPacket for CapPacket {}
