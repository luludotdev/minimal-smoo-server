use std::net::SocketAddr;

use color_eyre::Result;
use futures::StreamExt;
use tokio::net::TcpListener;
use tokio_util::codec::Framed;
use tracing::info;

use crate::config::SharedConfig;
use crate::packet::PacketCodec;
use crate::Args;

pub struct Server {
    addr: SocketAddr,
    config: SharedConfig,
}

impl Server {
    pub async fn new(args: &Args, config: SharedConfig) -> Self {
        let addr = {
            let config = config.read().await;

            let port = args.port.or_else(|| config.server.port()).unwrap_or(1027);
            let host = args
                .host
                .or_else(|| config.server.host())
                .unwrap_or_else(|| "127.0.0.1".parse().unwrap());

            SocketAddr::from((host, port))
        };

        // TODO: implement socket
        Self { addr, config }
    }

    pub async fn listen(&self) -> Result<()> {
        info!(addr = %self.addr, "Server listening");
        let listener = TcpListener::bind(self.addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?;

            tokio::spawn(async move {
                let mut frames = Framed::new(stream, PacketCodec);

                let packet = frames.next().await;
                dbg!(packet);
            });
        }
    }
}
