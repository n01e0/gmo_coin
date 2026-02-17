use crate::{
    LeverageSymbol, Response, ResponseList, ResponsePage, Side, Symbol, endpoint,
    error::{GmoCoinError, Result},
};
use ring::hmac;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fmt};
use ureq::http::{HeaderMap, header::HeaderValue};

type UreqResponse = ureq::http::Response<ureq::Body>;

fn response_body(mut response: UreqResponse) -> Result<String> {
    Ok(response.body_mut().read_to_string()?)
}

fn parse_response<T: DeserializeOwned>(response: UreqResponse) -> Result<T> {
    let body = response_body(response)?;
    Ok(serde_json::from_str(&body)?)
}

fn api_key() -> Result<String> {
    Ok(env::var("GMO_COIN_API_KEY")?)
}

fn secret_key() -> Result<String> {
    Ok(env::var("GMO_COIN_SECRET_KEY")?)
}

fn insert_header(
    headers: &mut HeaderMap<HeaderValue>,
    key: &'static str,
    value: &str,
) -> Result<()> {
    headers.insert(key, HeaderValue::from_str(value)?);
    Ok(())
}

fn set_request_headers<B>(
    request: &mut ureq::RequestBuilder<B>,
    timestamp: u64,
    sign: &str,
    api_key: &str,
) -> Result<()> {
    let timestamp_string = timestamp.to_string();
    let headers = request
        .headers_mut()
        .ok_or(GmoCoinError::MissingRequestHeaders)?;
    insert_header(headers, "API-KEY", api_key)?;
    insert_header(headers, "API-TIMESTAMP", &timestamp_string)?;
    insert_header(headers, "API-SIGN", sign)?;
    Ok(())
}

fn sign_request(timestamp: u64, method: &str, path: &str, body: &str, secret_key: &str) -> String {
    let text = format!("{}{}{}{}", timestamp, method, path, body);
    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, secret_key.as_bytes());
    hex::encode(hmac::sign(&signed_key, text.as_bytes()).as_ref())
}

/// ## Margin
/// 余力情報
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Margin {
    pub actual_profit_loss: String,
    pub available_amount: String,
    pub margin: String,
    pub profit_loss: String,
}

pub fn margin() -> Result<Response<Margin>> {
    let path = "/v1/account/margin";
    let resp = get_without_params(path)?;

    parse_response(resp)
}

/// ## Asset
/// 資産残高
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assets {
    pub amount: String,
    pub available: String,
    pub conversion_rate: String,
    pub symbol: String,
}

pub fn assets() -> Result<Response<Vec<Assets>>> {
    let path = "/v1/account/assets";
    let resp = get_without_params(path)?;

    parse_response(resp)
}

/// ## TradingVolume
/// 取引高情報
pub fn trading_volume() -> Result<Response<Value>> {
    let path = "/v1/account/tradingVolume";
    let resp = get_without_params(path)?;

    parse_response(resp)
}

/// ## Order
/// 注文情報
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub root_order_id: usize,
    pub order_id: usize,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub execution_type: String,
    pub settle_type: String,
    pub size: String,
    pub executed_size: String,
    pub price: String,
    pub losscut_price: String,
    pub status: String,
    pub time_in_force: String,
    pub timestamp: String,
}

/// ## orders
/// 注文情報(OrderInfo)の取得
///
/// ### Parameters
///  - order_id: 注文ID（カンマ区切りで複数指定可）
pub fn orders<T: ToString>(order_id: T) -> Result<ResponseList<OrderInfo>> {
    let path = "/v1/orders";
    let resp = get_with_params(path, json!({ "orderId": order_id.to_string() }))?;

    parse_response(resp)
}

/// ## ActiveOrder
/// 有効注文
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrders {
    pub root_order_id: usize,
    pub order_id: usize,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub execution_type: String,
    pub settle_type: String,
    pub size: String,
    pub executed_size: String,
    pub price: String,
    pub losscut_price: String,
    pub status: String,
    pub time_in_force: String,
    pub timestamp: String,
}

/// ## active_orders
/// 有効注文(ActiveOrders)の取得
pub fn active_orders(
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

    let resp = get_with_params(path, query)?;
    parse_response(resp)
}

/// ## Execution
/// 約定情報
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    pub execution_id: usize,
    pub order_id: usize,
    pub symbol: String,
    pub side: String,
    pub settle_type: String,
    pub size: String,
    pub price: String,
    pub loss_gain: String,
    pub fee: String,
    pub timestamp: String,
}

/// ## ExecutionsParam
/// Execution取得のためのパラメタ
pub enum ExecutionsParam {
    OrderId(String),
    ExecutionId(String),
}

impl ExecutionsParam {
    pub fn order_id<T: ToString>(value: T) -> Self {
        ExecutionsParam::OrderId(value.to_string())
    }

    pub fn execution_id<T: ToString>(value: T) -> Self {
        ExecutionsParam::ExecutionId(value.to_string())
    }
}

impl fmt::Display for self::ExecutionsParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionsParam::OrderId(_) => write!(f, "orderId"),
            ExecutionsParam::ExecutionId(_) => write!(f, "executionId"),
        }
    }
}

/// ## executions
/// 約定情報(Execution)の取得
pub fn executions(param: ExecutionsParam) -> Result<ResponseList<Execution>> {
    let path = "/v1/executions";
    let query = match param {
        ExecutionsParam::OrderId(value) => json!({ "orderId": value }),
        ExecutionsParam::ExecutionId(value) => json!({ "executionId": value }),
    };

    let resp = get_with_params(path, query)?;
    parse_response(resp)
}

/// ## LatestExecution
/// 最新約定一覧
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestExecutions {
    pub execution_id: usize,
    pub order_id: usize,
    pub symbol: String,
    pub side: String,
    pub settle_type: String,
    pub size: String,
    pub price: String,
    pub loss_gain: String,
    pub fee: String,
    pub timestamp: String,
}

/// ## latest_executions
/// 最新約定一覧取得
pub fn latest_executions(
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

    let resp = get_with_params(path, query)?;
    parse_response(resp)
}

/// ## OpenPosition
/// 有効建玉
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenPositions {
    pub position_id: usize,
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub order_size: String,
    pub price: String,
    pub loss_gain: String,
    pub leverage: String,
    pub losscut_price: String,
    pub timestamp: String,
}

/// ## open_positions
/// 有効建玉の取得
pub fn open_positions(
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

    let resp = get_with_params(path, query)?;
    parse_response(resp)
}

/// ## PositionSummary
/// 建玉サマリ
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionSummary {
    pub average_position_rate: String,
    pub position_loss_gain: String,
    pub side: String,
    pub sum_order_quantity: String,
    pub sum_position_quantity: String,
    pub symbol: String,
}

/// ## position_summary
/// 建玉サマリの取得
pub fn position_summary(symbol: Symbol) -> Result<ResponseList<PositionSummary>> {
    let path = "/v1/positionSummary";
    let query = json!({ "symbol": format!("{}", symbol) });

    let resp = get_with_params(path, query)?;
    parse_response(resp)
}

#[derive(Debug)]
pub enum ExecutionType {
    MARKET,
    LIMIT,
    STOP,
}

impl fmt::Display for self::ExecutionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionType::MARKET => write!(f, "MARKET"),
            ExecutionType::LIMIT => write!(f, "LIMIT"),
            ExecutionType::STOP => write!(f, "STOP"),
        }
    }
}

/// ## order
/// 新規注文
pub fn order(
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

    let resp = post_with_params(path, payload)?;

    parse_response(resp)
}

/// ## change_order
/// 注文変更
pub fn change_order(
    order_id: usize,
    price: String,
    losscut_price: Option<String>,
) -> Result<UreqResponse> {
    let path = "/v1/changeOrder";
    let mut payload = json!({
        "orderId":order_id,
        "price":price,
    });

    if let Some(losscut_price) = losscut_price {
        payload["losscutPrice"] = json!(losscut_price);
    }

    post_with_params(path, payload)
}

/// ## cancel_order
/// 注文キャンセル
pub fn cancel_order(order_id: usize) -> Result<UreqResponse> {
    let path = "/v1/cancelOrder";
    let query = json!({ "orderId": order_id });

    post_with_params(path, query)
}

/// ## cancel_orders
/// 複数注文のキャンセル
pub fn cancel_orders(order_ids: Vec<usize>) -> Result<UreqResponse> {
    let path = "/v1/cancelOrders";
    let query = json!({ "orderIds": order_ids });

    post_with_params(path, query)
}

/// ## cancel_bulk_order
/// 指定銘柄の一括キャンセル
pub fn cancel_bulk_order(symbols: Vec<Symbol>) -> Result<UreqResponse> {
    let path = "/v1/cancelBulkOrder";
    let symbols: Vec<String> = symbols.into_iter().map(|s| s.to_string()).collect();
    let query = json!({ "symbols": symbols });

    post_with_params(path, query)
}

/// ## SettlePosition
/// レバレッジ取引で使う奴
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlePosition {
    pub position_id: usize,
    pub size: String,
}

/// ## close_order
/// 決済注文
pub fn close_order(
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

    let resp = post_with_params(path, payload)?;
    parse_response(resp)
}

/// ## close_bulk_order
/// 一括決済注文
pub fn close_bulk_order(
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

    let resp = post_with_params(path, payload)?;
    parse_response(resp)
}

/// ## change_losscut_price
/// 建玉のロスカットレート変更
pub fn change_losscut_price(position_id: usize, losscut_price: String) -> Result<UreqResponse> {
    let path = "/v1/changeLosscutPrice";
    let query = json!({
        "positionId": position_id,
        "losscutPrice": losscut_price
    });

    post_with_params(path, query)
}

fn timestamp() -> Result<u64> {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH)?;

    Ok(since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1000000)
}

fn get_with_params(path: &'static str, query: Value) -> Result<UreqResponse> {
    let timestamp = timestamp()?;
    let api_key = api_key()?;
    let secret_key = secret_key()?;
    let sign = sign_request(timestamp, "GET", path, "", &secret_key);

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    let mut request = ureq::get(&url);
    set_request_headers(&mut request, timestamp, &sign, &api_key)?;

    for (key, value) in query_pairs(query) {
        request = request.query(&key, &value);
    }

    Ok(request.call()?)
}

fn post_with_params(path: &'static str, payload: Value) -> Result<UreqResponse> {
    let timestamp = timestamp()?;
    let api_key = api_key()?;
    let secret_key = secret_key()?;
    let body = payload.to_string();
    let sign = sign_request(timestamp, "POST", path, &body, &secret_key);

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    let mut request = ureq::post(&url).content_type("application/json");
    set_request_headers(&mut request, timestamp, &sign, &api_key)?;

    Ok(request.send(&body)?)
}

fn get_without_params(path: &'static str) -> Result<UreqResponse> {
    let timestamp = timestamp()?;
    let api_key = api_key()?;
    let secret_key = secret_key()?;
    let sign = sign_request(timestamp, "GET", path, "", &secret_key);

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    let mut request = ureq::get(&url);
    set_request_headers(&mut request, timestamp, &sign, &api_key)?;

    Ok(request.call()?)
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
