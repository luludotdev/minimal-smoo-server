use glam::{Quat, Vec3};
use smoo_derive::Packet;

#[derive(Debug, Clone, Copy, PartialEq, Packet)]
#[packet("Player")]
pub struct PlayerPacket {
    pub position: Vec3,
    pub quaternion: Quat,
    pub animation_blend_weights: [f32; 6],
    pub act: i16,
    pub subact: i16,
}
