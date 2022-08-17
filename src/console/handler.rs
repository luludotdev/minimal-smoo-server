use std::sync::Arc;

use color_eyre::eyre::bail;
use color_eyre::Result;
use tracing::info;

use super::commands::{Command, MoonCommand};
use crate::server::Server;

pub(super) async fn handle_command(command: Command, server: Arc<Server>) -> Result<HandleResult> {
    match command {
        Command::Exit => Ok(HandleResult::Exit),

        Command::LoadConfig => {
            // TODO
            bail!("not yet implemented")
        }

        Command::List => {
            // TODO
            bail!("not yet implemented")
        }

        Command::Send {
            stage,
            scenario,
            warp_id,
            players,
        } => {
            // TODO
            bail!("not yet implemented")
        }

        Command::SendAll {
            stage,
            scenario,
            warp_id,
        } => {
            // TODO
            bail!("not yet implemented")
        }

        Command::Moon(MoonCommand::List) => {
            // TODO
            bail!("not yet implemented")
        }

        Command::Moon(MoonCommand::Sync) => {
            server.sync_moons().await?;
            info!("Synced moons to all players");

            Ok(HandleResult::Ok)
        }

        Command::Moon(MoonCommand::Give { id, players }) => {
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
