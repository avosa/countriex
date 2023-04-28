// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Country {
//     pub address_format: Option<String>,
//     pub alpha2: String,
//     pub alpha3: String,
//     pub continent: String,
//     pub country_code: String,
//     pub currency_code: String,
//     pub name: String,
//     pub world_region: Option<String>,
// }


// pub type Countries = HashMap<String, Country>;


use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Country {
    pub address_format: Option<String>,
    pub alpha2: String,
    pub alpha3: String,
    pub continent: String,
    pub country_code: Option<String>,
    pub currency_code: Option<String>,
    pub gec: Option<String>,
    pub geo: Option<String>,
    pub international_prefix: Option<String>,
    pub ioc: Option<String>,
    pub name: String,
    pub national_destination_code_lengths: Option<Vec<u32>>,
    pub national_number_lengths: Option<Vec<u32>>,
    pub national_prefix: Option<String>,
    pub nanp_prefix: Option<String>,
    pub nationality: Option<String>,
    pub number: Option<String>,
    pub languages_official: Option<HashMap<String, String>>,
    pub languages_spoken: Option<HashMap<String, String>>,
    pub postal_code: Option<String>,
    pub region: Option<String>,
    pub unofficial_names: Option<Vec<String>>,
    pub subregion: Option<String>,
    pub un_locode: Option<String>,
    pub vat_rates: Option<String>,
    pub alt_currency: Option<String>,
    pub eea_member: Option<bool>,
    pub eu_member: Option<bool>,
    pub world_region: Option<String>,
}

pub type Countries = HashMap<String, Country>;
