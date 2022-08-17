use std::sync::Arc;

use color_eyre::eyre::bail;
use color_eyre::Result;
use tracing::info;

use super::commands::{Command, MoonCommand};
use crate::server::Server;

pub(super) async fn handle_command(command: Command, server: Arc<Server>) -> Result<HandleResult> {
    match command {
        Command::Exit => Ok(HandleResult::Exit),

        Command::Moon(MoonCommand::List) => {
            // TODO
            bail!("not yet implemented")
        }

        Command::Moon(MoonCommand::Sync) => {
            server.sync_moons().await?;
            info!("Synced moons to all players");

            Ok(HandleResult::Ok)
        }

        Command::Moon(MoonCommand::Send { id, player }) => {
            // TODO
            bail!("not yet implemented")
        }

        // TODO
        _ => Ok(HandleResult::Ok),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum HandleResult {
    Ok,
    Exit,
}
