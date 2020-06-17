use crate::{endpoint, LeverageSymbol, Symbol, Response, ResponsePage, ResponseList};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Result, json};
use ring::hmac;
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
///
///  - actualProfitLoss: 時価評価総額
///  - availableAmount:  取引余力
///  - Margin:           拘束証拠金
///  - profitLoss:       評価損益
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
///
///  - amount:          残高
///  - available:       利用可能金額(残高 - 出金予定額)
///  - conversionRate:  円転レート
///  - symbol:          銘柄名
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

/// ## Order
/// 注文情報
///
///  - root_order_id:   親注文ID
///  - order_id:        注文ID
///  - symbol:          銘柄名
///  - side:            売買区分
///  - order_type:      取引区分
///  - execution_type:  注文タイプ
///  - settle_type:     決済区分
///  - size:            発注数量
///  - executed_size:   約定数量
///  - price:           注文価格
///  - losscut_price:   ロスカットレート
///  - status:          注文ステータス
///  - time_in_force:   執行数量条件
///  - timestamp:       注文日時
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
///
///  - order_id:        オーダーID

pub fn orders(order_id: usize) -> Result<ResponseList<OrderInfo>> {
    let path = "/v1/orders";
    let resp = get_with_params(path, json!({ "orderId": order_id }));

    serde_json::from_str(&resp.into_string().unwrap())
}

/// ## ActiveOrder
/// 有効注文
///
///  - root_order_id:   親注文ID
///  - order_id:        注文ID
///  - symbol:          銘柄名
///  - side:            売買区分
///  - order_type:      取引区分
///  - execution_type:  注文タイプ
///  - settle_type:     決済区分
///  - size:            発注数量
///  - executed_size:   約定数量
///  - price:           注文価格
///  - losscut_price:   ロスカットレート
///  - status:          注文ステータス
///  - time_in_force:   執行数量条件
///  - timestamp:       注文日時
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
///
/// ### Parameters
///  - symbol:  シンボル
///  - page:    取得対象ページ(デフォルトでは1)
///  - count:   1ページあたりの取得件数(デフォルトは100(最大値))
pub fn active_orders(
    symbol: Symbol,
    page: Option<usize>,
    count: Option<usize>,
) -> Result<ResponsePage<ActiveOrders>> {
    let path = "/v1/activeOrders";
    let query = json!({
        "symbol": format!("{}", symbol),
        "page": page.unwrap_or(1),
        "count": count.unwrap_or(1)
    });

    let resp = get_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap())
}

/// ## Execution
/// 約定情報
///
///  - execution_id:    約定ID
///  - order_id:        注文ID
///  - symbol:          銘柄名
///  - side:            売買区分
///  - settle_type:     決済区分
///  - size:            約定数量
///  - price:           約定レート
///  - loss_gain:       決済損益
///  - fee:             取引手数料
///  - timestamp:       約定日時
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
///
///  - order_id:        注文ID
///  - execution_id:    約定ID
pub enum ExecutionsParam {
    OrderId(usize),
    ExecutionId(usize),
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
///
/// ### Params
/// ExecutionsParam

pub fn executions(param: ExecutionsParam) -> Result<ResponseList<Execution>> {
    let path = "/v1/executions";
    let query = json!({
        "param":match param { ExecutionsParam::OrderId(n) => n, ExecutionsParam::ExecutionId(n) => n, }});

    let resp = get_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap())
}

/// ## LatestExecution
/// 最新約定一覧
///
///  - execution_id:    約定ID
///  - order_id:        注文ID
///  - symbol:          銘柄名
///  - side:            売買区分
///  - settle_type:     決済区分
///  - size:            約定数量
///  - price:           約定レート
///  - loss_gain:       決済損益
///  - fee:             取引手数料
///  - timestamp:       約定日時
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
///
/// ### Params
///  - symbol:  銘柄
///  - page:    取得対象ページ(デフォルトでは1)
///  - count:   1ページあたりの取得件数
pub fn latest_executions(
    symbol: Symbol,
    page: Option<usize>,
    count: Option<usize>,
) -> Result<ResponsePage<LatestExecutions>> {
    let path = "/v1/latestExecutions";
    let query = json!({
        "symbol":format!("{}", symbol),
        "page":page.unwrap_or(1),
        "count":count.unwrap_or(1)
    });

    let resp = get_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap())
}

/// ## OpenPosition
/// 有効建玉
///
///  - position_id:     建玉ID
///  - symbol:          銘柄名
///  - side:            売買区分
///  - size:            建玉数量
///  - order_size:      発注数量
///  - price:           建玉レート
///  - loss_gain:       評価損益
///  - leverage:        レバレッジ
///  - losscut_price:   ロスカットレート
///  - timestamp:       約定日時
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
///
/// ### Params
///  - symbol:  銘柄
///  - page:    取得対象ページ
///  - count:   取得件数
pub fn open_positions(
    symbol: Symbol,
    page: Option<usize>,
    count: Option<usize>,
) -> Result<ResponsePage<OpenPositions>> {
    let path = "/v1/openPositions";
    let query = json!({
        "symbol":format!("{}", symbol),
        "page":format!("{}", page.unwrap_or(1)),
        "count":format!("{}", count.unwrap_or(1))
    });

    let resp = get_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap())
}

/// ## PositionSummary
/// 建玉サマリ
///
///  - average_position_rate:   平均建玉レート
///  - position_loss_gain:      評価損益
///  - side:                    売買区分
///  - sum_order_quantity:      発注中数量
///  - sum_position_quantity:   建玉数量
///  - symbol:                  銘柄名
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
///
/// ### Params
///  - symbol: 銘柄名
pub fn position_summary(symbol: Symbol) -> Result<ResponseList<PositionSummary>> {
    let path = "/v1/positionSummary";
    let query = json!({ "symbol": format!("{}", symbol) });

    let resp = get_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap())
}

/// ## Order
/// 注文
///
///  - data: 注文対象のorderId
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Side {
    BUY,
    SELL,
}

impl fmt::Display for self::Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::BUY => write!(f, "BUY"),
            Side::SELL => write!(f, "SELL"),
        }
    }
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
///
/// ### params
///  - symbol:          銘柄名
///  - side:            BUY/SELL
///  - execution_type:  MARKET/LIMIT/STOP
///  - price:           LIMIT/STOP の場合は必要
///  - size:            量
pub fn order(
    symbol: Symbol,
    side: Side,
    execution_type: ExecutionType,
    price: Option<String>,
    size: String,
) -> Response<String> {
    let path = "/v1/order";
    let query = json!({
            "symbol": format!("{}", symbol),
            "side":  format!("{}", side),
            "executionType": format!("{}", execution_type),
            "price": format!("{}", price.unwrap_or("".to_string())),
            "size": format!("{}", size)
    });

    let resp = post_with_params(path, query);

    serde_json::from_str(&resp.into_string().unwrap()).unwrap()
}

/// ## change_order
/// 注文変更
///
/// ### Params
///  - order_id:        注文ID
///  - price:           価格
///  - losscut_price:   ロスカットレート
pub fn change_order(
    order_id: usize,
    price: String,
    losscut_price: Option<String>,
) -> ureq::Response {
    let path = "/v1/order";
    let query = json!({
        "orderId":order_id,
        "price":price,
        "losscutPrice":format!("{}", losscut_price.unwrap_or("".to_string()))
    });

    post_with_params(path, query)
}

/// ## cancel_order
/// 注文キャンセル
///
/// ### Params
///  - order_id: 注文ID
pub fn cancel_order(order_id: usize) -> ureq::Response {
    let path = "/v1/cancelOrder";
    let query = json!({ "orderId": order_id });

    post_with_params(path, query)
}

/// ## SettlePosition
/// レバレッジ取引で使う奴
///
///  - position_id
///  - size
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlePosition {
    pub position_id: usize,
    pub size: String,
}

/// ## close_order
/// 決済注文
/// 対象: レバレッジ取引
///
/// ### Params
///  - symbol
///  - side
///  - execution_type
///  - price
///  - settle_position: SettlePosition
pub fn close_order(
    symbol: LeverageSymbol,
    side: Side,
    execution_type: ExecutionType,
    price: Option<String>,
    settle_position: SettlePosition,
) -> Response<String> {
    let path = "/v1/closeOrder";
    let query = json!({
        "symbol": format!("{}", symbol),
        "side": format!("{}", side),
        "executionType": format!("{}", execution_type),
        "price": format!("{}", price.unwrap_or("".to_string())),
        "settlePosition": [
            {
                "positionId": settle_position.position_id,
                "size": settle_position.size
            }
        ]
    });

    let resp = post_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap()).unwrap()
}

/// ## close_bulk_order
/// 一括決済注文
///
/// ### Params
///  - symbol
///  - side
///  - execution_type
///  - price
///  - size
pub fn close_bulk_order(
    symbol: LeverageSymbol,
    side: Side,
    execution_type: ExecutionType,
    price: Option<String>,
    size: String,
) -> Response<String> {
    let path = "/v1/closeBulkOrder";
    let query = json!({
        "symbol": format!("{}", symbol),
        "side": format!("{}", side),
        "executionType": format!("{}", execution_type),
        "price": format!("{}", price.unwrap_or("".to_string())),
        "size": format!("{}", size)
    });

    let resp = post_with_params(path, query);
    serde_json::from_str(&resp.into_string().unwrap()).unwrap()
}

/// ## change_losscut_price
/// 建玉のロスカットレート変更
///
/// ### Params
///  - position_id
///  - losscut_price
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

fn get_with_params(path: &'static str, json: Value) -> ureq::Response {
    let timestamp = timestamp();
    let text = format!("{}GET{}", timestamp, path);
    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, (&SECRET_KEY).as_bytes());
    let sign = hex::encode(hmac::sign(&signed_key, text.as_bytes()).as_ref());

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    ureq::get(&url)
        .set("API-KEY", &API_KEY)
        .set("API-TIMESTAMP", &format!("{}", timestamp))
        .set("API-SIGN", &sign)
        .send_json(json)
}

fn post_with_params(path: &'static str, json: Value) -> ureq::Response {
    let timestamp = timestamp();
    let text = format!("{}POST{}", timestamp, path);
    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, (&SECRET_KEY).as_bytes());
    let sign = hex::encode(hmac::sign(&signed_key, text.as_bytes()).as_ref());

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    ureq::post(&url)
        .set("content-type", "application/json")
        .set("API-KEY", &API_KEY)
        .set("API-TIMESTAMP", &format!("{}", timestamp))
        .set("API-SIGN", &sign)
        .send_json(json)
}

fn get_without_params(path: &'static str) -> ureq::Response {
    let timestamp = timestamp();
    let text = format!("{}GET{}", timestamp, path);
    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, (&SECRET_KEY).as_bytes());
    let sign = hex::encode(hmac::sign(&signed_key, text.as_bytes()).as_ref());

    let url = format!("{}{}", endpoint::PRIVATE_API, path);
    ureq::get(&url)
        .set("API-KEY", &API_KEY)
        .set("API-TIMESTAMP", &format!("{}", timestamp))
        .set("API-SIGN", &sign)
        .call()
}
