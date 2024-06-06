use std::ops::Index;

use easy_scraper::Pattern;
use serde::{Deserialize, Serialize};

use crate::core::AssetSymbol;

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
			level: map["share_price"].parse()?,
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
	pub fn to_json(&self) -> Result<String, Error> {
		let json = serde_json::to_string_pretty(self)?;
		Ok(json)
	}

	pub fn iter(&self) -> impl Iterator<Item=&MarketPrice> {
		self.0.iter()
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
	pub level: f64,
	pub currency: String,
	pub epoch: u64,
}

pub type FetchPricesError = anyhow::Error;

pub fn fetch_prices(symbols: &[AssetSymbol]) -> Result<PriceList, FetchPricesError> {
	let symbol_list = symbols.iter().map(AssetSymbol::to_string).collect::<Vec<String>>().join(",");
	let url = format!("https://finance.yahoo.com/quotes/{}/view/v1?ncid=yahooproperties_portfolios_pbody", symbol_list);
	let client = reqwest::blocking::Client::builder()
		.user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36")
		.build()?;
	let text = client.get(url).send()?.text()?;
	let price_list = price_list_from_summary(text)?;
	Ok(price_list)
}
