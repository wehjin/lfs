use clap::{Parser, Subcommand};

use lots::LotsArgs;

use crate::cli::market::MarketArgs;
pub mod cap;
pub mod lots;
pub mod market;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Market(MarketArgs),
    Lots(LotsArgs),
    Cap,
}

pub fn run(cli: &Cli) -> anyhow::Result<()> {
    match &cli.command {
        Command::Market(args) => market::run(args),
        Command::Lots(args) => lots::run(args),
        Command::Cap => cap::print_cap(),
    }
}
