use crate::core::AssetFilter;
use crate::data::{read_stash, write_stash};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct LotsArgs {
	#[clap(short, long, help = "Filter by asset")]
	pub asset: Option<String>,
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

pub fn run(args: &LotsArgs) -> anyhow::Result<()> {
	let asset_filter = AssetFilter::new(&args.asset);
	if let Some(command) = &args.command {
		match command {
			LotCommand::Add(args) => add_lots(args),
		}
	} else {
		view_lots(asset_filter)
	}
}

fn view_lots(asset_filter: AssetFilter) -> anyhow::Result<()> {
	let stash = read_stash()?;
	for (id, lot) in stash.to_lots(&asset_filter) {
		println!("{}: {}", id, serde_json::to_string(lot).unwrap());
	}
	Ok(())
}


fn add_lots(args: &AddLotArgs) -> anyhow::Result<()> {
	let mut stash = read_stash()?;
	stash.add_lot(args.symbol.parse()?, args.size, args.cost, args.host.parse()?);
	write_stash(&stash)?;
	println!("{} lots", stash.lots.len());
	Ok(())
}
