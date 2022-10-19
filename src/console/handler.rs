use std::sync::Arc;

use color_eyre::eyre::bail;
use color_eyre::Result;
use tracing::{info, warn};
use uuid::Uuid;

use super::commands::{Command, ConfigCommand, MoonCommand};
use crate::config::SharedConfig;
use crate::packet::{ChangeStagePacket, IntoPacket};
use crate::server::Server;

pub(super) async fn handle_command(
    command: Command,
    server: Arc<Server>,
    config: SharedConfig,
) -> Result<HandleResult> {
    match command {
        Command::Exit => Ok(HandleResult::Exit),

        Command::Config(ConfigCommand::Reload) => {
            let mut config = config.write().await;

            config.reload().await?;
            info!("Loaded config from file");

            Ok(HandleResult::Ok)
        }

        Command::Config(ConfigCommand::Save) => {
            let config = config.read().await;

            config.save().await?;
            info!("Force saved config to file");

            Ok(HandleResult::Ok)
        }

        Command::List => {
            let players = server.list_players().await;
            info!(?players);

            Ok(HandleResult::Ok)
        }

        Command::Send {
            stage,
            scenario,
            warp_id,
            players,
        } => {
            let resolved = server.resolve_players(players).await;
            if resolved.is_empty() {
                warn!("No players selected! (Use * to select all players)");
                return Ok(HandleResult::Ok);
            }

            let packet = ChangeStagePacket {
                stage: stage.stage_name_fixed(),
                id: warp_id.parse()?,
                scenario,
                sub_scenario: 0,
            };

            let packet = packet.into_packet(Uuid::nil());
            server.broadcast_some(packet, resolved).await?;

            Ok(HandleResult::Ok)
        }

        Command::SendAll {
            stage,
            scenario,
            warp_id,
        } => {
            let packet = ChangeStagePacket {
                stage: stage.stage_name_fixed(),
                id: warp_id.parse()?,
                scenario,
                sub_scenario: 0,
            };

            let packet = packet.into_packet(Uuid::nil());
            server.broadcast(packet).await?;

            Ok(HandleResult::Ok)
        }

        Command::Moon(MoonCommand::List) => {
            // TODO
            bail!("not yet implemented")
        }

        Command::Moon(MoonCommand::Sync) => {
            server.sync_moons().await?;
            info!("Synced current moon state to all players");

            Ok(HandleResult::Ok)
        }

        Command::Moon(MoonCommand::Reload) => {
            let persist_moons = {
                let config = config.read().await;
                config.moons.persist
            };

            if !persist_moons {
                warn!("Moon persistence is disabled!");
                return Ok(HandleResult::Ok);
            }

            server.reload_moons().await?;
            info!("Reloaded moons from file");

            Ok(HandleResult::Ok)
        }

        Command::Moon(MoonCommand::Clear) => {
            let persist_moons = {
                let config = config.read().await;
                config.moons.persist
            };

            if !persist_moons {
                warn!("Moon persistence is disabled, only clearing moons in-memory.");
            }

            server.clear_moons().await?;
            info!("Cleared moons");

            Ok(HandleResult::Ok)
        }

        Command::Moon(MoonCommand::Add { id }) => {
            // TODO
            bail!("not yet implemented")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum HandleResult {
    Ok,
    Exit,
}
