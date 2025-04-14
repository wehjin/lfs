use crate::data::read_stash;
use crate::yf::fetch_prices;

pub fn print_cap() -> anyhow::Result<()> {
    let stash = read_stash()?;
    let assets = stash.assets();
    let market_prices = fetch_prices(assets.as_slice())?.to_map();
    let cap = stash.value(&market_prices);
    let formatted_cap = format_f64(cap, RoundStyle::Floor, SeparationStyle::Char(','));
    println!("{} USD", formatted_cap);
    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RoundStyle {
    Floor,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SeparationStyle {
    Char(char),
}

fn format_f64(f: f64, round_style: RoundStyle, separation_style: SeparationStyle) -> String {
    let rounded: String;
    match round_style {
        RoundStyle::Floor => {
            rounded = f.floor().to_string();
        }
    }
    let separated: String;
    match separation_style {
        SeparationStyle::Char(separator) => {
            let mut segments = vec![];
            let mut len = rounded.chars().count();
            while len > 0 {
                let segment_len = 3.min(len);
                let start = len - segment_len;
                let segment = rounded
                    .chars()
                    .skip(start)
                    .take(segment_len)
                    .collect::<String>();
                segments.push(segment);
                len -= segment_len;
            }
            segments.reverse();
            separated = segments.join(separator.to_string().as_str());
        }
    }
    separated
}

#[cfg(test)]
mod test_format_f64 {
    use crate::cli::cap::{format_f64, RoundStyle, SeparationStyle};

    #[test]
    fn floor_with_char() {
        let tests = [
            (0000789.3199999999997, &"789"),
            (0006789.3199999999997, &"6,789"),
            (0056789.3199999999997, &"56,789"),
            (0456789.3199999999997, &"456,789"),
            (3456789.3199999999997, &"3,456,789"),
        ];
        for (f, &expected) in tests {
            let actual = format_f64(f, RoundStyle::Floor, SeparationStyle::Char(','));
            assert_eq!(expected, actual.as_str());
        }
    }
}
