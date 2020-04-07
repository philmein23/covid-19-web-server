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
    pub cases_per_one_million: Option<i32>,
    pub deaths_per_one_million: Option<i32>,
    pub tests: Option<i32>,
    pub tests_per_one_million: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CountryInfo {
    pub _id: Option<i32>,
    pub iso2: Option<String>,
    pub iso3: Option<String>,
    #[serde(deserialize_with = "round_deserialize")]
    pub lat: f32,
    #[serde(deserialize_with = "round_deserialize")]
    pub long: f32,
    pub flag: Option<String>
}

struct F32Visitor;

impl<'de> Visitor<'de> for F32Visitor {
    type Value = f32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "needs to be a f32 type")
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where E: Error {
        println!("VISITOR: {}", v);
        Ok(v.round())
    }
}

fn round_deserialize<'de, D>(d: D) -> Result<f32, D::Error> where D: Deserializer<'de> {
    d.deserialize_f32(F32Visitor)
}
