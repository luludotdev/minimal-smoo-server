use std::collections::HashSet;
use std::net::IpAddr;
use std::num::NonZeroU8;
use std::path::{Path, PathBuf};

use color_eyre::eyre::Context;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;
use uuid::Uuid;

// region: Config
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub moons: MoonConfig,
    pub costumes: CostumesConfig,
}

impl Config {
    #[inline(always)]
    fn path_buf() -> PathBuf {
        PathBuf::from("./config.toml")
    }

    pub async fn load() -> Result<Self> {
        let path = Self::path_buf();
        if !path.exists() {
            let config = Self::load_default().await?;
            return Ok(config);
        }

        let bytes = fs::read(&path).await.context("failed to read config")?;
        let result = toml::from_slice::<Config>(&bytes);

        match result {
            Ok(config) => Ok(config),
            Err(_) => {
                let config = Self::load_default().await?;
                Ok(config)
            }
        }
    }

    async fn load_default() -> Result<Self> {
        let config = Self::default();
        config.save().await?;

        Ok(config)
    }

    pub async fn save(&self) -> Result<()> {
        let path = Self::path_buf();
        let serialized = toml::to_string_pretty(&self)?;

        fs::write(path, serialized)
            .await
            .context("failed to write config")?;

        Ok(())
    }
}
// endregion

// region: ServerConfig
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    host: Option<IpAddr>,
    port: Option<u16>,

    pub max_players: NonZeroU8,
}

impl Default for ServerConfig {
    #[inline]
    fn default() -> Self {
        Self {
            host: Default::default(),
            port: Default::default(),
            max_players: NonZeroU8::new(8).unwrap(),
        }
    }
}

impl ServerConfig {
    #[inline]
    pub fn host(&self) -> Option<IpAddr> {
        self.host
    }

    #[inline]
    pub fn port(&self) -> Option<u16> {
        self.port
    }
}
// endregion

// region: MoonConfig
#[derive(Debug, Deserialize, Serialize)]
pub struct MoonConfig {
    pub persist: bool,
    persist_file: PathBuf,
}

impl Default for MoonConfig {
    #[inline]
    fn default() -> Self {
        Self {
            persist: Default::default(),
            persist_file: PathBuf::from("./moons.json"),
        }
    }
}

impl MoonConfig {
    #[inline]
    pub fn persist_file(&self) -> &Path {
        &self.persist_file
    }
}
// endregion

// region: CostumesConfig
#[derive(Debug, Deserialize, Serialize)]
pub struct CostumesConfig {
    pub banned_costumes: HashSet<String>,
    pub allowed_players: HashSet<Uuid>,
}

impl Default for CostumesConfig {
    #[inline]
    fn default() -> Self {
        Self {
            banned_costumes: HashSet::from(["MarioInvisible".to_owned()]),
            allowed_players: Default::default(),
        }
    }
}

impl CostumesConfig {
    #[inline]
    pub fn is_banned(&self, costume: &str) -> bool {
        self.banned_costumes.contains(costume)
    }

    #[inline]
    pub fn is_allowed(&self, id: &Uuid) -> bool {
        self.allowed_players.contains(id)
    }
}
// endregion
