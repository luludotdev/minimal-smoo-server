use std::fmt::Debug;
use std::net::SocketAddr;

use futures::SinkExt;
use uuid::Uuid;

use crate::packet::{IntoPacket, Packet};
use crate::server::Sink;

pub struct Peer {
    pub id: Uuid,
    addr: SocketAddr,
    sink: Sink,
}

impl Peer {
    pub fn new(sink: Sink, addr: SocketAddr) -> Self {
        Self {
            id: Uuid::nil(),
            addr,
            sink,
        }
    }

    #[inline]
    pub async fn send(&mut self, packet: Packet) {
        let _ = self.sink.send(packet).await;
    }

    pub async fn send_nil_uuid<T: IntoPacket>(&mut self, packet: T) {
        let data = packet.into();
        let packet = Packet {
            id: Uuid::nil(),
            data,
        };

        let _ = self.sink.send(packet).await;
    }

    #[inline]
    pub async fn disconnect(&mut self) {
        // TODO: Handle error
        let _ = self.sink.close().await;
    }
}

impl Debug for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Peer").field("addr", &self.addr).finish()
    }
}
