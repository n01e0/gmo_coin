use crate::{
    LeverageSymbol, Response, ResponseList, ResponsePage, Side, Symbol, endpoint, error::Result,
};
use ring::hmac;
use serde_json::{Value, json};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use super::api::{
    ActiveOrders, Assets, Execution, ExecutionType, ExecutionsParam, LatestExecutions, Margin,
    OpenPositions, OrderInfo, PositionSummary, SettlePosition,
};

pub async fn margin() -> Result<Response<Margin>> {
    let path = "/v1/account/margin";
    let resp = get_without_params(path).await?;

    Ok(resp.json::<Response<Margin>>().await?)
}

pub async fn assets() -> Result<Response<Vec<Assets>>> {
    let path = "/v1/account/assets";
    let resp = get_without_params(path).await?;

    Ok(resp.json::<Response<Vec<Assets>>>().await?)
}

pub async fn trading_volume() -> Result<Response<Value>> {
    let path = "/v1/account/tradingVolume";
    let resp = get_without_params(path).await?;

    Ok(resp.json::<Response<Value>>().await?)
}

pub async fn orders<T: ToString>(order_id: T) -> Result<ResponseList<OrderInfo>> {
    let path = "/v1/orders";
    let resp = get_with_params(path, json!({ "orderId": order_id.to_string() })).await?;

    Ok(resp.json::<ResponseList<OrderInfo>>().await?)
}

pub async fn active_orders(
    symbol: Symbol,
    page: Option<usize>,
    count: Option<usize>,
) -> Result<ResponsePage<ActiveOrders>> {
    let path = "/v1/activeOrders";
    let query = json!({
        "symbol": format!("{}", symbol),
        "page": page.unwrap_or(1),
        "count": count.unwrap_or(100)
    });

    let resp = get_with_params(path, query).await?;
    Ok(resp.json::<ResponsePage<ActiveOrders>>().await?)
}

pub async fn executions(param: ExecutionsParam) -> Result<ResponseList<Execution>> {
    let path = "/v1/executions";
    let query = match param {
        ExecutionsParam::OrderId(value) => json!({ "orderId": value }),
        ExecutionsParam::ExecutionId(value) => json!({ "executionId": value }),
    };

    let resp = get_with_params(path, query).await?;
    Ok(resp.json::<ResponseList<Execution>>().await?)
}

pub async fn latest_executions(
    symbol: Symbol,
    page: Option<usize>,
    count: Option<usize>,
) -> Result<ResponsePage<LatestExecutions>> {
    let path = "/v1/latestExecutions";
    let query = json!({
        "symbol":format!("{}", symbol),
        "page":page.unwrap_or(1),
        "count":count.unwrap_or(100)
    });

    let resp = get_with_params(path, query).await?;
    Ok(resp.json::<ResponsePage<LatestExecutions>>().await?)
}

pub async fn open_positions(
    symbol: Symbol,
    page: Option<usize>,
    count: Option<usize>,
) -> Result<ResponsePage<OpenPositions>> {
    let path = "/v1/openPositions";
    let query = json!({
        "symbol":format!("{}", symbol),
        "page":page.unwrap_or(1),
        "count":count.unwrap_or(100)
    });

    let resp = get_with_params(path, query).await?;
    Ok(resp.json::<ResponsePage<OpenPositions>>().await?)
}

pub async fn position_summary(symbol: Symbol) -> Result<ResponseList<PositionSummary>> {
    let path = "/v1/positionSummary";
    let query = json!({ "symbol": format!("{}", symbol) });

    let resp = get_with_params(path, query).await?;
    Ok(resp.json::<ResponseList<PositionSummary>>().await?)
}

pub async fn order(
    symbol: Symbol,
    side: Side,
    execution_type: ExecutionType,
    price: Option<String>,
    size: String,
) -> Result<Response<String>> {
    let path = "/v1/order";
    let mut payload = json!({
        "symbol": format!("{}", symbol),
        "side":  format!("{}", side),
        "executionType": format!("{}", execution_type),
        "size": size
    });

    if let Some(price) = price {
        payload["price"] = json!(price);
    }

    let resp = post_with_params(path, payload).await?;

    Ok(resp.json::<Response<String>>().await?)
}

pub async fn change_order(
    order_id: usize,
    price: String,
    losscut_price: Option<String>,
) -> Result<reqwest::Response> {
    let path = "/v1/changeOrder";
    let mut payload = json!({
        "orderId":order_id,
        "price":price,
    });

    if let Some(losscut_price) = losscut_price {
        payload["losscutPrice"] = json!(losscut_price);
    }

    post_with_params(path, payload).await
}

pub async fn cancel_order(order_id: usize) -> Result<reqwest::Response> {
    let path = "/v1/cancelOrder";
    let query = json!({ "orderId": order_id });

    post_with_params(path, query).await
}

pub async fn cancel_orders(order_ids: Vec<usize>) -> Result<reqwest::Response> {
    let path = "/v1/cancelOrders";
    let query = json!({ "orderIds": order_ids });

    post_with_params(path, query).await
}

pub async fn cancel_bulk_order(symbols: Vec<Symbol>) -> Result<reqwest::Response> {
    let path = "/v1/cancelBulkOrder";
    let symbols: Vec<String> = symbols.into_iter().map(|s| s.to_string()).collect();
    let query = json!({ "symbols": symbols });

    post_with_params(path, query).await
}

pub async fn close_order(
    symbol: LeverageSymbol,
    side: Side,
    execution_type: ExecutionType,
    price: Option<String>,
    settle_position: SettlePosition,
) -> Result<Response<String>> {
    let path = "/v1/closeOrder";
    let mut payload = json!({
        "symbol": format!("{}", symbol),
        "side": format!("{}", side),
        "executionType": format!("{}", execution_type),
        "settlePosition": [
            {
                "positionId": settle_position.position_id,
                "size": settle_position.size
            }
        ]
    });

    if let Some(price) = price {
        payload["price"] = json!(price);
    }

    let resp = post_with_params(path, payload).await?;
    Ok(resp.json::<Response<String>>().await?)
}

pub async fn close_bulk_order(
    symbol: LeverageSymbol,
    side: Side,
    execution_type: ExecutionType,
    price: Option<String>,
    size: String,
) -> Result<Response<String>> {
    let path = "/v1/closeBulkOrder";
    let mut payload = json!({
        "symbol": format!("{}", symbol),
        "side": format!("{}", side),
        "executionType": format!("{}", execution_type),
        "size": size
    });

    if let Some(price) = price {
        payload["price"] = json!(price);
    }

    let resp = post_with_params(path, payload).await?;
    Ok(resp.json::<Response<String>>().await?)
}

pub async fn change_losscut_price(
    position_id: usize,
    losscut_price: String,
) -> Result<reqwest::Response> {
    let path = "/v1/changeLosscutPrice";
    let query = json!({
        "positionId": position_id,
        "losscutPrice": losscut_price
    });

    post_with_params(path, query).await
}

fn api_key() -> Result<String> {
    Ok(env::var("GMO_COIN_API_KEY")?)
}

fn secret_key() -> Result<String> {
    Ok(env::var("GMO_COIN_SECRET_KEY")?)
}

fn timestamp() -> Result<u64> {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH)?;

    Ok(since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1000000)
}

fn sign_request(timestamp: u64, method: &str, path: &str, body: &str, secret_key: &str) -> String {
    let text = format!("{}{}{}{}", timestamp, method, path, body);
    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, secret_key.as_bytes());
    hex::encode(hmac::sign(&signed_key, text.as_bytes()).as_ref())
}

async fn get_with_params(path: &'static str, query: Value) -> Result<reqwest::Response> {
    let timestamp = timestamp()?;
    let api_key = api_key()?;
    let secret_key = secret_key()?;
    let sign = sign_request(timestamp, "GET", path, "", &secret_key);

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    let client = reqwest::Client::new();

    let mut request = client
        .get(&url)
        .header("API-KEY", api_key)
        .header("API-TIMESTAMP", format!("{}", timestamp))
        .header("API-SIGN", sign);

    for (key, value) in query_pairs(query) {
        request = request.query(&[(key, value)]);
    }

    Ok(request.send().await?.error_for_status()?)
}

async fn post_with_params(path: &'static str, payload: Value) -> Result<reqwest::Response> {
    let timestamp = timestamp()?;
    let api_key = api_key()?;
    let secret_key = secret_key()?;
    let body = payload.to_string();
    let sign = sign_request(timestamp, "POST", path, &body, &secret_key);

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    Ok(reqwest::Client::new()
        .post(&url)
        .header("content-type", "application/json")
        .header("API-KEY", api_key)
        .header("API-TIMESTAMP", format!("{}", timestamp))
        .header("API-SIGN", sign)
        .body(body)
        .send()
        .await?
        .error_for_status()?)
}

async fn get_without_params(path: &'static str) -> Result<reqwest::Response> {
    let timestamp = timestamp()?;
    let api_key = api_key()?;
    let secret_key = secret_key()?;
    let sign = sign_request(timestamp, "GET", path, "", &secret_key);

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    Ok(reqwest::Client::new()
        .get(&url)
        .header("API-KEY", api_key)
        .header("API-TIMESTAMP", format!("{}", timestamp))
        .header("API-SIGN", sign)
        .send()
        .await?
        .error_for_status()?)
}

fn query_pairs(query: Value) -> Vec<(String, String)> {
    match query {
        Value::Object(map) => map
            .into_iter()
            .filter_map(|(key, value)| value_to_query_string(value).map(|value| (key, value)))
            .collect(),
        _ => Vec::new(),
    }
}

fn value_to_query_string(value: Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::Bool(value) => Some(value.to_string()),
        Value::Number(value) => Some(value.to_string()),
        Value::String(value) => Some(value),
        Value::Array(values) => {
            let values: Vec<String> = values
                .into_iter()
                .filter_map(value_to_query_string)
                .collect();
            Some(values.join(","))
        }
        Value::Object(_) => None,
    }
}
