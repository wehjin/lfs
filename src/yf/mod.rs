use std::collections::HashMap;
use std::ops::Index;
use std::str::FromStr;

use easy_scraper::Pattern;
use reqwest::header::ACCEPT;
use serde::{Deserialize, Serialize};

use crate::core::AssetSymbol;

#[cfg(test)]
mod tests;

pub type Error = anyhow::Error;

const EARLY_PATTERN: &'static str = r#"
			<table><tbody><tr>
				<td><a title="{{asset}}">{{symbol}}</a></td>
				<td><fin-streamer value="{{share_price}}"></fin-streamer></td>
				<td></td>
				<td></td>
				<td>{{share_price_currency}}</td>
				<td><fin-streamer value="{{unix_epoch}}"></fin-streamer></td>
			</tr></tbody></table>
		"#;

const PATTERN_20241117: &'static str = r#"
			<table><tbody><tr>
				<td><a title="{{asset}}">{{symbol}}</a></td>
				<td><fin-streamer data-field="regularMarketPrice" data-value="{{share_price}}"></fin-streamer></td>
				<td></td>
				<td></td>
				<td></td>
				<td>{{share_price_currency}}</td>
			</tr></tbody></table>
		"#;

pub fn price_list_from_summary(html: impl AsRef<str>) -> Result<PriceList, Error> {
    let s = html.as_ref();
    let early_matches = Pattern::new(EARLY_PATTERN)
        .map_err(|s| Error::msg(s))?
        .matches(s);
    let matches = if early_matches.is_empty() {
        Pattern::new(PATTERN_20241117)
            .map_err(|s| Error::msg(s))?
            .matches(s)
    } else {
        early_matches
    };
    let mut prices = Vec::new();
    for map in matches {
        let price = MarketPrice {
            symbol: map["symbol"].to_string(),
            asset: map["asset"].to_string(),
            level: map["share_price"].parse()?,
            currency: map["share_price_currency"].to_string(),
            epoch: match map.get("unix_epoch") {
                None => chrono::Utc::now().timestamp() as u64,
                Some(s) => s.parse::<u64>()?,
            },
        };
        prices.push(price);
    }
    Ok(PriceList(prices))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceList(Vec<MarketPrice>);

impl PriceList {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(json)
    }

    pub fn iter(&self) -> impl Iterator<Item = &MarketPrice> {
        self.0.iter()
    }
    pub fn to_map(&self) -> HashMap<AssetSymbol, MarketPrice> {
        let mut map = HashMap::new();
        for price in &self.0 {
            let symbol = AssetSymbol::from_str(price.symbol.as_str()).expect("valid symbol");
            map.insert(symbol, price.clone());
        }
        map
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
    let mut symbols = symbols.to_vec();
    symbols.sort();
    symbols.dedup();
    let symbol_list = symbols
        .iter()
        .map(AssetSymbol::to_string)
        .collect::<Vec<String>>()
        .join(",");
    let url = format!(
        "https://finance.yahoo.com/quotes/{}/view/v1?ncid=yahooproperties_portfolios_pbody",
        symbol_list
    );
    let client = reqwest::blocking::Client::builder()
		.user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
		.build()?;
    let text = client.get(&url)
		.header(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
		.send()?
		.text()?;
    let price_list = price_list_from_summary(&text)?;
    if price_list.len() != symbols.len() {
        let mut price_list_symbols = price_list.to_map().keys().cloned().collect::<Vec<_>>();
        price_list_symbols.sort();
        return Err(anyhow::anyhow!(
            "length of price list does not match\nGot:\n{} {:?}\nExpected:\n{} {:?}\nUrl: {}\nText: {}",
            price_list.len(),
            price_list_symbols,
            symbols.len(),
            symbols,
            url,
            text
        ));
    }
    Ok(price_list)
}
