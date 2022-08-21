use std::collections::BTreeSet;

use color_eyre::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::config::SharedConfig;

pub type MoonMap = BTreeSet<i32>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Moons {
    #[serde(rename = "moons")]
    map: MoonMap,

    #[serde(skip)]
    config: SharedConfig,
}

impl Moons {
    pub async fn insert(&mut self, id: i32) -> Result<()> {
        self.map.insert(id);
        self.save().await
    }

    #[inline]
    pub fn difference(&self, other: &MoonMap) -> MoonMap {
        self.map.difference(other).copied().collect()
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

    pub async fn reload(&mut self) -> Result<()> {
        let moons = Self::load(self.config.clone()).await?;
        *self = moons;

        Ok(())
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
