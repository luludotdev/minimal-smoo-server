use bytes::{Bytes, BytesMut};
use uuid::Uuid;

use super::cap_packet::CapPacket;
use super::PacketBytes;

#[derive(Debug)]
pub struct PacketHeader {
    pub id: Uuid,
    pub packet: PacketType,
}

impl PacketBytes for PacketHeader {
    fn write_bytes(&self, bytes: &mut BytesMut) -> usize {
        todo!()
    }

    fn from_bytes(bytes: &mut Bytes) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum PacketType {
    Unknown,
    Init(),         // TODO
    Player(),       // TODO
    Cap(CapPacket), // TODO
    Game(),         // TODO
    Tag(),          // TODO
    Connect(),      // TODO
    Disconnect,
    Costume(),     // TODO
    Shine(),       // TODO
    Capture(),     // TODO
    ChangeStage(), // TODO
}

impl PacketBytes for PacketType {
    fn write_bytes(&self, bytes: &mut BytesMut) -> usize {
        todo!()
    }

    fn from_bytes(_: &mut Bytes) -> Self {
        unimplemented!()
    }
}
