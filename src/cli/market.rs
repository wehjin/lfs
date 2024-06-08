use clap::Args;

use crate::core::AssetSymbol;
use crate::yf;

#[derive(Debug, Args)]
pub struct MarketArgs {
	#[clap(value_delimiter = ',', required = true)]
	symbols: Vec<String>,
}

pub fn run(args: &MarketArgs) -> anyhow::Result<()> {
	let symbols = parse_symbols(args)?;
	let printout = yf::fetch_prices(&symbols)?.to_json()?;
	println!("{}", printout);
	Ok(())
}

fn parse_symbols(args: &MarketArgs) -> anyhow::Result<Vec<AssetSymbol>> {
	let mut symbols = Vec::new();
	for symbol in &args.symbols {
		let symbol = symbol.parse::<AssetSymbol>()?;
		symbols.push(symbol)
	}
	Ok(symbols)
}
