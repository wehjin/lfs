use std::{fs, io};
use std::collections::BTreeMap;
use std::io::ErrorKind;
use std::str::FromStr;
use std::string::ParseError;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub fn write_stash(stash: &Stash) -> io::Result<()> {
	let json = serde_json::to_string_pretty(stash)?;
	fs::write(paths::main_stash_json()?, json)
}

pub fn read_stash() -> io::Result<Stash> {
	match fs::read(paths::main_stash_json()?) {
		Ok(bytes) => Ok(serde_json::from_slice::<Stash>(bytes.as_slice())?),
		Err(e) => if ErrorKind::NotFound == e.kind() {
			Ok(Stash::default())
		} else {
			Err(e)
		},
	}
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stash {
	pub max_lot_id: u64,
	pub lots: BTreeMap<u64, Lot>,
}

impl Stash {
	pub fn add_lot(&mut self, asset: AssetSymbol, size: f64, cost: f64, host: AssetHost) {
		let basis = Basis {
			cost,
			count: size,
			time: Utc::now(),
			host: host.clone(),
		};
		let lot = Lot { asset, size, basis, host };
		let next_id = self.max_lot_id + 1;
		self.lots.insert(next_id, lot);
		self.max_lot_id = next_id;
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lot {
	pub asset: AssetSymbol,
	pub size: f64,
	pub basis: Basis,
	pub host: AssetHost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Basis {
	pub cost: f64,
	pub count: f64,
	pub time: DateTime<Utc>,
	pub host: AssetHost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetSymbol(String);

impl FromStr for AssetSymbol {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let symbol = Self(s.to_uppercase());
		Ok(symbol)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetHost(String);

impl FromStr for AssetHost {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let host = Self(s.trim().to_string());
		Ok(host)
	}
}

impl Default for AssetHost {
	fn default() -> Self {
		Self("Left cheek".to_string())
	}
}

mod paths {
	use std::{fs, io};
	use std::io::ErrorKind;
	use std::path::PathBuf;

	pub fn data_dir() -> io::Result<PathBuf> {
		let path = base_dir();
		if !path.is_dir() {
			if path.exists() {
				return Err(ErrorKind::AlreadyExists.into());
			}
			fs::create_dir_all(&path)?;
		}
		Ok(path)
	}

	fn base_dir() -> PathBuf {
		#[cfg(debug_assertions)]
		const EXTENSION: &str = "debug";
		#[cfg(not(debug_assertions))]
		const EXTENSION: &str = "release";
		dirs::data_dir().expect("data_dir").join(format!("{}_data", PKG_NAME)).join(EXTENSION)
	}

	const PKG_NAME: &str = env!("CARGO_PKG_NAME");

	pub fn main_stash_json() -> io::Result<PathBuf> {
		Ok(data_dir()?.join("main_stash.json"))
	}
}