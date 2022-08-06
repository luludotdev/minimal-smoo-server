use std::collections::HashMap;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use uuid::Uuid;

use crate::player::Player;

#[derive(Debug, Default)]
pub struct Players {
    map: HashMap<Uuid, Player>,
}

impl Players {
    #[inline]
    pub fn get(&self, id: &Uuid) -> Result<&Player> {
        self.map
            .get(id)
            .ok_or_else(|| eyre!("player should exist in the map"))
    }

    #[inline]
    pub fn get_mut(&mut self, id: &Uuid) -> Result<&mut Player> {
        self.map
            .get_mut(id)
            .ok_or_else(|| eyre!("player should exist in the map"))
    }

    #[inline]
    pub fn insert(&mut self, id: Uuid, player: Player) -> Option<Player> {
        self.map.insert(id, player)
    }

    #[inline]
    pub fn get_all(&self) -> Vec<&Player> {
        self.map.values().collect::<Vec<_>>()
    }
}
