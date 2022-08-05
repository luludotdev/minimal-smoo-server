use glam::{Quat, Vec3};
use smoo_derive::PacketBytes;

use super::header::PacketType;
use super::traits::Packet;

#[derive(Debug, Clone, Copy, PartialEq, PacketBytes)]
pub struct CapPacket {
    pub position: Vec3,
    pub quaternion: Quat,
    pub cap_out: bool,
    pub cap_anim: [u8; 30],
}

impl From<CapPacket> for PacketType {
    #[inline(always)]
    fn from(packet: CapPacket) -> Self {
        Self::Cap(packet)
    }
}

impl Packet for CapPacket {}
