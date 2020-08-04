use colored::*;
use yahoo_finance::{history, Interval};

struct Price {
    pub name: String,
    pub today: f64,
    pub percent_change: f64,
}

// TODO: Make these return a &str somehow (&'static or &'a str)
fn dollars(d: f64) -> String {
    format!("${:.2}", d)
}

fn percent(p: f64) -> String {
    format!("{:.2}%", p * 100.0)
}

impl Price {
    fn fmt(&self) -> String {
        if self.percent_change > 0.0 {
            return format!(
                "{}: {} ({})",
                self.name.bold().white(),
                dollars(self.today).green(),
                percent(self.percent_change).green(),
            );
        } else if self.percent_change < 0.0 {
            return format!(
                "{}: {} ({})",
                self.name.bold().white(),
                dollars(self.today).red(),
                percent(self.percent_change).red(),
            );
        } else {
            return format!(
                "{}: {} ({})",
                self.name.bold().white(),
                dollars(self.today).green(),
                percent(self.percent_change).green(),
            );
        }
    }
}

fn get_prices() -> Result<Vec<Price>, yahoo_finance::Error> {
    let symbols = vec!["^GSPC", "BTC-USD"];
    let names = vec!["SP500", "  BTC"];

    let mut data: Vec<Price> = vec![];
    for i in 0..symbols.len() {
        let sec_data = history::retrieve_interval(symbols[i], Interval::_5d)?;

        let yesterday = sec_data[sec_data.len() - 2].close;
        let today = sec_data[sec_data.len() - 1].close;
        let percent_change = (yesterday - today) / today;

        data.push(Price {
            name: names[i].to_string(),
            today,
            percent_change,
        });
    }

    Ok(data)
}

pub fn display_prices() {
    for price in get_prices().unwrap() {
        println!("{}", price.fmt());
    }
}
