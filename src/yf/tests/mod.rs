use crate::yf;
use crate::yf::PATTERN_20241117;
use anyhow::Error;
use easy_scraper::Pattern;

const SUMMARY_HTML: &str = include_str!("sample_summary.html");

#[test]
pub fn print_json() -> anyhow::Result<()> {
    let prices = yf::price_list_from_summary(SUMMARY_HTML)?;
    let json = prices.to_json()?;
    println!("{}", json);
    Ok(())
}

#[test]
pub fn parse_20241117() -> anyhow::Result<()> {
    let html = include_str!("sample_20241117.html");
    let s = html.as_ref();
    let pattern = Pattern::new(PATTERN_20241117).map_err(|s| Error::msg(s))?;
    let matches = pattern.matches(s);
    assert_eq!(2, matches.len());
    Ok(())
}
