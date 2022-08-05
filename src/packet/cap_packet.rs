use bytes::{Buf, BufMut, Bytes, BytesMut};
use glam::{Quat, Vec3};

use super::header::PacketType;
use super::traits::{Packet, PacketBytes};

#[derive(Debug)]
pub struct CapPacket {
    pub position: Vec3,
    pub quaternion: Quat,
    pub cap_out: bool,
    pub cap_anim: [u8; 30],
}

impl PacketBytes for CapPacket {
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        let mut written = 0;

        written += self.position.write_bytes(buf);
        written += self.quaternion.write_bytes(buf);
        written += self.cap_out.write_bytes(buf);

        buf.put(&self.cap_anim[..]);
        written += self.cap_anim.len();

        written
    }

    fn from_bytes(buf: &mut Bytes) -> Self {
        let position = Vec3::from_bytes(buf);
        let quaternion = Quat::from_bytes(buf);
        let cap_out = bool::from_bytes(buf);

        let mut cap_anim = [0u8; 30];
        buf.copy_to_slice(&mut cap_anim);

        Self {
            position,
            quaternion,
            cap_out,
            cap_anim,
        }
    }
}

impl From<CapPacket> for PacketType {
    #[inline(always)]
    fn from(packet: CapPacket) -> Self {
        Self::Cap(packet)
    }
}

impl Packet for CapPacket {}
