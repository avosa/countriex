mod data;
mod models;

use serde_json;
// use std::collections::HashMap;
use data::DATA_JSON;
use models::{Countries, Country};

pub fn all() -> usize {
    let countries: Countries = serde_json::from_str(DATA_JSON).expect("Failed to parse JSON data");
    countries.len()
}

pub fn get_by(key: &str, value: &str) -> Option<Country> {
    let countries: Countries = serde_json::from_str(DATA_JSON).expect("Failed to parse JSON data");
    countries.values().find(|country| matches(country, key, value)).cloned()
}

pub fn filter(key: &str, value: &str) -> Vec<Country> {
    let countries: Countries = serde_json::from_str(DATA_JSON).expect("Failed to parse JSON data");
    countries.values().filter(|country| matches(country, key, value)).cloned().collect()
}

fn matches(country: &Country, key: &str, value: &str) -> bool {
    match key {
        "alpha2" => country.alpha2 == value,
        "alpha3" => country.alpha3 == value,
        "continent" => country.continent == value,
        // "country_code" => country.country_code == value,
        "currency_code" => country.currency_code.as_ref().map_or(false, |s| s == value),
        "name" => country.name == value,
        "region" => country.region.as_ref().map_or(false, |s| s == value),
        "eu_member" => country.eu_member.map_or(false, |b| b.to_string() == value),
        _ => false,
    }
}
