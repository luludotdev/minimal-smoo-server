use bytes::{Buf, BufMut, Bytes, BytesMut};
use color_eyre::{Report, Result};
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

// region: PacketHeader
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Packet {
    pub id: Uuid,
    pub data: PacketData,
}

impl Packet {
    pub fn to_bytes(self) -> Bytes {
        let mut buf = BytesMut::with_capacity(128);
        self.write_bytes(&mut buf);

        buf.freeze()
    }

    #[inline]
    pub fn from_bytes(mut buf: Bytes) -> Result<Self> {
        <Self as PacketBytes>::from_bytes(&mut buf)
    }

    #[inline]
    pub const fn buf_size() -> usize {
        std::mem::size_of::<Uuid>() + std::mem::size_of::<u16>() + std::mem::size_of::<u16>()
    }

    fn read_body<T: Buf>(id: Uuid, packet_id: u16, buf: &mut T) -> Result<Packet> {
        match packet_id {
            1 => {
                let packet = InitPacket::from_bytes(buf)?;
                Ok(Packet {
                    id,
                    data: packet.into(),
                })
            }

            2 => {
                let packet = PlayerPacket::from_bytes(buf)?;
                Ok(Packet {
                    id,
                    data: packet.into(),
                })
            }

            3 => {
                let packet = CapPacket::from_bytes(buf)?;
                Ok(Packet {
                    id,
                    data: packet.into(),
                })
            }

            4 => {
                let packet = GamePacket::from_bytes(buf)?;
                Ok(Packet {
                    id,
                    data: packet.into(),
                })
            }

            5 => Ok(Packet {
                id,
                data: PacketData::Tag,
            }),

            6 => {
                let packet = ConnectPacket::from_bytes(buf)?;
                Ok(Packet {
                    id,
                    data: packet.into(),
                })
            }

            7 => Ok(Packet {
                id,
                data: PacketData::Disconnect,
            }),

            8 => {
                let packet = CostumePacket::from_bytes(buf)?;
                Ok(Packet {
                    id,
                    data: packet.into(),
                })
            }

            9 => {
                let packet = ShinePacket::from_bytes(buf)?;
                Ok(Packet {
                    id,
                    data: packet.into(),
                })
            }

            10 => {
                let packet = CapturePacket::from_bytes(buf)?;
                Ok(Packet {
                    id,
                    data: packet.into(),
                })
            }

            11 => {
                let packet = ChangeStagePacket::from_bytes(buf)?;
                Ok(Packet {
                    id,
                    data: packet.into(),
                })
            }

            _ => Ok(Packet {
                id,
                data: PacketData::Unknown,
            }),
        }
    }
}

impl PacketBytes for Packet {
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        let mut written = 0;
        let packet_id = self.data.id();

        written += self.id.write_bytes(buf);
        written += packet_id.write_bytes(buf);

        let mut packet_buf = BytesMut::with_capacity(128);
        let packet_byte_count = self.data.write_bytes(&mut packet_buf);

        let packet_byte_short = packet_byte_count as u16;
        written += packet_byte_short.write_bytes(buf);

        buf.put(packet_buf);
        written += packet_byte_count;

        written
    }

    fn from_bytes<T: Buf>(buf: &mut T) -> Result<Self> {
        let id = <Uuid as PacketBytes>::from_bytes(buf)?;
        let packet_id = u16::from_bytes(buf)?;

        // Packet length
        let _ = u16::from_bytes(buf)?;
        Self::read_body(id, packet_id, buf)
    }
}

impl From<Packet> for Bytes {
    #[inline]
    fn from(header: Packet) -> Self {
        header.to_bytes()
    }
}

impl TryFrom<Bytes> for Packet {
    type Error = Report;

    #[inline]
    fn try_from(buf: Bytes) -> Result<Self, Self::Error> {
        Self::from_bytes(buf)
    }
}
// endregion

// region: PacketData
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PacketData {
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

impl PacketData {
    #[inline]
    pub fn id(&self) -> u16 {
        match self {
            PacketData::Unknown => 0,
            PacketData::Init(_) => 1,
            PacketData::Player(_) => 2,
            PacketData::Cap(_) => 3,
            PacketData::Game(_) => 4,
            PacketData::Tag => 5,
            PacketData::Connect(_) => 6,
            PacketData::Disconnect => 7,
            PacketData::Costume(_) => 8,
            PacketData::Shine(_) => 9,
            PacketData::Capture(_) => 10,
            PacketData::ChangeStage(_) => 11,
        }
    }
}

impl PacketBytes for PacketData {
    fn write_bytes(&self, buf: &mut BytesMut) -> usize {
        match self {
            // Do nothing
            PacketData::Unknown | PacketData::Tag | PacketData::Disconnect => 0,

            PacketData::Init(packet) => packet.write_bytes(buf),
            PacketData::Player(packet) => packet.write_bytes(buf),
            PacketData::Cap(packet) => packet.write_bytes(buf),
            PacketData::Game(packet) => packet.write_bytes(buf),
            PacketData::Connect(packet) => packet.write_bytes(buf),
            PacketData::Costume(packet) => packet.write_bytes(buf),
            PacketData::Shine(packet) => packet.write_bytes(buf),
            PacketData::Capture(packet) => packet.write_bytes(buf),
            PacketData::ChangeStage(packet) => packet.write_bytes(buf),
        }
    }

    fn from_bytes<T: Buf>(_: &mut T) -> Result<Self> {
        unimplemented!()
    }
}
// endregion

// region: PartialPacket
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartialPacket {
    pub id: Uuid,
    pub packet_id: u16,
    pub body_length: u16,
}

impl PartialPacket {
    #[inline]
    pub fn upgrade<T: Buf>(self, buf: &mut T) -> Result<Packet> {
        Packet::read_body(self.id, self.packet_id, buf)
    }
}

impl PacketBytes for PartialPacket {
    fn write_bytes(&self, _: &mut BytesMut) -> usize {
        unimplemented!()
    }

    fn from_bytes<T: Buf>(buf: &mut T) -> Result<Self> {
        let id = <Uuid as PacketBytes>::from_bytes(buf)?;
        let packet_id = u16::from_bytes(buf)?;
        let body_length = u16::from_bytes(buf)?;

        let partial = Self {
            id,
            packet_id,
            body_length,
        };

        Ok(partial)
    }
}
// endregion
