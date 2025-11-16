use crate::command::Commands;

#[derive(clap::Parser)]
#[command(author, version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
