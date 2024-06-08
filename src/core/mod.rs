use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::string::ParseError;

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

