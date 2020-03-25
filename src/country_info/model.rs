use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CountryInfo {
    pub country: String,
    pub cases: i32,
    pub today_cases: i32,
    pub deaths: i32,
    pub today_deaths: i32,
    pub recovered: i32,
    pub critical: i32,
    pub cases_per_one_million: i32,
}
