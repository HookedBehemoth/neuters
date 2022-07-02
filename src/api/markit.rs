#![allow(non_snake_case)]
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::{
    api::fetch::{fetch, fetch_multiple},
    de::unix::DateType,
};

use super::{common::Articles, error::ApiResult};

// They hardcodeded it, so it should work for a while.
const MODTOKEN_URL: &str = "https://apiservice.reuters.com/api/service/modtoken?method=get&format=json&apikey=72461C50B1CEAD3135BA6BDA53B203D3&deviceid=E7CDD293-9C3A-5AB9-9181-58E1B572DD44";

#[derive(Deserialize)]
struct ModTokenRaw {
    access_token: String,
    expires_in: String,
}

#[derive(Debug)]
pub struct ModToken {
    pub token: String,
    pub expires_in: u64,
    pub start: SystemTime,
}

pub fn fetch_market_token(client: &ureq::Agent) -> ApiResult<ModToken> {
    let token = client
        .get(MODTOKEN_URL)
        .call()?
        .into_json::<ModTokenRaw>()?;
    println!("{}, {}", token.access_token, token.expires_in);
    Ok(ModToken {
        token: token.access_token,
        expires_in: token.expires_in.parse().unwrap_or(14000),
        start: SystemTime::now(),
    })
}

const BASE_URL: &str = "https://content.markitcdn.com/api.markitondemand.com/apiman-gateway/MOD/";
const EXACT_PATH: &str = "chartworks-xref/1.0/xref/exact";
const STATES_PATH: &str = "chartworks-data/1.0/chartapi/series";

#[derive(Deserialize)]
struct ExchangeId {
    xid: u32,
}
#[derive(Deserialize)]
struct ExchangeIds {
    items: Vec<ExchangeId>,
}
#[derive(Deserialize)]
struct Exact {
    data: ExchangeIds,
}

pub fn fetch_ids(client: &ureq::Agent, token: &ModToken, names: &[&str]) -> ApiResult<Vec<u32>> {
    let url = format!("{}{}", BASE_URL, EXACT_PATH);
    #[derive(Serialize)]
    struct Element<'a> {
        symbol: &'a str,
    }
    let query: Vec<Element> = names.iter().map(|n| Element { symbol: n }).collect();
    let query = serde_json::to_string(&query)?;

    println!("query {}", query);

    let response: Exact = client
        .get(&url)
        .query("inputs", &query)
        .query("access_token", &token.token)
        .call()?
        .into_json()?;

    Ok(response.data.items.iter().map(|e| e.xid).collect())
}

#[derive(Serialize)]
struct GraphQueryElement<'a> {
    Label: &'a str,
    Type: &'a str,
    Symbol: String,
}

#[derive(Serialize)]
struct GraphQuery<'a> {
    days: u32,
    dataNormalized: bool,
    dataPeriod: &'a str,
    dataInterval: u32,
    realtime: bool,
    yFormat: &'a str,
    timeServiceFormat: &'a str,
    returnDateType: &'a str,
    rulerIntradayStart: u32,
    rulerIntradayStop: u32,
    rulerInterdayStart: u32,
    rulerInterdayStop: u32,
    elements: Vec<GraphQueryElement<'a>>,
}

#[derive(Deserialize)]
pub struct Graph {
    pub Dates: Vec<DateType>,
    pub NormalizeDate: DateType,
    pub Elements: Vec<GraphElement>,
}

#[derive(Deserialize)]
pub struct GraphElement {
    pub CompanyName: String,
    pub UtcOffsetMinutes: i32,
    pub ExchangeId: String,
    pub Currency: String,
    pub ComponentSeries: Vec<ComponentSeries>,
}

#[derive(Deserialize)]
pub struct ComponentSeries {
    pub Type: String,
    pub MaxValue: f64,
    pub MinValue: f64,
    pub MaxValueDate: DateType,
    pub MinValueDate: DateType,
    pub Values: Vec<f64>,
}

pub fn fetch_graph(client: &ureq::Agent, token: &ModToken, ids: &[u32]) -> ApiResult<Graph> {
    let url = format!("{BASE_URL}{STATES_PATH}");

    let elements = ids
        .iter()
        .map(|id| GraphQueryElement {
            Label: "53d43b4a",
            Type: "price",
            Symbol: id.to_string(),
        })
        .collect();

    let data = GraphQuery {
        days: 365,
        dataNormalized: false,
        dataPeriod: "Day",
        dataInterval: 1,
        realtime: false,
        yFormat: "0.###",
        timeServiceFormat: "shit",
        returnDateType: "Unix", // MSDate, ISO8601, RFC1123, or Unix
        rulerIntradayStart: 31,
        rulerIntradayStop: 3,
        rulerInterdayStart: 10957,
        rulerInterdayStop: 365,
        elements,
    };

    Ok(client
        .post(&url)
        .query("access_token", &token.token)
        .send_json(data)?
        .into_json()?)
}

pub fn related_articles(client: &ureq::Agent, symbol: &str) -> ApiResult<Articles> {
    const API_URL: &str =
        "https://www.reuters.com/pf/api/v3/content/fetch/articles-by-stock-symbol-v1";

    let query = format!(r#"{{"website":"reuters","symbol":"{symbol}","arc-site":"reuters"}}"#);

    fetch(client, API_URL, &query)
}

#[derive(Deserialize)]
pub struct Quote {
    pub r#type: Box<str>,
    pub name: Box<str>,
    pub currency: Box<str>,
    pub day_high: f32,
    pub day_low: f32,
    pub fiftytwo_wk_high: f32,
    pub fiftytwo_wk_low: f32,
    pub last: f32,
    pub percent_change: f32,
    pub net_change: f32,
    pub time: u64,
}

#[derive(Deserialize)]
pub struct Quotes {
    pub market_data: Vec<Quote>,
}

pub fn quote(client: &ureq::Agent, symbols: &[&str]) -> ApiResult<Quotes> {
    const API_URL: &str = "https://www.reuters.com/pf/api/v3/content/fetch/quote-by-rics-v2";

    const FIELDS: &str = "ric,\
        type:ricType,\
        name,\
        currency,\
        day_high:high,\
        day_low:low,\
        fiftytwo_wk_high:fiftyTwoWeekHigh,\
        fiftytwo_wk_low:fiftyTwoWeekLow,\
        last,\
        percent_change:pctChange,\
        net_change:netChange,\
        time:updatedTimeStamp";

    let query = format!(r#"{{"fields":"{FIELDS}","rics":"{}"}}"#, symbols.join(","));

    fetch_multiple(client, API_URL, &query)
}
