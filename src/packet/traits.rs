use bytes::{Bytes, BytesMut};

pub trait PacketBytes {
    fn write_bytes(&self, bytes: &mut BytesMut) -> usize;
    fn from_bytes(bytes: &mut Bytes) -> Self;
}

pub trait Packet: PacketBytes {}
