use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::string::ParseError;

use crate::data::AssetHost;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct AssetSymbol(String);

impl AssetSymbol {
	pub fn as_str(&self) -> &str { &self.0 }
}

impl Display for AssetSymbol {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0)
	}
}

impl FromStr for AssetSymbol {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let symbol = Self(s.to_uppercase());
		Ok(symbol)
	}
}

pub enum HostFilter {
	None,
	One(AssetHost),
}

impl HostFilter {
	pub fn new(value: &Option<String>) -> Self {
		match value {
			Some(s) => {
				let asset_host = AssetHost::from_str(s).unwrap();
				Self::One(asset_host)
			}
			None => Self::None,
		}
	}
	pub fn pass(&self, value: &AssetHost) -> bool {
		match self {
			HostFilter::None => true,
			HostFilter::One(s) => s == value,
		}
	}
}

pub enum AssetFilter {
	None,
	One(AssetSymbol),
}

impl AssetFilter {
	pub fn new(value: &Option<String>) -> Self {
		match value {
			Some(s) if !s.is_empty() => {
				let asset_symbol = AssetSymbol::from_str(s).unwrap();
				Self::One(asset_symbol)
			}
			_ => Self::None
		}
	}
	pub fn pass(&self, value: &AssetSymbol) -> bool {
		match self {
			AssetFilter::None => true,
			AssetFilter::One(filter) => filter == value,
		}
	}
}