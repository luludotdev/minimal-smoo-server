use std::net::SocketAddr;

use tracing::info;

use crate::config::SharedConfig;
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

    pub fn listen(&self) {
        info!(addr = %self.addr, "Server listening");
        todo!()
    }
}
