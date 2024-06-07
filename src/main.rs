use clap::Parser;

use cli::Cli;

fn main() -> anyhow::Result<()> {
	let cli = Cli::parse();
	cli::run(&cli)
}

mod cli;

pub mod core;
pub mod data;
pub mod yf;
