use clap::{Parser, Subcommand};

use lots::LotsArgs;

use crate::cli::market::MarketArgs;
use crate::data::read_stash;
use crate::yf::fetch_prices;

pub mod market;
pub mod lots;

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
		Command::Cap => print_cap(),
	}
}

fn print_cap() -> anyhow::Result<()> {
	let stash = read_stash()?;
	let assets = stash.assets();
	let market_prices = fetch_prices(assets.as_slice())?.to_map();
	let cap = stash.value(&market_prices);
	println!("{} USD", cap);
	Ok(())
}