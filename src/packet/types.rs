use bytes::{Buf, BufMut, Bytes, BytesMut};
use glam::{Quat, Vec3};
use uuid::Uuid;

use super::traits::PacketBytes;

// region: Standard Types
impl PacketBytes for bool {
    #[inline]
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        let uint = if *self { 1 } else { 0 };
        buf.put_u8(uint);

        std::mem::size_of::<u8>()
    }

    #[inline]
    fn from_bytes(buf: &mut Bytes) -> Self {
        let uint = buf.get_u8();
        uint == 1
    }
}

impl<const N: usize> PacketBytes for [u8; N] {
    #[inline]
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        buf.put(&self[..]);
        std::mem::size_of::<Self>()
    }

    #[inline]
    fn from_bytes(buf: &mut Bytes) -> Self {
        let mut dst = [0u8; N];
        buf.copy_to_slice(&mut dst);

        dst
    }
}
// endregion

// region: Numeric Types
macro_rules! packet_bytes_num {
    ($type:ty) => {
        paste::paste! {
            #[automatically_derived]
            impl crate::packet::PacketBytes for $type {
                #[inline]
                fn write_bytes(&self, buf: &mut bytes::BytesMut) -> usize {
                    buf.[<put_ $type>](*self);
                    std::mem::size_of::<$type>()
                }

                #[inline]
                fn from_bytes(buf: &mut bytes::Bytes) -> Self {
                    buf.[<get_ $type>]()
                }
            }
        }
    };
}

macro_rules! packet_bytes_num_le {
    ($type:ty) => {
        paste::paste! {
            #[automatically_derived]
            impl crate::packet::PacketBytes for $type {
                #[inline]
                fn write_bytes(&self, buf: &mut bytes::BytesMut) -> usize {
                    buf.[<put_ $type _le>](*self);
                    std::mem::size_of::<$type>()
                }

                #[inline]
                fn from_bytes(buf: &mut bytes::Bytes) -> Self {
                    buf.[<get_ $type _le>]()
                }
            }
        }
    };
}

packet_bytes_num!(u8);
packet_bytes_num!(i8);
packet_bytes_num_le!(u16);
packet_bytes_num_le!(i16);
packet_bytes_num_le!(u32);
packet_bytes_num_le!(i32);
// endregion

// region: Foreign Types
impl PacketBytes for Uuid {
    #[inline]
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        let uuid = self.into_bytes();
        buf.put(&uuid[..]);

        uuid.len()
    }

    #[inline]
    fn from_bytes(buf: &mut Bytes) -> Self {
        let mut dst = [0u8; 16];
        buf.copy_to_slice(&mut dst);

        Uuid::from_bytes(dst)
    }
}

impl PacketBytes for Vec3 {
    #[inline]
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        buf.put_f32_le(self.x);
        buf.put_f32_le(self.y);
        buf.put_f32_le(self.z);

        std::mem::size_of::<f32>() * 3
    }

    #[inline]
    fn from_bytes(buf: &mut Bytes) -> Self {
        Self {
            x: buf.get_f32_le(),
            y: buf.get_f32_le(),
            z: buf.get_f32_le(),
        }
    }
}

impl PacketBytes for Quat {
    #[inline]
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        buf.put_f32_le(self.x);
        buf.put_f32_le(self.y);
        buf.put_f32_le(self.z);
        buf.put_f32_le(self.w);

        std::mem::size_of::<f32>() * 4
    }

    #[inline]
    fn from_bytes(buf: &mut Bytes) -> Self {
        Quat::from_xyzw(
            buf.get_f32_le(),
            buf.get_f32_le(),
            buf.get_f32_le(),
            buf.get_f32_le(),
        )
    }
}
// endregion
