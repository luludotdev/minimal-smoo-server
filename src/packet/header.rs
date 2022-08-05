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
pub struct PacketHeader {
    pub id: Uuid,
    pub packet: PacketType,
}

impl PacketHeader {
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

    fn read_body<T: Buf>(id: Uuid, packet_id: u16, buf: &mut T) -> Result<PacketHeader> {
        match packet_id {
            1 => {
                let packet = InitPacket::from_bytes(buf)?;
                Ok(PacketHeader {
                    id,
                    packet: packet.into(),
                })
            }

            2 => {
                let packet = PlayerPacket::from_bytes(buf)?;
                Ok(PacketHeader {
                    id,
                    packet: packet.into(),
                })
            }

            3 => {
                let packet = CapPacket::from_bytes(buf)?;
                Ok(PacketHeader {
                    id,
                    packet: packet.into(),
                })
            }

            4 => {
                let packet = GamePacket::from_bytes(buf)?;
                Ok(PacketHeader {
                    id,
                    packet: packet.into(),
                })
            }

            5 => Ok(PacketHeader {
                id,
                packet: PacketType::Tag,
            }),

            6 => {
                let packet = ConnectPacket::from_bytes(buf)?;
                Ok(PacketHeader {
                    id,
                    packet: packet.into(),
                })
            }

            7 => Ok(PacketHeader {
                id,
                packet: PacketType::Disconnect,
            }),

            8 => {
                let packet = CostumePacket::from_bytes(buf)?;
                Ok(PacketHeader {
                    id,
                    packet: packet.into(),
                })
            }

            9 => {
                let packet = ShinePacket::from_bytes(buf)?;
                Ok(PacketHeader {
                    id,
                    packet: packet.into(),
                })
            }

            10 => {
                let packet = CapturePacket::from_bytes(buf)?;
                Ok(PacketHeader {
                    id,
                    packet: packet.into(),
                })
            }

            11 => {
                let packet = ChangeStagePacket::from_bytes(buf)?;
                Ok(PacketHeader {
                    id,
                    packet: packet.into(),
                })
            }

            _ => Ok(PacketHeader {
                id,
                packet: PacketType::Unknown,
            }),
        }
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

    fn from_bytes<T: Buf>(buf: &mut T) -> Result<Self> {
        let id = <Uuid as PacketBytes>::from_bytes(buf)?;
        let packet_id = u16::from_bytes(buf)?;

        // Packet length
        let _ = u16::from_bytes(buf)?;
        Self::read_body(id, packet_id, buf)
    }
}

impl From<PacketHeader> for Bytes {
    #[inline]
    fn from(header: PacketHeader) -> Self {
        header.to_bytes()
    }
}

impl TryFrom<Bytes> for PacketHeader {
    type Error = Report;

    #[inline]
    fn try_from(buf: Bytes) -> Result<Self, Self::Error> {
        Self::from_bytes(buf)
    }
}
// endregion

// region: PacketType
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn id(&self) -> u16 {
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
    pub fn upgrade<T: Buf>(self, buf: &mut T) -> Result<PacketHeader> {
        PacketHeader::read_body(self.id, self.packet_id, buf)
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
