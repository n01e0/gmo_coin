use crate::{Response, Symbol, endpoint, error::Result};
use serde::de::DeserializeOwned;

use super::api::{
    ExchangeStatus, Kline, KlineInterval, LatestRate, Snapshot, SymbolRule, TradesList,
};

pub async fn status() -> Result<Response<ExchangeStatus>> {
    let path = "/v1/status";
    get_json(format!("{}{}", endpoint::PUBLIC_API, path)).await
}

pub async fn ticker(symbol: Option<Symbol>) -> Result<Response<Vec<LatestRate>>> {
    let path = "/v1/ticker";
    let mut url = format!("{}{}", endpoint::PUBLIC_API, path);
    if let Some(symbol) = symbol {
        url = format!("{}?symbol={}", url, symbol);
    }

    get_json(url).await
}

pub async fn orderbooks(symbol: Symbol) -> Result<Response<Snapshot>> {
    let path = "/v1/orderbooks";
    let url = format!("{}{}?symbol={}", endpoint::PUBLIC_API, path, symbol);

    get_json(url).await
}

pub async fn trades(
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

    get_json(url).await
}

pub async fn klines(
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

    get_json(url).await
}

pub async fn symbols() -> Result<Response<Vec<SymbolRule>>> {
    let path = "/v1/symbols";
    get_json(format!("{}{}", endpoint::PUBLIC_API, path)).await
}

async fn get_json<T: DeserializeOwned>(url: String) -> Result<T> {
    Ok(reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json::<T>()
        .await?)
}
