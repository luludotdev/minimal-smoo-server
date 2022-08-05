use bytes::{Buf, BufMut, Bytes, BytesMut};
use glam::{Quat, Vec3};
use uuid::Uuid;

use super::traits::PacketBytes;

impl PacketBytes for bool {
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        let uint = if *self { 1 } else { 0 };
        buf.put_u8(uint);

        1
    }

    fn from_bytes(buf: &mut Bytes) -> Self {
        let uint = buf.get_u8();
        uint == 1
    }
}

impl PacketBytes for Uuid {
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        let uuid = self.into_bytes();
        buf.put(&uuid[..]);

        16
    }

    fn from_bytes(buf: &mut Bytes) -> Self {
        let mut dst = [0u8; 16];
        buf.copy_to_slice(&mut dst);

        Uuid::from_bytes(dst)
    }
}

impl PacketBytes for Vec3 {
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        buf.put_f32_le(self.x);
        buf.put_f32_le(self.y);
        buf.put_f32_le(self.z);

        12
    }

    fn from_bytes(buf: &mut Bytes) -> Self {
        Self {
            x: buf.get_f32_le(),
            y: buf.get_f32_le(),
            z: buf.get_f32_le(),
        }
    }
}

impl PacketBytes for Quat {
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        buf.put_f32_le(self.x);
        buf.put_f32_le(self.y);
        buf.put_f32_le(self.z);
        buf.put_f32_le(self.w);

        16
    }

    fn from_bytes(buf: &mut Bytes) -> Self {
        Quat::from_xyzw(
            buf.get_f32_le(),
            buf.get_f32_le(),
            buf.get_f32_le(),
            buf.get_f32_le(),
        )
    }
}
