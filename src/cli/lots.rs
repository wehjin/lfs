use crate::cli::cap::{format_f64, RoundStyle, SeparationStyle};
use crate::core::{AssetFilter, HostFilter};
use crate::data::{read_stash, write_stash};
use clap::{Args, Subcommand};
use std::str::FromStr;

#[derive(Debug, Args)]
pub struct LotsArgs {
    #[clap(long, help = "Filter by asset")]
    pub asset: Option<String>,
    #[clap(long, help = "Filter by host")]
    pub host: Option<String>,
    #[clap(long, help = "Filter by size")]
    pub size: Option<PositiveShareCount>,
    #[clap(subcommand)]
    pub command: Option<LotCommand>,
}

#[derive(Debug, Subcommand)]
pub enum LotCommand {
    Add(AddLotArgs),
    Remove(RemoveLotArgs),
    Resize { count: PositiveShareCount },
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

#[derive(Debug, Copy, Clone)]
pub enum PositiveShareCount {
    Number(f64),
}

impl PositiveShareCount {
    pub fn matches_f64(&self, value: f64) -> bool {
        match self {
            PositiveShareCount::Number(f) => value == *f,
        }
    }
}

impl FromStr for PositiveShareCount {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f64>() {
            Ok(f) if f > 0.0 => Ok(PositiveShareCount::Number(f)),
            Ok(_) => Err("must be positive"),
            _ => Err("must be a number"),
        }
    }
}

pub fn run(args: &LotsArgs) -> anyhow::Result<()> {
    let asset_filter = AssetFilter::new(&args.asset);
    let host_filter = HostFilter::new(&args.host);
    let size_filter = args.size;
    if let Some(command) = &args.command {
        match command {
            LotCommand::Add(args) => add_lots(args),
            LotCommand::Remove(args) => {
                remove_lots(asset_filter, host_filter, size_filter, args.count)
            }
            LotCommand::Resize { count } => {
                match count {
                    PositiveShareCount::Number(new_size) => {
                        let mut stash = read_stash()?;
                        let lots = stash.to_lots(&asset_filter, &host_filter, &size_filter);
                        match lots.len() {
                            0 => {
                                println!("no matching lot");
                            }
                            1 => {
                                let &(id, _lot) = lots.first().expect("lot is present");
                                let mut removed = stash.remove_lot(id).expect("can remove lot");
                                removed.size = *new_size;
                                stash.insert_lot(id, removed);
                                write_stash(&stash)?;
                                println!("lot resized to {}", new_size);
                            }
                            _ => {
                                println!("too many lots: {}", lots.len());
                            }
                        }
                    }
                }
                Ok(())
            }
        }
    } else {
        view_lots(asset_filter, host_filter, size_filter)
    }
}

fn view_lots(
    asset_filter: AssetFilter,
    host_filter: HostFilter,
    size_filter: Option<PositiveShareCount>,
) -> anyhow::Result<()> {
    let stash = read_stash()?;
    let mut lots = stash.to_lots(&asset_filter, &host_filter, &size_filter);
    lots.sort_by(|&(_, a), &(_, b)| a.asset.cmp(&b.asset));

    print_divider();
    println!("| {:12} | {:12} | {:12} |", "shares", "asset", "custodian");
    print_divider();
    for (_id, lot) in lots {
        println!(
            "| {:12} | {:12} | {:12} |",
            format_f64(lot.size, RoundStyle::Floor, SeparationStyle::Char(',')),
            lot.asset.as_str(),
            lot.host.as_str(),
        );
    }
    print_divider();
    Ok(())
}

fn print_divider() {
    println!("+-{:-^12}-+-{:-^12}-+-{:-^12}-+", "", "", "");
}

fn remove_lots(
    asset_filter: AssetFilter,
    host_filter: HostFilter,
    size_filter: Option<PositiveShareCount>,
    count: usize,
) -> anyhow::Result<()> {
    let mut stash = read_stash()?;
    let ids = {
        let mut lots = stash.to_lots(&asset_filter, &host_filter, &size_filter);
        lots.truncate(count);
        lots.iter().map(|(id, _lot)| *id).collect::<Vec<u64>>()
    };

    let mut removed = 0usize;
    for id in ids {
        if let Some(lot) = stash.remove_lot(id) {
            let shares = format_f64(lot.size, RoundStyle::Floor, SeparationStyle::Char(','));
            println!(
                "| {:12} | {:8} | {:12} |",
                shares,
                lot.asset.as_str(),
                lot.host.as_str(),
            );
        }
        removed += 1;
    }
    write_stash(&stash)?;
    println!("{} removed", removed);
    Ok(())
}
fn add_lots(args: &AddLotArgs) -> anyhow::Result<()> {
    let mut stash = read_stash()?;
    stash.add_lot(
        args.symbol.parse()?,
        args.size,
        args.cost,
        args.host.parse()?,
    );
    write_stash(&stash)?;
    println!("1 added");
    Ok(())
}
