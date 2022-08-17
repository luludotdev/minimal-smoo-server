use color_eyre::Result;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Helper};

#[allow(clippy::unused_async)]
pub async fn read_loop<H: Helper>(mut rl: Editor<H>) -> Result<()> {
    loop {
        match rl.readline("> ") {
            Ok(line) => {
                dbg!(line);
            }

            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                tracing::info!("Exiting...");
                std::process::exit(0);
            }

            Err(err) => return Err(err.into()),
        }
    }
}
