use smoo_derive::Packet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Packet)]
#[packet("Moon")]
pub struct MoonPacket {
    pub id: i32,
    pub is_grand: bool,
}
