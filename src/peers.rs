use std::collections::{HashMap, HashSet};

use color_eyre::eyre::eyre;
use color_eyre::Result;
use futures::future::join_all;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use crate::packet::Packet;
use crate::peer::Peer;
use crate::players::Players;

#[derive(Debug, Default)]
pub struct Peers {
    map: HashMap<Uuid, Peer>,
}

impl Peers {
    #[inline]
    pub fn count(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn get_mut(&mut self, id: &Uuid) -> Result<&mut Peer> {
        self.map
            .get_mut(id)
            .ok_or_else(|| eyre!("peer should exist in the map"))
    }

    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.map.keys().copied()
    }

    #[inline]
    pub fn insert(&mut self, id: Uuid, peer: Peer) -> Option<Peer> {
        self.map.insert(id, peer)
    }

    pub async fn remove(&mut self, id: &Uuid, players: &RwLock<Players>) -> Option<Peer> {
        let peer = self.map.remove(id);
        let peer = match peer {
            Some(mut peer) => {
                peer.disconnect().await;
                Some(peer)
            }

            None => peer,
        };

        let mut players = players.write().await;
        if let Some(player) = players.remove(id) {
            info!("{player} disconnected");
        };

        peer
    }

    pub async fn broadcast(&mut self, packet: Packet) {
        let sender = packet.id;
        let jobs =
            self.map
                .iter_mut()
                .filter(|(id, _)| **id != sender)
                .map(|(_, peer)| async move {
                    peer.send(packet).await;
                });

        join_all(jobs).await;
    }

    pub async fn broadcast_some(&mut self, packet: Packet, players: HashSet<Uuid>) {
        let sender = packet.id;
        let jobs = self
            .map
            .iter_mut()
            .filter(|(id, _)| **id != sender)
            .filter(|(id, _)| players.contains(id))
            .map(|(_, peer)| async move {
                peer.send(packet).await;
            });

        join_all(jobs).await;
    }
}
