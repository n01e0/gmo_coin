#![cfg(feature = "async")]

use gmo_coin::public::api::KlineInterval;
use gmo_coin::public::async_api;
use gmo_coin::Symbol;
use std::future::Future;
use std::time::Duration;

async fn call_with_retry<T, F, Fut>(name: &str, mut f: F) -> T
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, reqwest::Error>>,
{
    let mut last_err = None;

    for _ in 0..8 {
        match f().await {
            Ok(resp) => return resp,
            Err(err) => {
                last_err = Some(err);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    panic!("{} failed after retries: {:?}", name, last_err);
}

#[tokio::test]
async fn status_returns_known_exchange_status() {
    let resp = call_with_retry("public::async_api::status", async_api::status).await;

    assert_eq!(resp.status, 0);
    assert!(
        matches!(
            resp.data.status.as_str(),
            "OPEN" | "PREOPEN" | "MAINTENANCE"
        ),
        "unexpected exchange status: {}",
        resp.data.status
    );
}

#[tokio::test]
async fn ticker_returns_btc_when_symbol_is_btc() {
    let resp = call_with_retry("public::async_api::ticker", || async {
        async_api::ticker(Some(Symbol::BTC)).await
    })
    .await;

    assert_eq!(resp.status, 0);
    assert!(!resp.data.is_empty(), "ticker response must not be empty");
    assert_eq!(resp.data[0].symbol, "BTC");
}

#[tokio::test]
async fn orderbooks_returns_asks_and_bids() {
    let resp = call_with_retry("public::async_api::orderbooks", || async {
        async_api::orderbooks(Symbol::BTC).await
    })
    .await;

    assert_eq!(resp.status, 0);
    assert_eq!(resp.data.symbol, "BTC");
    assert!(!resp.data.asks.is_empty(), "asks must not be empty");
    assert!(!resp.data.bids.is_empty(), "bids must not be empty");
}

#[tokio::test]
async fn trades_returns_non_empty_list() {
    let resp = call_with_retry("public::async_api::trades", || async {
        async_api::trades(Symbol::BTC, Some(1), Some(5)).await
    })
    .await;

    assert_eq!(resp.status, 0);
    assert!(!resp.data.list.is_empty(), "trades list must not be empty");
    assert!(
        matches!(resp.data.list[0].side.as_str(), "BUY" | "SELL"),
        "unexpected trade side: {}",
        resp.data.list[0].side
    );
}

#[tokio::test]
async fn symbols_contains_core_symbols() {
    let resp = call_with_retry("public::async_api::symbols", async_api::symbols).await;

    assert_eq!(resp.status, 0);
    assert!(!resp.data.is_empty(), "symbols must not be empty");

    let has_btc = resp.data.iter().any(|s| s.symbol == "BTC");
    let has_btc_jpy = resp.data.iter().any(|s| s.symbol == "BTC_JPY");

    assert!(has_btc, "symbols should include BTC");
    assert!(has_btc_jpy, "symbols should include BTC_JPY");
}

#[tokio::test]
async fn klines_returns_candles_for_sample_date() {
    let resp = call_with_retry("public::async_api::klines", || async {
        async_api::klines(Symbol::BTC, KlineInterval::OneMin, "20210417".to_string()).await
    })
    .await;

    assert_eq!(resp.status, 0);
    assert!(!resp.data.is_empty(), "klines must not be empty");
    assert!(
        !resp.data[0].open_time.is_empty(),
        "open_time must not be empty"
    );
}
