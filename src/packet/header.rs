use bytes::{Bytes, BytesMut};
use uuid::Uuid;

use super::cap_packet::CapPacket;
use super::capture_packet::CapturePacket;
use super::change_stage_packet::ChangeStagePacket;
use super::connect_packet::ConnectPacket;
use super::costume_packet::CostumePacket;
use super::game_packet::GamePacket;
use super::init_packet::InitPacket;
use super::player_packet::PlayerPacket;
use super::shine_packet::ShinePacket;
use super::PacketBytes;

#[derive(Debug)]
pub struct PacketHeader {
    pub id: Uuid,
    pub packet: PacketType,
}

impl PacketBytes for PacketHeader {
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        todo!()
    }

    fn from_bytes(buf: &mut Bytes) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum PacketType {
    Unknown,
    Init(InitPacket),
    Player(PlayerPacket),
    Cap(CapPacket),
    Game(GamePacket),
    Tag,
    Connect(ConnectPacket),
    Disconnect,
    Costume(CostumePacket),
    Shine(ShinePacket),
    Capture(CapturePacket),
    ChangeStage(ChangeStagePacket),
}

impl PacketBytes for PacketType {
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        todo!()
    }

    fn from_bytes(_: &mut Bytes) -> Self {
        unimplemented!()
    }
}
