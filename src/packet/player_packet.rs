use glam::{Quat, Vec3};
use smoo_derive::PacketBytes;

use super::header::PacketType;
use super::traits::Packet;

#[derive(Debug, Clone, Copy, PartialEq, PacketBytes)]
pub struct PlayerPacket {
    pub position: Vec3,
    pub quaternion: Quat,
    pub animation_blend_weights: [f32; 6],
    pub act: u16,
    pub subact: u16,
}

impl From<PlayerPacket> for PacketType {
    #[inline(always)]
    fn from(packet: PlayerPacket) -> Self {
        Self::Player(packet)
    }
}

impl Packet for PlayerPacket {}
