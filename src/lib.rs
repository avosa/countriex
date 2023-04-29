pub mod data;
pub mod models;

use serde_yaml;
use data::DATA_YAML;
use models::{Countries, Country};

pub fn all() -> Vec<Country> {
    let countries: Countries = serde_yaml::from_str(DATA_YAML).expect("Failed to parse YAML data");
    countries.values().cloned().collect()
}

pub fn get_by(key: &str, value: &str) -> Option<Country> {
    let countries: Countries = serde_yaml::from_str(DATA_YAML).expect("Failed to parse YAML data");
    countries.values().find(|country| matches(country, key, value)).cloned()
}

pub fn filter(key: &str, value: &str) -> Vec<Country> {
    let countries: Countries = serde_yaml::from_str(DATA_YAML).expect("Failed to parse YAML data");
    countries.values().filter(|country| matches(country, key, value)).cloned().collect()
}

fn matches(country: &Country, key: &str, value: &str) -> bool {
    match key {
        "alpha2" => country.alpha2 == value,
        "alpha3" => country.alpha3 == value,
        "continent" => country.continent == value,
        "currency_code" => country.currency_code.as_ref().map_or(false, |s| s == value),
        "name" => country.name == value,
        "region" => country.region.as_ref().map_or(false, |s| s == value),
        "eu_member" => country.eu_member.map_or(false, |b| b.to_string() == value),
        _ => false,
    }
}
