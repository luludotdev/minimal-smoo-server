use std::sync::Arc;

use clap::Parser;
use color_eyre::Result;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Helper};
use tracing::error;

use super::commands::Command;
use super::handler::{handle_command, HandleResult};
use crate::config::SharedConfig;
use crate::server::Server;

pub async fn read_loop<H: Helper>(
    mut rl: Editor<H>,
    server: Arc<Server>,
    config: SharedConfig,
) -> Result<()> {
    loop {
        match rl.readline("> ") {
            Ok(line) => {
                rl.add_history_entry(&line);

                let args = line.split(' ');
                let command = match Command::try_parse_from(args) {
                    Ok(command) => command,
                    Err(_) => {
                        error!("Invalid command!");
                        continue;
                    }
                };

                match handle_command(command, server.clone(), config.clone()).await {
                    Ok(HandleResult::Ok) => (),
                    Ok(HandleResult::Exit) => break,

                    Err(error) => {
                        error!(
                            "An error occurred while processing that command\n{:?}",
                            error
                        );

                        continue;
                    }
                };
            }

            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(err) => return Err(err.into()),
        }
    }

    tracing::info!("Exiting...");
    std::process::exit(0);
}
