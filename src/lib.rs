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
use std::fmt;

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

impl fmt::Display for self::Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Debug)]
pub enum SymbolError {
    SymbolParseError
}

impl fmt::Display for self::SymbolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SymbolError::SymbolParseError => write!(f, "Can't parse symbol"),
        }
    }
}

impl std::str::FromStr for self::Symbol {
    type Err = self::SymbolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BTC" | "btc" => Ok(Symbol::BTC),
            "ETH" | "etc" => Ok(Symbol::ETH),
            "BCH" | "bch" => Ok(Symbol::BCH),
            "LTC" | "ltc" => Ok(Symbol::LTC),
            "XRP" | "xrp" => Ok(Symbol::XRP),
            "BTC_JPY" | "btc_jpy" => Ok(Symbol::BTC_JPY),
            "ETH_JPY" | "eth_jpy" => Ok(Symbol::ETH_JPY),
            "BCH_JPY" | "bch_jpy" => Ok(Symbol::BCH_JPY),
            "LTC_JPY" | "ltc_jpy" => Ok(Symbol::LTC_JPY),
            "XRP_JPY" | "xrp_jpy" => Ok(Symbol::XRP_JPY),
            _ => Err(SymbolError::SymbolParseError),
        }
    }
}

impl Symbol {
    pub fn from_str(s: &str) -> Result<Symbol, SymbolError> {
        s.parse() 
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

impl fmt::Display for self::LeverageSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LeverageSymbol::BTC_JPY => write!(f, "BTC_JPY"),
            LeverageSymbol::ETH_JPY => write!(f, "ETH_JPY"),
            LeverageSymbol::BCH_JPY => write!(f, "BCH_JPY"),
            LeverageSymbol::LTC_JPY => write!(f, "LTC_JPY"),
            LeverageSymbol::XRP_JPY => write!(f, "XRP_JPY"),
        }
    }
}

impl std::str::FromStr for self::LeverageSymbol {
    type Err = self::SymbolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BTC_JPY" | "btc_jpy" => Ok(LeverageSymbol::BTC_JPY),
            "ETH_JPY" | "eth_jpy" => Ok(LeverageSymbol::ETH_JPY),
            "BCH_JPY" | "bch_jpy" => Ok(LeverageSymbol::BCH_JPY),
            "LTC_JPY" | "ltc_jpy" => Ok(LeverageSymbol::LTC_JPY),
            "XRP_JPY" | "xrp_jpy" => Ok(LeverageSymbol::XRP_JPY),
            _ => Err(SymbolError::SymbolParseError),
        }
    }
}

/// ## Side
/// 売買
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
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
pub enum SideError {
    SideParseError
}

impl fmt::Display for self::SideError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SideError::SideParseError => write!(f, "SideParseError"),
        }
    }
}

impl std::str::FromStr for self::Side {
    type Err = self::SideError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUY" | "buy" => Ok(Side::BUY),
            "SELL"|"sell" => Ok(Side::SELL),
            _ => Err(SideError::SideParseError),
        }
    }
}

impl Side {
    pub fn from_str(s: &str) -> Result<Side, SideError> {
        s.parse()
    }
}

/// ## Pagenation
/// ページ形式で返ってくるレスポンス用の構造体
///
///  - currentPage
///  - count
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagenation {
    pub current_page: usize,
    pub count: usize,
}

/// ## Response
///
/// Response
#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub status: usize,
    pub data: T,
    pub responsetime: String
}

/// ## ResponsePage
///
/// Response
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsePage<T> {
    pub status: usize,
    pub data: DataPage<T>,
    pub responsetime: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPage<T> {
    pub pagination: Pagenation,
    pub list: Vec<T>
}

/// ## ResponseList
///
/// Response
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseList<T> {
    pub status: usize,
    pub data: List<T>,
    pub responsetime: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List<T> {
    pub list: Vec<T>
}

