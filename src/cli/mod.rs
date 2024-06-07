use clap::{Args, Parser, Subcommand};

use crate::core::AssetSymbol;
use crate::yf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	#[clap(subcommand)]
	pub command: Command,
}

impl Cli {
	pub fn run(&mut self) -> anyhow::Result<()> {
		match &self.command {
			Command::Market(args) => {
				let symbols = args.parse_symbols()?;
				let printout = yf::fetch_prices(&symbols)?.to_json()?;
				println!("{}", printout);
			}
		}
		Ok(())
	}
}

#[derive(Debug, Subcommand)]
pub enum Command {
	Market(MarketArgs),
}

#[derive(Debug, Args)]
pub struct MarketArgs {
	#[clap(value_delimiter = ',', required = true)]
	symbols: Vec<String>,
}

impl MarketArgs {
	pub fn parse_symbols(&self) -> anyhow::Result<Vec<AssetSymbol>> {
		let mut symbols = Vec::new();
		for symbol in &self.symbols {
			let symbol = symbol.parse::<AssetSymbol>()?;
			symbols.push(symbol)
		}
		Ok(symbols)
	}
}

