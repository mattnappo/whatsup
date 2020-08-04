use whatsup::header::display_header;
use whatsup::prices::display_prices;
use whatsup::weather::display_weather;

fn main() {
    display_header();
    display_weather();
    display_prices();
}
