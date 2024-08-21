use crate::core::{AssetFilter, HostFilter};
use crate::data::{read_stash, write_stash, Lot};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct LotsArgs {
	#[clap(short, long, help = "Filter by asset")]
	pub asset: Option<String>,
	#[clap(short = 'c', long, help = "Filter by host")]
	pub host: Option<String>,
	#[clap(subcommand)]
	pub command: Option<LotCommand>,
}

#[derive(Debug, Subcommand)]
pub enum LotCommand {
	Add(AddLotArgs),
	Remove(RemoveLotArgs),
}

#[derive(Debug, Args)]
pub struct AddLotArgs {
	symbol: String,
	size: f64,
	cost: f64,
	host: String,
}

#[derive(Debug, Args)]
pub struct RemoveLotArgs {
	#[clap(short, long, help = "Number to remove", default_value_t = 1)]
	count: usize,
}

pub fn run(args: &LotsArgs) -> anyhow::Result<()> {
	let asset_filter = AssetFilter::new(&args.asset);
	let host_filter = HostFilter::new(&args.host);
	if let Some(command) = &args.command {
		match command {
			LotCommand::Add(args) => add_lots(args),
			LotCommand::Remove(args) => remove_lots(asset_filter, host_filter, args.count),
		}
	} else {
		view_lots(asset_filter, host_filter)
	}
}

fn view_lots(asset_filter: AssetFilter, host_filter: HostFilter) -> anyhow::Result<()> {
	let stash = read_stash()?;
	let mut lots = stash.to_lots(&asset_filter, &host_filter);
	lots.sort_by(|&(_, a), &(_, b)| a.asset.cmp(&b.asset));
	for (id, lot) in lots {
		print_lot(id, lot);
	}
	Ok(())
}

fn print_lot(id: u64, lot: &Lot) {
	println!("{}: {}", id, serde_json::to_string(lot).unwrap());
}

fn remove_lots(asset_filter: AssetFilter, host_filter: HostFilter, count: usize) -> anyhow::Result<()> {
	let mut stash = read_stash()?;
	let mut lots = stash.to_lots(&asset_filter, &host_filter).into_iter().map(|(id, _lot)| id).collect::<Vec<_>>();
	lots.truncate(count);
	for id in &lots {
		if let Some(lot) = stash.remove_lot(*id) {
			print_lot(*id, &lot)
		}
	}
	write_stash(&stash)?;
	println!("{} removed", lots.len());
	Ok(())
}
fn add_lots(args: &AddLotArgs) -> anyhow::Result<()> {
	let mut stash = read_stash()?;
	stash.add_lot(args.symbol.parse()?, args.size, args.cost, args.host.parse()?);
	write_stash(&stash)?;
	println!("1 added");
	Ok(())
}
