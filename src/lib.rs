extern crate hex;
#[macro_use]
extern crate lazy_static;
extern crate ring;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate ureq;

pub mod endpoint;
pub mod private;
pub mod public;

use serde::{Deserialize, Serialize};

/// ## Symbol
/// validなsymbol
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Symbol {
    BTC,
    ETH,
    BCH,
    LTC,
    XRP,
    BTC_JPY,
    ETH_JPY,
    BCH_JPY,
    LTC_JPY,
    XRP_JPY,
}

impl std::fmt::Display for self::Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Symbol::BTC => write!(f, "BTC"),
            Symbol::ETH => write!(f, "ETH"),
            Symbol::BCH => write!(f, "BCH"),
            Symbol::LTC => write!(f, "LTC"),
            Symbol::XRP => write!(f, "XRP"),
            Symbol::BTC_JPY => write!(f, "BTC_JPY"),
            Symbol::ETH_JPY => write!(f, "ETH_JPY"),
            Symbol::BCH_JPY => write!(f, "BCH_JPY"),
            Symbol::LTC_JPY => write!(f, "LTC_JPY"),
            Symbol::XRP_JPY => write!(f, "XRP_JPY"),
        }
    }
}

/// ## LevarageSymbol
/// levarageのvalidなsymbol
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum LeverageSymbol {
    BTC_JPY,
    ETH_JPY,
    BCH_JPY,
    LTC_JPY,
    XRP_JPY,
}

impl std::fmt::Display for self::LeverageSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LeverageSymbol::BTC_JPY => write!(f, "BTC_JPY"),
            LeverageSymbol::ETH_JPY => write!(f, "ETH_JPY"),
            LeverageSymbol::BCH_JPY => write!(f, "BCH_JPY"),
            LeverageSymbol::LTC_JPY => write!(f, "LTC_JPY"),
            LeverageSymbol::XRP_JPY => write!(f, "XRP_JPY"),
        }
    }
}
/// ## Pagenation
/// ページ形式で返ってくるレスポンス用の構造体
///
/// ### pagenation
/// #### currentPage
/// #### count
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagenation {
    pub current_page: String,
    pub count: String,
}
