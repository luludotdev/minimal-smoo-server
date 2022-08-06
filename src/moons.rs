use std::collections::{BTreeMap, HashSet};

use color_eyre::Result;
use tokio::fs;

use crate::config::SharedConfig;

pub type MoonMap = BTreeMap<i32, bool>;

#[derive(Debug)]
pub struct Moons {
    map: MoonMap,
    config: SharedConfig,
}

impl Moons {
    #[inline]
    pub async fn insert(&mut self, id: i32, is_grand: bool) -> Result<()> {
        self.map.insert(id, is_grand);
        self.save().await
    }

    pub fn difference(&self, other: &MoonMap) -> MoonMap {
        let keys = self.map.keys().collect::<HashSet<_>>();
        let other_keys = other.keys().collect::<HashSet<_>>();

        keys.difference(&other_keys)
            .map(|key| (**key, *self.map.get(key).unwrap()))
            .collect()
    }

    // region: Persistence
    pub async fn load(config: SharedConfig) -> Result<Self> {
        let cfg = config.read().await;
        let map: MoonMap = if !cfg.moons.persist {
            Default::default()
        } else {
            let path = &cfg.moons.persist_file;
            if !cfg.moons.persist_file.exists() {
                Default::default()
            } else {
                let body = fs::read(path).await?;
                serde_json::from_slice(&body)?
            }
        };

        drop(cfg);
        let moons = Self { map, config };
        moons.save().await?;

        Ok(moons)
    }

    async fn save(&self) -> Result<()> {
        let cfg = self.config.read().await;
        let path = &cfg.moons.persist_file;

        if cfg.moons.persist {
            let json = serde_json::to_string_pretty(&self.map)?;
            fs::write(path, &json).await?;
        }

        Ok(())
    }
    // endregion
}
