use std::borrow::ToOwned;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use flume::{Receiver, Sender};
use futures::stream::{SplitSink, SplitStream};
use futures::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_util::codec::Framed;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::config::SharedConfig;
use crate::packet::{
    ConnectPacket, ConnectionType, CostumePacket, InitPacket, IntoPacket, Packet, PacketCodec,
    PacketData,
};
use crate::peer::Peer;
use crate::peers::Peers;
use crate::player::Player;
use crate::players::Players;
use crate::Args;

pub type Sink = SplitSink<Framed<TcpStream, PacketCodec>, Packet>;
pub type Stream = SplitStream<Framed<TcpStream, PacketCodec>>;

#[derive(Debug)]
pub struct Server {
    addr: SocketAddr,
    config: SharedConfig,

    peers: RwLock<Peers>,
    players: RwLock<Players>,
    // shines: RwLock<HashSet<i32>>,
    process_tx: Sender<(Uuid, Packet)>,
    process_rx: Receiver<(Uuid, Packet)>,
}

#[derive(Debug, Clone, Copy)]
pub enum ReplyType {
    /// Invalid, disconnect peer
    Invalid,

    /// Don't reply
    None,

    /// Reply to the peer that sent the message
    Reply(Packet),

    /// Broadcast the reply to everyone except the sender
    Broadcast(Packet),
}

impl Server {
    pub async fn new(args: &Args, config: SharedConfig) -> Arc<Self> {
        let addr = {
            let config = config.read().await;

            let port = args.port.or_else(|| config.server.port()).unwrap_or(1027);
            let host = args
                .host
                .or_else(|| config.server.host())
                .unwrap_or_else(|| "127.0.0.1".parse().unwrap());

            SocketAddr::from((host, port))
        };

        let (p_tx, p_rx) = flume::unbounded();
        let server = Self {
            addr,
            config,
            peers: Default::default(),
            players: Default::default(),
            // shines: Default::default(),
            process_tx: p_tx,
            process_rx: p_rx,
        };

        Arc::new(server)
    }

    pub async fn listen(self: Arc<Self>) -> Result<()> {
        info!(addr = %self.addr, "Server listening");
        let listener = TcpListener::bind(self.addr).await?;

        loop {
            let server = self.clone();
            let (stream, addr) = listener.accept().await?;

            // TODO: Handle error
            let _ = stream.set_nodelay(true);

            debug!(?addr, "accepted");

            tokio::spawn(async move {
                let (sink, stream) = Framed::new(stream, PacketCodec).split();
                let peer = Peer::new(sink, addr);

                if let Err(error) = server.handle_connection(stream, peer).await {
                    error!(%error);
                }

                debug!(?addr, "closed");
            });
        }
    }

    pub async fn process_packets(self: Arc<Self>) {
        while let Ok((id, packet)) = self.process_rx.recv_async().await {
            let reply = match self.process_packet(id, packet).await {
                Ok(reply) => reply,

                Err(error) => {
                    error!(%error);
                    continue;
                }
            };

            match reply {
                ReplyType::Invalid => break,
                ReplyType::None => (),

                ReplyType::Reply(packet) => {
                    let mut peers = self.peers.write().await;
                    let peer = match peers.get_mut(&id) {
                        Some(peer) => peer,
                        None => {
                            error!("peer should exist in the map");
                            continue;
                        }
                    };

                    peer.send(packet).await;
                }

                ReplyType::Broadcast(packet) => {
                    let mut peers = self.peers.write().await;
                    peers.broadcast(packet).await;
                }
            }
        }
    }

    async fn handle_connection(self: Arc<Self>, mut stream: Stream, mut peer: Peer) -> Result<()> {
        let max_players = {
            let config = self.config.read().await;
            u16::from(config.server.max_players.get())
        };

        let init = InitPacket { max_players };
        peer.send_nil_uuid(init).await;

        let connect_packet = match stream.next().await {
            Some(packet) => packet?,
            None => return Ok(()),
        };

        let id = connect_packet.id;
        let connect_data = match connect_packet.data {
            PacketData::Connect(data) => data,
            _ => {
                // First packet must be connect packet
                return Ok(());
            }
        };

        // TODO: Max players check

        // Send state of existing players
        {
            let mut peers = self.peers.write().await;
            let players = self.players.read().await;

            for uuid in peers.keys() {
                if uuid == id {
                    continue;
                }

                let player = match players.get(&uuid) {
                    Some(player) => player,
                    None => continue,
                };

                let packet = ConnectPacket {
                    connection_type: ConnectionType::Init,
                    max_players,
                    nickname: player.name.clone().parse()?,
                };

                let packet = packet.into_packet(player.id);
                peer.send(packet).await;

                if let Some(costume) = &player.costume {
                    let costume_packet: CostumePacket = costume.clone().try_into()?;
                    let costume_packet = costume_packet.into_packet(player.id);

                    peer.send(costume_packet).await;
                }
            }

            // Insert peer into server state
            peers.insert(id, peer);
        }

        // Insert player into server state
        {
            let mut players = self.players.write().await;
            match players.get(&connect_packet.id) {
                Some(player) => {
                    // Reconnect
                    info!("{player} reconnected");
                }

                None => {
                    // First connect
                    let name = connect_data.nickname.try_to_string()?;
                    let player = Player::new(connect_packet.id, name);

                    info!("{player} connected");
                    let _ = players.insert(id, player);
                }
            }
        }

        // Broadcast connect and costume packets to other clients in the background
        {
            let mut peers = self.peers.write().await;
            peers.broadcast(connect_packet).await;

            let players = self.players.read().await;
            let player = players
                .get(&connect_packet.id)
                .ok_or_else(|| eyre!("player should exist in the map"))?;

            if let Some(costume) = &player.costume {
                let costume_packet: CostumePacket = costume.clone().try_into()?;
                let costume_packet = costume_packet.into_packet(connect_packet.id);

                peers.broadcast(costume_packet).await;
            }
        }

        while let Some(packet) = stream.next().await {
            let packet = packet?;
            self.process_tx.send((id, packet))?;
        }

        let disconnect_packet = Packet {
            id,
            data: PacketData::Disconnect,
        };

        let mut peers = self.peers.write().await;
        peers.remove(&id).await;
        peers.broadcast(disconnect_packet).await;

        Ok(())
    }

    async fn process_packet(&self, id: Uuid, packet: Packet) -> Result<ReplyType> {
        let reply = match &packet.data {
            PacketData::Disconnect | PacketData::Init(_) => ReplyType::Invalid,

            PacketData::Game(data) => {
                let mut players = self.players.write().await;
                let player = players
                    .get_mut(&id)
                    .ok_or_else(|| eyre!("player should exist in the map"))?;

                player.scenario = Some(data.scenario);
                player.is_2d = data.is_2d;
                player.last_game = Some(*data);

                // Send the position of all players when a player join a stage
                // If we don't do so, people are gonna be invisible or to their previous position until they move
                let mut peers = self.peers.write().await;
                let peer = peers.get_mut(&id);

                if let Some(peer) = peer {
                    let players = players.get_all();
                    let positions = players.into_iter().map(|player| {
                        let stage = player.stage().map(ToOwned::to_owned);
                        let pos = player.last_pos.as_ref().copied();

                        (stage, player.id, pos)
                    });

                    let self_stage = data.stage.try_as_str()?;
                    for (stage, id, last_position) in positions {
                        if let (Some(player_stage), Some(last_packet)) = (stage, last_position) {
                            if player_stage == self_stage {
                                let packet = last_packet.into_packet(id);
                                peer.send(packet).await;
                            }
                        }
                    }
                }

                ReplyType::Broadcast(packet)
            }

            PacketData::Costume(data) => {
                let mut players = self.players.write().await;
                let player = players
                    .get_mut(&id)
                    .ok_or_else(|| eyre!("player should exist in the map"))?;

                player.loaded = true;
                player.set_costume(*data)?;

                // TODO: Sync shines

                // TODO: Banned costumes

                ReplyType::Broadcast(packet)
            }

            // PacketData::Shine(_) => todo!(),

            // Broadcast as-is
            PacketData::Player(_)
            | PacketData::Cap(_)
            | PacketData::Capture(_)
            | PacketData::ChangeStage(_) => ReplyType::Broadcast(packet),

            _ => ReplyType::None,
        };

        Ok(reply)
    }
}
