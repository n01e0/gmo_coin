use crate::{endpoint, Pagenation, Response, Symbol};
use serde::{Deserialize, Serialize};
use serde_json::Result;

type UreqResponse = ureq::http::Response<ureq::Body>;

fn response_body(mut response: UreqResponse) -> String {
    response.body_mut().read_to_string().unwrap()
}

/// ## ExchangeStatus
/// 取引所の稼動状態
///
///  - status
///     - `MAINTENANCE`
///     - `PREOPEN`
///     - `OPEN`
#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeStatus {
    pub status: String,
}

pub fn status() -> Result<Response<ExchangeStatus>> {
    let path = "/v1/status";
    let resp = ureq::get(&format!("{}{}", endpoint::PUBLIC_API, path))
        .call()
        .unwrap();
    serde_json::from_str(&response_body(resp))
}

/// ## SymbolRate
/// 銘柄の最新レート
#[derive(Debug, Serialize, Deserialize)]
pub struct LatestRate {
    pub ask: String,
    pub bid: String,
    pub high: String,
    pub last: String,
    pub low: String,
    pub symbol: String,
    pub timestamp: String,
    pub volume: String,
}

/// ### ticker
/// symbolの指定が無い場合、全銘柄のレートを取得する。
pub fn ticker(symbol: Option<Symbol>) -> Result<Response<Vec<LatestRate>>> {
    let path = "/v1/ticker";
    let mut url = format!("{}{}", endpoint::PUBLIC_API, path);
    if let Some(symbol) = symbol {
        url = format!("{}?symbol={}", url, symbol);
    }
    let resp = ureq::get(&url).call().unwrap();
    serde_json::from_str(&response_body(resp))
}

/// ### Ask
/// for snapshot
#[derive(Debug, Serialize, Deserialize)]
pub struct Ask {
    pub price: String,
    pub size: String,
}

/// ### Bid
/// for snapshot
#[derive(Debug, Serialize, Deserialize)]
pub struct Bid {
    pub price: String,
    pub size: String,
}

/// ## Snapshot
/// 板情報
/// symbolの指定が無い場合はBTC_JPYを取得する
#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub asks: Vec<Ask>,
    pub bids: Vec<Bid>,
    pub symbol: String,
}

pub fn orderbooks(symbol: Symbol) -> Result<Response<Snapshot>> {
    let path = "/v1/orderbooks";
    let url = format!("{}{}?symbol={}", endpoint::PUBLIC_API, path, symbol);
    let resp = ureq::get(&url).call().unwrap();

    serde_json::from_str(&response_body(resp))
}

/// ## Trade
/// 取引履歴
///
/// ### price
/// 約定価格
///
/// ### page
/// 売買区分
///
/// ### size
/// 約定数量
///
/// ### timestamp
/// 約定日時
#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub price: String,
    pub side: String,
    pub size: String,
    pub timestamp: String,
}

/// ## TradesList
/// 取引履歴のリスト
#[derive(Debug, Serialize, Deserialize)]
pub struct TradesList {
    pub pagination: Pagenation,
    pub list: Vec<Trade>,
}

pub fn trades(
    symbol: Symbol,
    page: Option<usize>,
    count: Option<usize>,
) -> Result<Response<TradesList>> {
    let path = "/v1/trades";
    let mut query = format!("?symbol={}", symbol);
    if let Some(page) = page {
        query = format!("{}&page={}", query, page);
    }

    if let Some(count) = count {
        query = format!("{}&count={}", query, count);
    }

    let url = format!("{}{}{}", endpoint::PUBLIC_API, path, query);
    let resp = ureq::get(&url).call().unwrap();

    serde_json::from_str(&response_body(resp))
}

/// ## KlineInterval
/// ローソク足の間隔
#[derive(Debug)]
pub enum KlineInterval {
    OneMin,
    FiveMin,
    TenMin,
    FifteenMin,
    ThirtyMin,
    OneHour,
    FourHour,
    EightHour,
    TwelveHour,
    OneDay,
    OneWeek,
    OneMonth,
}

impl std::fmt::Display for self::KlineInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            KlineInterval::OneMin => write!(f, "1min"),
            KlineInterval::FiveMin => write!(f, "5min"),
            KlineInterval::TenMin => write!(f, "10min"),
            KlineInterval::FifteenMin => write!(f, "15min"),
            KlineInterval::ThirtyMin => write!(f, "30min"),
            KlineInterval::OneHour => write!(f, "1hour"),
            KlineInterval::FourHour => write!(f, "4hour"),
            KlineInterval::EightHour => write!(f, "8hour"),
            KlineInterval::TwelveHour => write!(f, "12hour"),
            KlineInterval::OneDay => write!(f, "1day"),
            KlineInterval::OneWeek => write!(f, "1week"),
            KlineInterval::OneMonth => write!(f, "1month"),
        }
    }
}

/// ## Kline
/// ローソク足
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    pub open_time: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

/// ## klines
/// ローソク足を取得する
///
/// ### Params
///  - symbol: 銘柄
///  - interval: 間隔（例: 1min）
///  - date: 対象日（YYYYMMDD）
pub fn klines(
    symbol: Symbol,
    interval: KlineInterval,
    date: String,
) -> Result<Response<Vec<Kline>>> {
    let path = "/v1/klines";
    let url = format!(
        "{}{}?symbol={}&interval={}&date={}",
        endpoint::PUBLIC_API,
        path,
        symbol,
        interval,
        date
    );
    let resp = ureq::get(&url).call().unwrap();

    serde_json::from_str(&response_body(resp))
}

/// ## SymbolRule
/// 取引ルール
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolRule {
    pub symbol: String,
    pub min_order_size: String,
    pub max_order_size: String,
    pub size_step: String,
    pub tick_size: String,
    pub taker_fee: String,
    pub maker_fee: String,
}

/// ## symbols
/// 取引ルールを取得する
pub fn symbols() -> Result<Response<Vec<SymbolRule>>> {
    let path = "/v1/symbols";
    let resp = ureq::get(&format!("{}{}", endpoint::PUBLIC_API, path))
        .call()
        .unwrap();

    serde_json::from_str(&response_body(resp))
}
