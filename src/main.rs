use clap::{Args, Parser, Subcommand};

use crate::core::AssetSymbol;

pub mod yf;

pub mod core;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[clap(subcommand)]
	command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
	Market(MarketArgs),
}

#[derive(Debug, Args)]
struct MarketArgs {
	#[clap(value_delimiter = ',', required = true)]
	symbols: Vec<String>,
}

impl MarketArgs {
	fn parse_symbols(&self) -> anyhow::Result<Vec<AssetSymbol>> {
		let mut symbols = Vec::new();
		for symbol in &self.symbols {
			let symbol = symbol.parse::<AssetSymbol>()?;
			symbols.push(symbol)
		}
		Ok(symbols)
	}
}

fn main() -> anyhow::Result<()> {
	let app = Cli::parse();
	match &app.command {
		Command::Market(args) => {
			let symbols = args.parse_symbols()?;
			let printout = yf::fetch_prices(&symbols)?.to_json()?;
			println!("{}", printout);
		}
	}
	Ok(())
}
