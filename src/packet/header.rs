use bytes::{BufMut, Bytes, BytesMut};
use color_eyre::Result;
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

impl PacketHeader {
    pub fn bytes(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(128);
        self.write_bytes(&mut buf);

        buf.freeze()
    }
}

impl PacketBytes for PacketHeader {
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        let mut written = 0;
        let packet_id = self.packet.id();

        written += self.id.write_bytes(buf);
        written += packet_id.write_bytes(buf);

        let mut packet_buf = BytesMut::with_capacity(128);
        let packet_byte_count = self.packet.write_bytes(&mut packet_buf);

        let packet_byte_short = packet_byte_count as u16;
        written += packet_byte_short.write_bytes(buf);

        buf.put(packet_buf);
        written += packet_byte_count;

        written
    }

    fn from_bytes(buf: &mut Bytes) -> Result<Self> {
        todo!()
    }
}

impl From<PacketHeader> for Bytes {
    #[inline]
    fn from(header: PacketHeader) -> Self {
        header.bytes()
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

impl PacketType {
    #[inline]
    pub fn id(&self) -> u8 {
        match self {
            PacketType::Unknown => 0,
            PacketType::Init(_) => 1,
            PacketType::Player(_) => 2,
            PacketType::Cap(_) => 3,
            PacketType::Game(_) => 4,
            PacketType::Tag => 5,
            PacketType::Connect(_) => 6,
            PacketType::Disconnect => 7,
            PacketType::Costume(_) => 8,
            PacketType::Shine(_) => 9,
            PacketType::Capture(_) => 10,
            PacketType::ChangeStage(_) => 11,
        }
    }
}

impl PacketBytes for PacketType {
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        match self {
            // Do nothing
            PacketType::Unknown | PacketType::Tag | PacketType::Disconnect => 0,

            PacketType::Init(packet) => packet.write_bytes(buf),
            PacketType::Player(packet) => packet.write_bytes(buf),
            PacketType::Cap(packet) => packet.write_bytes(buf),
            PacketType::Game(packet) => packet.write_bytes(buf),
            PacketType::Connect(packet) => packet.write_bytes(buf),
            PacketType::Costume(packet) => packet.write_bytes(buf),
            PacketType::Shine(packet) => packet.write_bytes(buf),
            PacketType::Capture(packet) => packet.write_bytes(buf),
            PacketType::ChangeStage(packet) => packet.write_bytes(buf),
        }
    }

    fn from_bytes(_: &mut Bytes) -> Result<Self> {
        unimplemented!()
    }
}
