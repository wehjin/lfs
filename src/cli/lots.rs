use crate::cli::{AddLotArgs, LotCommand, LotsArgs};
use crate::data::{read_stash, write_stash};

pub fn run(args: &LotsArgs) -> anyhow::Result<()> {
	if let Some(command) = &args.command {
		match command {
			LotCommand::Add(args) => run_add_lots(args),
		}
	} else {
		run_list_lots()
	}
}

fn run_list_lots() -> anyhow::Result<()> {
	let stash = read_stash()?;
	for (id, lot) in &stash.lots {
		println!("{}: {}", id, serde_json::to_string(lot)?);
	}
	Ok(())
}

fn run_add_lots(args: &AddLotArgs) -> anyhow::Result<()> {
	let mut stash = read_stash()?;
	stash.add_lot(args.symbol.parse()?, args.size, args.cost, args.host.parse()?);
	write_stash(&stash)?;
	println!("{} lots", stash.lots.len());
	Ok(())
}
