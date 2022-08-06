use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::player::Player;

type SharedPlayer = Arc<RwLock<Player>>;

#[derive(Debug, Default)]
pub struct Players {
    map: RwLock<HashMap<Uuid, SharedPlayer>>,
}

impl Players {
    #[inline]
    pub async fn get(&self, id: &Uuid) -> Option<SharedPlayer> {
        let players = self.map.read().await;
        players.get(id).cloned()
    }

    pub async fn insert(&self, player: Player) -> SharedPlayer {
        let id = player.id;
        let player = Arc::new(RwLock::new(player));

        let mut players = self.map.write().await;
        players.insert(id, player.clone());

        player
    }
}
