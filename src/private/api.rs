use crate::{endpoint, LeverageSymbol, Response, ResponseList, ResponsePage, Side, Symbol};
use ring::hmac;
use serde::{Deserialize, Serialize};
use serde_json::{json, Result, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fmt};

lazy_static! {
    static ref API_KEY: String =
        env::var("GMO_COIN_API_KEY").expect("You need set API key to GMO_COIN_API_KEY");
    static ref SECRET_KEY: String =
        env::var("GMO_COIN_SECRET_KEY").expect("You need set API secret to GMO_COIN_SECRET_KEY");
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
    let resp = get_without_params(path);

    serde_json::from_str(&resp.into_string().unwrap())
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
    let resp = get_without_params(path);

    serde_json::from_str(&resp.into_string().unwrap())
}

/// ## TradingVolume
/// 取引高情報
pub fn trading_volume() -> Result<Response<Value>> {
    let path = "/v1/account/tradingVolume";
    let resp = get_without_params(path);

    serde_json::from_str(&resp.into_string().unwrap())
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
    let resp = get_with_params(path, json!({ "orderId": order_id.to_string() }));

    serde_json::from_str(&resp.into_string().unwrap())
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

    let resp = get_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap())
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

    let resp = get_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap())
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

    let resp = get_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap())
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

    let resp = get_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap())
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

    let resp = get_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap())
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
) -> Response<String> {
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

    let resp = post_with_params(path, payload);

    serde_json::from_str(&resp.into_string().unwrap()).unwrap()
}

/// ## change_order
/// 注文変更
pub fn change_order(
    order_id: usize,
    price: String,
    losscut_price: Option<String>,
) -> ureq::Response {
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
pub fn cancel_order(order_id: usize) -> ureq::Response {
    let path = "/v1/cancelOrder";
    let query = json!({ "orderId": order_id });

    post_with_params(path, query)
}

/// ## cancel_orders
/// 複数注文のキャンセル
pub fn cancel_orders(order_ids: Vec<usize>) -> ureq::Response {
    let path = "/v1/cancelOrders";
    let query = json!({ "orderIds": order_ids });

    post_with_params(path, query)
}

/// ## cancel_bulk_order
/// 指定銘柄の一括キャンセル
pub fn cancel_bulk_order(symbols: Vec<Symbol>) -> ureq::Response {
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
) -> Response<String> {
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

    let resp = post_with_params(path, payload);
    serde_json::from_str(&resp.into_string().unwrap()).unwrap()
}

/// ## close_bulk_order
/// 一括決済注文
pub fn close_bulk_order(
    symbol: LeverageSymbol,
    side: Side,
    execution_type: ExecutionType,
    price: Option<String>,
    size: String,
) -> Response<String> {
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

    let resp = post_with_params(path, payload);
    serde_json::from_str(&resp.into_string().unwrap()).unwrap()
}

/// ## change_losscut_price
/// 建玉のロスカットレート変更
pub fn change_losscut_price(position_id: usize, losscut_price: String) -> ureq::Response {
    let path = "/v1/changeLosscutPrice";
    let query = json!({
        "positionId": position_id,
        "losscutPrice": losscut_price
    });

    post_with_params(path, query)
}

fn timestamp() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwords");

    since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1000000
}

fn get_with_params(path: &'static str, query: Value) -> ureq::Response {
    let timestamp = timestamp();
    let text = format!("{}GET{}", timestamp, path);
    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, SECRET_KEY.as_bytes());
    let sign = hex::encode(hmac::sign(&signed_key, text.as_bytes()).as_ref());

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    let mut request = ureq::get(&url);
    request.set("API-KEY", &API_KEY);
    request.set("API-TIMESTAMP", &format!("{}", timestamp));
    request.set("API-SIGN", &sign);

    for (key, value) in query_pairs(query) {
        request.query(&key, &value);
    }

    request.call()
}

fn post_with_params(path: &'static str, payload: Value) -> ureq::Response {
    let timestamp = timestamp();
    let body = payload.to_string();
    let text = format!("{}POST{}{}", timestamp, path, body);
    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, SECRET_KEY.as_bytes());
    let sign = hex::encode(hmac::sign(&signed_key, text.as_bytes()).as_ref());

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    ureq::post(&url)
        .set("content-type", "application/json")
        .set("API-KEY", &API_KEY)
        .set("API-TIMESTAMP", &format!("{}", timestamp))
        .set("API-SIGN", &sign)
        .send_string(&body)
}

fn get_without_params(path: &'static str) -> ureq::Response {
    let timestamp = timestamp();
    let text = format!("{}GET{}", timestamp, path);
    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, SECRET_KEY.as_bytes());
    let sign = hex::encode(hmac::sign(&signed_key, text.as_bytes()).as_ref());

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    ureq::get(&url)
        .set("API-KEY", &API_KEY)
        .set("API-TIMESTAMP", &format!("{}", timestamp))
        .set("API-SIGN", &sign)
        .call()
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
