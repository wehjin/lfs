use clap::Parser;

use cli::Cli;

pub mod yf;

pub mod core;
mod cli;

fn main() -> anyhow::Result<()> {
	let mut app = Cli::parse();
	app.run()
}
