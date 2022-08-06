use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;

use color_eyre::Result;
use futures::stream::{SplitSink, SplitStream};
use futures::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_util::codec::Framed;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::config::SharedConfig;
use crate::packet::{InitPacket, Packet, PacketCodec, PacketData};
use crate::peer::Peer;
use crate::player::Player;
use crate::players::Players;
use crate::Args;

pub type Sink = SplitSink<Framed<TcpStream, PacketCodec>, Packet>;
pub type Stream = SplitStream<Framed<TcpStream, PacketCodec>>;

#[derive(Debug)]
pub struct Server {
    addr: SocketAddr,
    config: SharedConfig,

    peers: RwLock<HashMap<Uuid, Peer>>,
    players: Players,
    shines: RwLock<HashSet<i32>>,
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

        let server = Self {
            addr,
            config,
            peers: Default::default(),
            players: Default::default(),
            shines: Default::default(),
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

        let connect_data = match connect_packet.data {
            PacketData::Connect(data) => data,
            _ => {
                // First packet must be connect packet
                return Ok(());
            }
        };

        // TODO: Max players check

        // Insert peer into server state
        {
            let mut peers = self.peers.write().await;

            // Disconnect previous connections for this player
            if let Some(mut stale) = peers.remove(&connect_packet.id) {
                stale.disconnect().await;
            }

            peer.id = connect_packet.id;
            peers.insert(connect_packet.id, peer);
        }

        // Insert player into server state
        match self.players.get(&connect_packet.id).await {
            Some(player) => {
                // Reconnect
                let player = player.read().await;
                info!("{player} reconnected");
            }

            None => {
                // First connect
                let name = connect_data.nickname.try_to_string()?;
                let player = Player::new(connect_packet.id, name);

                info!("{player} connected");
                let _ = self.players.insert(player).await;
            }
        }

        // TODO: Broadcast connect and costume packet
        // TODO: Impl broadcast and broadcast_bg

        while let Some(packet) = stream.next().await {
            let packet = packet?;
            // TODO: Handle packets
        }

        Ok(())
    }
}
