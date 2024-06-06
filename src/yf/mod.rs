use std::ops::Index;

use easy_scraper::Pattern;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

pub type Error = anyhow::Error;

pub fn price_list_from_summary(html: impl AsRef<str>) -> Result<PriceList, Error> {
	let s = html.as_ref();
	let pattern = Pattern::new(r#"
			<table><tbody><tr>
				<td><a title="{{asset}}">{{symbol}}</a></td>
				<td><fin-streamer value="{{share_price}}"></fin-streamer></td>
				<td></td>
				<td></td>
				<td>{{share_price_currency}}</td>
				<td><fin-streamer value="{{unix_epoch}}"></fin-streamer></td>
			</tr></tbody></table>
		"#).map_err(|s| Error::msg(s))?;
	let matches = pattern.matches(s);
	let mut prices = Vec::new();
	for map in matches {
		let price = MarketPrice {
			symbol: map["symbol"].to_string(),
			asset: map["asset"].to_string(),
			price: map["share_price"].parse()?,
			currency: map["share_price_currency"].to_string(),
			epoch: map["unix_epoch"].parse()?,
		};
		prices.push(price);
	}
	Ok(PriceList(prices))
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceList(Vec<MarketPrice>);

impl PriceList {
	pub fn to_json(&self, pretty: bool) -> Result<String, Error> {
		let json = match pretty {
			true => serde_json::to_string_pretty(self),
			false => serde_json::to_string(self),
		}?;
		Ok(json)
	}
}

impl Index<usize> for PriceList {
	type Output = MarketPrice;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPrice {
	pub symbol: String,
	pub asset: String,
	pub price: f64,
	pub currency: String,
	pub epoch: u64,
}
