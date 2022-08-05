use glam::{Quat, Vec3};
use smoo_derive::Packet;

use super::fixed_string::FixedString;

#[derive(Debug, Clone, Copy, PartialEq, Packet)]
#[packet("Cap")]
pub struct CapPacket {
    pub position: Vec3,
    pub quaternion: Quat,
    pub cap_out: bool,
    pub cap_anim: FixedString<0x30>,
}
