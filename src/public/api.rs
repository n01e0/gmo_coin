use crate::{endpoint, Pagenation, Symbol};
use serde::{Deserialize, Serialize};
use serde_json::Result;

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

pub fn status() -> Result<ExchangeStatus> {
    let path = "/v1/status";
    let resp = ureq::get(&format!("{}{}", endpoint::PUBLIC_API, path))
        .call()
        .into_json()
        .unwrap();
    serde_json::from_str(&resp["data"].to_string())
}

/// ## SymbolRate
/// 銘柄の最新レート
#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolRate {
    pub ask: String,
    pub bid: String,
    pub high: String,
    pub last: String,
    pub low: String,
    pub symbol: String,
    pub timestamp: String,
    pub volume: String,
}

/// ## LatestRate
/// 各銘柄の最新レート
#[derive(Debug, Serialize, Deserialize)]
pub struct LatestRate {
    pub status: usize,
    pub data: Vec<SymbolRate>,
    pub responsetime: String,
}

/// ### ticker
/// symbolの指定が無い場合、全銘柄のレートを取得する。
pub fn ticker(symbol: Option<Symbol>) -> Result<LatestRate> {
    let path = "/v1/ticker";
    let mut url = format!("{}{}", endpoint::PUBLIC_API, path);
    if let Some(symbol) = symbol {
        url = format!("{}?symbol={}", url, symbol);
    }
    let resp = ureq::get(&url).call();
    serde_json::from_str(&resp.into_string().unwrap())
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotResp {
    pub status: usize,
    pub data: Snapshot,
    pub responsetime: String,
}

pub fn orderbooks(symbol: Symbol) -> Result<SnapshotResp> {
    let path = "/v1/orderbooks";
    let url = format!("{}{}?symbol={}", endpoint::PUBLIC_API, path, symbol);
    let resp = ureq::get(&url).call();

    serde_json::from_str(&resp.into_string().unwrap())
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeListResp {
    pub pagenation: Pagenation,
    pub list: TradesList,
}

pub fn trades(symbol: Symbol, page: Option<usize>, count: Option<usize>) -> Result<TradeListResp> {
    let path = "/v1/trades";
    let mut query = format!("?symbol={}", symbol);
    if let Some(page) = page {
        query = format!("{}&page={}", query, page);
    }

    if let Some(count) = count {
        query = format!("{}&count={}", query, count);
    }

    let url = format!("{}{}{}", endpoint::PUBLIC_API, path, query);
    let resp = ureq::get(&url).call().into_json().unwrap();

    serde_json::from_str(&resp["data"].to_string())
}
