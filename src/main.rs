#![forbid(unsafe_code)]
#![deny(private_in_public)]
#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::unused_self,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::fn_params_excessive_bools,
    clippy::inefficient_to_string,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    clippy::cast_lossless,
    clippy::implicit_clone,
    clippy::unused_async,
    clippy::redundant_closure_for_method_calls,
    clippy::default_trait_access,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations
)]

use std::net::IpAddr;

use clap::{ArgAction, Parser};
use color_eyre::Result;
use console::reader;
use console::writer::{self, ThreadWriter};
use once_cell::sync::Lazy;
use rustyline::Editor;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use crate::config::Config;
use crate::server::Server;

mod config;
mod console;
mod moons;
mod packet;
mod peer;
mod peers;
mod player;
mod players;
mod server;

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
pub struct Args {
    /// Verbosity level
    #[arg(short, long)]
    #[clap(action = ArgAction::Count)]
    verbose: u8,

    /// Server bind host [default: 0.0.0.0]
    #[clap(short, long)]
    host: Option<IpAddr>,

    /// Server bind host [default: 1027]
    #[clap(short, long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let mut rl = Editor::<()>::new()?;
    let printer = rl.create_external_printer()?;
    let (writer, rx) = ThreadWriter::new();

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
    let fmt = fmt::layer()
        .with_target(args.verbose >= 2)
        .with_writer(writer);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .with(ErrorLayer::default())
        .init();

    let config = Config::load().await?.shared();
    let server = Server::new(&args, config.clone()).await?;

    let listen_handle = tokio::spawn(server.clone().listen());
    let process_handle = tokio::spawn(server.clone().process_packets());
    let moon_sync_handle = tokio::spawn(server.clone().sync_moons_loop());
    let reader_handle = tokio::spawn(reader::read_loop(rl, server, config));
    let writer_handle = tokio::spawn(writer::write_loop(printer, rx));

    let _ = futures::join!(
        listen_handle,
        process_handle,
        moon_sync_handle,
        reader_handle,
        writer_handle
    );

    Ok(())
}
