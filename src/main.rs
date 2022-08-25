use reqwest::header::{HeaderName, HeaderMap, HeaderValue};
struct Country {
    official_name: String,
    shorthand_name: String,
    current_leader: String,
    type_of_govern: String,
    last_three_leaders: String,
    area: u64,
    population: u64,
    gdp: u64,
    debt: i64,
}

fn main() {
    let header_map: HeaderMap = HeaderMap::new();
    header_map.insert(ACCEPT, "text/html")
    let response1= reqwest::blocking::get("https://en.wikipedia.org/wiki/List_of_countries_and_dependencies_by_population");
}
