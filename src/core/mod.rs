use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct AssetSymbol(String);

pub type SymbolParseError = anyhow::Error;

impl Display for AssetSymbol {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0)
	}
}

impl FromStr for AssetSymbol {
	type Err = SymbolParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let upper = s.to_uppercase();
		Ok(AssetSymbol(upper))
	}
}

