use serde::{Deserialize, Serialize, Deserializer};
use serde::de::{Visitor, Error};
use std::fmt;
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub country: String,
    pub country_info: CountryInfo,
    pub updated: i64,
    pub cases: i32,
    pub today_cases: i32,
    pub deaths: i32,
    pub today_deaths: i32,
    pub recovered: i32,
    pub active: i32,
    pub critical: i32,
    pub cases_per_one_million: Option<f32>,
    pub deaths_per_one_million: Option<f32>,
    pub tests: Option<i32>,
    pub tests_per_one_million: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CountryInfo {
    pub _id: Option<i32>,
    pub iso2: Option<String>,
    pub iso3: Option<String>,
    pub lat: f32,
    pub long: f32,
    pub flag: Option<String>
}


