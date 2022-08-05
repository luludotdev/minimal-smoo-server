use clap::Parser;
use color_eyre::Result;
use once_cell::sync::Lazy;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

mod config;

static VERSION: Lazy<String> = Lazy::new(|| {
    let mut version = format!("v{}", env!("CARGO_PKG_VERSION"));
    if let Some(hash) = option_env!("GIT_SHORT_HASH") {
        use std::fmt::Write as _;
        let _ = write!(version, " ({})", hash);
    }

    version
});

#[derive(Debug, Parser)]
#[clap(version = &VERSION[..], about)]
struct Args {
    /// Verbosity level
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let pkg_name = env!("TRACING_FMT");
    let filter = match args.verbose {
        #[cfg(debug_assertions)]
        0 | 1 | 2 => format!("{}=debug", pkg_name),

        #[cfg(not(debug_assertions))]
        0 => format!("{}=info", pkg_name),
        #[cfg(not(debug_assertions))]
        1 | 2 => format!("{}=debug", pkg_name),

        3 => format!("{}=trace", pkg_name),
        _ => "trace".into(),
    };

    let filter = EnvFilter::new(filter);
    let fmt = fmt::layer().with_target(args.verbose >= 2);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
