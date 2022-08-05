mod fixed_string;
mod header;
mod traits;
mod types;

mod cap_packet;
mod capture_packet;
mod change_stage_packet;
mod connect_packet;
mod costume_packet;
mod game_packet;
mod init_packet;
mod player_packet;
mod shine_packet;

pub use cap_packet::CapPacket;
pub use capture_packet::CapturePacket;
pub use change_stage_packet::ChangeStagePacket;
pub use connect_packet::{ConnectPacket, ConnectionType};
pub use costume_packet::CostumePacket;
pub use game_packet::GamePacket;
pub use header::*;
pub use init_packet::InitPacket;
pub use player_packet::PlayerPacket;
pub use shine_packet::ShinePacket;
pub use traits::*;

#[cfg(test)]
mod tests {
    use glam::{EulerRot, Quat, Vec3};
    use uuid::Uuid;

    use super::*;

    macro_rules! test_packet {
        ($data:ident) => {{
            let id = Uuid::new_v4();
            let packet1 = $data.into_header(id);

            let buf = packet1.to_bytes();
            let packet2 = PacketHeader::from_bytes(buf).unwrap();

            assert_eq!(packet1, packet2)
        }};
    }

    #[test]
    fn test_cap_packet() {
        let data = CapPacket {
            position: Vec3 {
                x: 10.0,
                y: -4.0,
                z: 6.9,
            },
            quaternion: Quat::from_euler(EulerRot::XYZ, 10.0, -5.0, 69.420),
            cap_out: true,
            cap_anim: "animation".parse().unwrap(),
        };

        test_packet!(data)
    }

    #[test]
    fn test_capture_packet() {
        let data = CapturePacket {
            model: "NutBoy".parse().unwrap(),
        };

        test_packet!(data)
    }

    #[test]
    fn test_change_stage_packet() {
        let data = ChangeStagePacket {
            id: "Cap".parse().unwrap(),
            stage: "CapKingdom".parse().unwrap(),
            scenario: 127,
            sub_scenario: 3,
        };

        test_packet!(data)
    }

    #[test]
    fn test_connect_packet() {
        let data = ConnectPacket {
            connection_type: ConnectionType::Init,
            max_players: 8,
            nickname: "Lulu".parse().unwrap(),
        };

        test_packet!(data)
    }

    #[test]
    fn test_costume_packet() {
        let data = CostumePacket {
            body: "Mario".parse().unwrap(),
            cap: "MarioKing".parse().unwrap(),
        };

        test_packet!(data)
    }

    #[test]
    fn test_game_packet() {
        let data = GamePacket {
            is_2d: false,
            scenario: 255,
            stage: "MoonKingdom".parse().unwrap(),
        };

        test_packet!(data)
    }

    #[test]
    fn test_init_packet() {
        let data = InitPacket { max_players: 8 };

        test_packet!(data)
    }

    #[test]
    fn test_player_packet() {
        let data = PlayerPacket {
            position: Vec3 {
                x: 450.0,
                y: -34.0,
                z: 6564.9,
            },
            quaternion: Quat::from_euler(EulerRot::YZX, -10.324, 5342.0, -69.420),
            animation_blend_weights: [0.0, 1.1, 2.2, 3.3, 4.4, 5.5],
            act: 7,
            subact: 77,
        };

        test_packet!(data)
    }

    #[test]
    fn test_shine_packet() {
        let data = ShinePacket {
            id: 69,
            is_grand: false,
        };

        test_packet!(data)
    }
}
