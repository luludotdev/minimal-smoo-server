use bytes::{Bytes, BytesMut};

pub trait PacketBytes {
    fn write_bytes(&self, bytes: &mut BytesMut);
    fn from_bytes(bytes: &mut Bytes) -> Self;
}
