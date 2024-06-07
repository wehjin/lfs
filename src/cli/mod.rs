use clap::{Args, Parser, Subcommand};

pub mod market;
pub mod lots;

pub fn run(cli: &Cli) -> anyhow::Result<()> {
	match &cli.command {
		Command::Market(args) => market::run(args),
		Command::Lots(args) => lots::run(args),
	}
}

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
}

#[derive(Debug, Args)]
pub struct MarketArgs {
	#[clap(value_delimiter = ',', required = true)]
	symbols: Vec<String>,
}

#[derive(Debug, Args)]
pub struct LotsArgs {
	#[clap(subcommand)]
	pub command: Option<LotCommand>,
}

#[derive(Debug, Subcommand)]
pub enum LotCommand {
	Add(AddLotArgs),
}

#[derive(Debug, Args)]
pub struct AddLotArgs {
	symbol: String,
	size: f64,
	cost: f64,
	host: String,
}