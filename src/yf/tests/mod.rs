use crate::yf;

const SUMMARY_HTML: &str = include_str!("sample_summary.html");

#[test]
pub fn print_json() -> anyhow::Result<()> {
	let prices = yf::price_list_from_summary(SUMMARY_HTML)?;
	let json = prices.to_json(true)?;
	println!("{}", json);
	Ok(())
}
