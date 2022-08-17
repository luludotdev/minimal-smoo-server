use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    disable_help_flag = true,
    disable_version_flag = true,
    no_binary_name = true
)]
pub enum Command {
    #[clap(subcommand)]
    Moon(MoonCommand),

    #[clap(alias = "quit", alias = "stop")]
    Exit,
}

#[derive(Debug, Parser)]
pub enum MoonCommand {
    List,
    Sync,
    Send { id: i32, player: String },
}
