use std::fmt::Debug;
use std::net::SocketAddr;

use futures::SinkExt;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::packet::{IntoPacket, Packet};
use crate::server::Sink;

pub struct Peer {
    pub id: Uuid,
    addr: SocketAddr,
    sink: Mutex<Sink>,
}

impl Peer {
    pub fn new(sink: Sink, addr: SocketAddr) -> Self {
        Self {
            id: Uuid::nil(),
            addr,
            sink: Mutex::new(sink),
        }
    }

    #[inline]
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    #[inline]
    pub async fn send(&self, packet: Packet) {
        let mut sink = self.sink.lock().await;
        let _ = sink.send(packet).await;
    }

    pub async fn send_nil_uuid<T: IntoPacket>(&mut self, packet: T) {
        let data = packet.into();
        let packet = Packet {
            id: Uuid::nil(),
            data,
        };

        let mut sink = self.sink.lock().await;
        let _ = sink.send(packet).await;
    }

    #[inline]
    pub async fn disconnect(&mut self) {
        let mut sink = self.sink.lock().await;

        // TODO: Handle error
        let _ = sink.close().await;
    }
}

impl Debug for Peer {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Peer").field("addr", &self.addr).finish()
    }
}
