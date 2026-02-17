use gmo_coin::Symbol;
use gmo_coin::error::Result;
use gmo_coin::public::api::{self, KlineInterval};
use std::thread::sleep;
use std::time::Duration;

fn call_with_retry<T, F>(name: &str, mut f: F) -> T
where
    F: FnMut() -> Result<T>,
{
    let mut last_err = None;

    for _ in 0..8 {
        match f() {
            Ok(resp) => return resp,
            Err(err) => {
                last_err = Some(err);
                sleep(Duration::from_millis(500));
            }
        }
    }

    panic!("{} failed after retries: {:?}", name, last_err);
}

#[test]
fn status_returns_known_exchange_status() {
    let resp = call_with_retry("public::api::status", api::status);

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

#[test]
fn ticker_returns_btc_when_symbol_is_btc() {
    let resp = call_with_retry("public::api::ticker", || api::ticker(Some(Symbol::BTC)));

    assert_eq!(resp.status, 0);
    assert!(!resp.data.is_empty(), "ticker response must not be empty");
    assert_eq!(resp.data[0].symbol, "BTC");
}

#[test]
fn orderbooks_returns_asks_and_bids() {
    let resp = call_with_retry("public::api::orderbooks", || api::orderbooks(Symbol::BTC));

    assert_eq!(resp.status, 0);
    assert_eq!(resp.data.symbol, "BTC");
    assert!(!resp.data.asks.is_empty(), "asks must not be empty");
    assert!(!resp.data.bids.is_empty(), "bids must not be empty");
}

#[test]
fn trades_returns_non_empty_list() {
    let resp = call_with_retry("public::api::trades", || {
        api::trades(Symbol::BTC, Some(1), Some(5))
    });

    assert_eq!(resp.status, 0);
    assert!(!resp.data.list.is_empty(), "trades list must not be empty");
    assert!(
        matches!(resp.data.list[0].side.as_str(), "BUY" | "SELL"),
        "unexpected trade side: {}",
        resp.data.list[0].side
    );
}

#[test]
fn symbols_contains_core_symbols() {
    let resp = call_with_retry("public::api::symbols", api::symbols);

    assert_eq!(resp.status, 0);
    assert!(!resp.data.is_empty(), "symbols must not be empty");

    let has_btc = resp.data.iter().any(|s| s.symbol == "BTC");
    let has_btc_jpy = resp.data.iter().any(|s| s.symbol == "BTC_JPY");

    assert!(has_btc, "symbols should include BTC");
    assert!(has_btc_jpy, "symbols should include BTC_JPY");
}

#[test]
fn klines_returns_candles_for_sample_date() {
    let resp = call_with_retry("public::api::klines", || {
        api::klines(Symbol::BTC, KlineInterval::OneMin, "20210417".to_string())
    });

    assert_eq!(resp.status, 0);
    assert!(!resp.data.is_empty(), "klines must not be empty");
    assert!(
        !resp.data[0].open_time.is_empty(),
        "open_time must not be empty"
    );
}
