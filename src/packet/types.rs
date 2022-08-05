use bytes::{Buf, BufMut, Bytes, BytesMut};
use glam::{Quat, Vec3};

use super::traits::PacketBytes;

impl PacketBytes for bool {
    fn write_bytes(&self, bytes: &mut BytesMut) {
        let uint = if *self { 1 } else { 0 };
        bytes.put_u8(uint);
    }

    fn from_bytes(bytes: &mut Bytes) -> Self {
        let uint = bytes.get_u8();
        uint == 1
    }
}

impl PacketBytes for Vec3 {
    fn write_bytes(&self, bytes: &mut BytesMut) {
        bytes.put_f32_le(self.x);
        bytes.put_f32_le(self.y);
        bytes.put_f32_le(self.z);
    }

    fn from_bytes(bytes: &mut Bytes) -> Self {
        Self {
            x: bytes.get_f32_le(),
            y: bytes.get_f32_le(),
            z: bytes.get_f32_le(),
        }
    }
}

impl PacketBytes for Quat {
    fn write_bytes(&self, bytes: &mut BytesMut) {
        bytes.put_f32_le(self.x);
        bytes.put_f32_le(self.y);
        bytes.put_f32_le(self.z);
        bytes.put_f32_le(self.w);
    }

    fn from_bytes(bytes: &mut Bytes) -> Self {
        Quat::from_xyzw(
            bytes.get_f32_le(),
            bytes.get_f32_le(),
            bytes.get_f32_le(),
            bytes.get_f32_le(),
        )
    }
}
