mod cli;
mod command;
mod error;

use clap::Parser;
pub use cli::*;
pub use error::*;

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.command.run()
}
