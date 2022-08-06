use std::collections::HashMap;

use uuid::Uuid;

use crate::player::Player;

#[derive(Debug, Default)]
pub struct Players {
    map: HashMap<Uuid, Player>,
}

impl Players {
    #[inline]
    pub fn get(&self, id: &Uuid) -> Option<&Player> {
        self.map.get(id)
    }

    #[inline]
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Player> {
        self.map.get_mut(id)
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
