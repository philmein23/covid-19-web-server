use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::env;
use warp::{reject, reply, Filter, Rejection, Reply};

mod country_info;
use country_info::Country;

#[derive(Debug, Deserialize, Serialize)]
pub struct ListOptions {
    pub recovered: Option<i32>,
    pub deaths: Option<Threshold>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Threshold {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "countries=info")
    }

    // Read and parse the URL as early as possible; we don't even start
    // the server if this isn't provided
    let upstream_url = env::var("URL")
        .ok()
        .and_then(|url| url.parse::<url::Url>().ok())
        .expect("URL has to be provided and has to be valid");

    let filtered_countries = warp::path!("countries" / u32 / String)
        .map(|id, name| format!("Param name: {} {}", id, name));

    let routes = warp::get().and(filtered_countries);

    // And we now inject/pass data to the route handlers (in this case the remote url)
    warp::serve(countries(upstream_url))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

pub fn countries(
    upstream_url: url::Url,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    all_countries(upstream_url.clone())
        .or(country_deaths(upstream_url.clone()))
        .or(country_recovered(upstream_url.clone()))
}

pub fn all_countries(
    upstream_url: url::Url,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("countries")
        .and(warp::get())
        // This is how to make something available as a parameter
        // from this point onwards in the filter chain
        .and(warp::any().map(move || upstream_url.clone()))
        .and_then(get_all_countries)
}

pub fn country_deaths(
    upstream_url: url::Url,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let upstream_url = warp::any().map(move || upstream_url.clone());
    warp::path!("countries" / "deaths")
        .and(warp::get())
        .and(upstream_url)
        .and(warp::query::<Threshold>())
        .and_then(get_country_deaths)
}

pub fn country_recovered(
    upstream_url: url::Url,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let upstream_url = warp::any().map(move || upstream_url.clone());

    warp::path!("countries" / "recovered")
        .and(warp::get())
        .and(upstream_url)
        .and(warp::query::<Threshold>())
        .and_then(get_country_recovered)
}

// A rejection is a concrete type from warp, but it does not provide automatic
// conversion from other types of errors. To "reject" requests, we need to
// define custom rejection types (by implementing the trait `warp::reject::Reject`
// for those types, with an empty impl block), and then wrap instances of those
// types using `warp::reject::custom`. To customize a bit how those errors are
// converted into responses, then look at the "rejections" example in warp's repo.
#[derive(Debug)]
struct FetchError;
impl warp::reject::Reject for FetchError {}

pub async fn get_country_recovered(
    upstream_url: url::Url,
    opts: Threshold,
) -> Result<impl Reply, Rejection> {
    match fetch_url(upstream_url).await {
        Ok(countries) => match (opts.min, opts.max) {
            (Some(min), None) => {
                let filtered = countries
                    .into_iter()
                    .filter(|country| country.recovered > min)
                    .collect::<Vec<Country>>();
                Ok(reply::json(&filtered))
            }
            (None, Some(max)) => {
                let filtered = countries
                    .into_iter()
                    .filter(|country| country.recovered < max)
                    .collect::<Vec<Country>>();
                Ok(reply::json(&filtered))
            }
            (Some(min), Some(max)) => {
                let filtered = countries
                    .into_iter()
                    .filter(|country| country.recovered > min && country.recovered < max)
                    .collect::<Vec<Country>>();
                Ok(reply::json(&filtered))
            }
            (None, None) => Ok(reply::json(&countries)),
        },
        Err(_) => Err(reject::custom(FetchError)),
    }
}

pub async fn get_country_deaths(
    upstream_url: url::Url,
    opts: Threshold,
) -> Result<impl Reply, Rejection> {
    match fetch_url(upstream_url).await {
        Ok(countries) => match (opts.min, opts.max) {
            (Some(min), None) => {
                let filtered = countries
                    .into_iter()
                    .filter(|country| country.deaths > min)
                    .collect::<Vec<Country>>();
                Ok(reply::json(&filtered))
            }
            (None, Some(max)) => {
                let filtered = countries
                    .into_iter()
                    .filter(|country| country.deaths < max)
                    .collect::<Vec<Country>>();
                Ok(reply::json(&filtered))
            }
            (Some(min), Some(max)) => {
                let filtered = countries
                    .into_iter()
                    .filter(|country| country.deaths > min && country.deaths < max)
                    .collect::<Vec<Country>>();
                Ok(reply::json(&filtered))
            }
            (None, None) => Ok(reply::json(&countries)),
        },
        Err(_) => Err(reject::custom(FetchError)),
    }
}

// The upstream_url here was given by the filter chain set up in `countries`
pub async fn get_all_countries(
    upstream_url: url::Url,
) -> Result<impl warp::Reply, warp::Rejection> {
    match fetch_url(upstream_url).await {
        Ok(countries) => Ok(reply::json(&countries)),
        Err(err) => {
            println!("Err: {:?}", err);
            Err(warp::reject::custom(FetchError))
        }
    }

    // Another way to do this..
    // fetch_url(upstream_url)
    //     .await
    //     .map(|countries| Ok(warp::reply::json(&countries)))
    //     .map_err(|_| warp::reject::custom(FetchError))
}

async fn fetch_url(url: reqwest::Url) -> Result<Vec<Country>, reqwest::Error> {
    let data = reqwest::get(url).await?.json().await?;
    Ok(data)
}
