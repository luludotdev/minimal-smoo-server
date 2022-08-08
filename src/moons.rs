use std::collections::{BTreeMap, HashSet};

use color_eyre::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::config::SharedConfig;

pub type MoonMap = BTreeMap<i32, bool>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Moons {
    #[serde(rename = "moons")]
    map: MoonMap,

    #[serde(skip)]
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
        let mut moons: Self = {
            let cfg = config.read().await;
            if cfg.moons.persist {
                let path = &cfg.moons.persist_file;
                if cfg.moons.persist_file.exists() {
                    let body = fs::read(path).await?;
                    toml::from_slice(&body)?
                } else {
                    Moons::default()
                }
            } else {
                Moons::default()
            }
        };

        moons.config = config;
        moons.save().await?;

        Ok(moons)
    }

    async fn save(&self) -> Result<()> {
        let cfg = self.config.read().await;
        let path = &cfg.moons.persist_file;

        if cfg.moons.persist {
            let body = toml::to_string_pretty(&self)?;
            fs::write(path, &body).await?;
        }

        Ok(())
    }
    // endregion
}
