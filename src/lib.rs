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
    XLM,
    XTZ,
    DOT,
    ATOM,
    DAI,
    FCR,
    ADA,
    LINK,
    DOGE,
    SOL,
    ASTR,
    NAC,
    SUI,
    BTC_JPY,
    ETH_JPY,
    BCH_JPY,
    LTC_JPY,
    XRP_JPY,
    DOT_JPY,
    ATOM_JPY,
    ADA_JPY,
    LINK_JPY,
    DOGE_JPY,
    SOL_JPY,
    SUI_JPY,
}

impl fmt::Display for self::Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::BTC => write!(f, "BTC"),
            Symbol::ETH => write!(f, "ETH"),
            Symbol::BCH => write!(f, "BCH"),
            Symbol::LTC => write!(f, "LTC"),
            Symbol::XRP => write!(f, "XRP"),
            Symbol::XLM => write!(f, "XLM"),
            Symbol::XTZ => write!(f, "XTZ"),
            Symbol::DOT => write!(f, "DOT"),
            Symbol::ATOM => write!(f, "ATOM"),
            Symbol::DAI => write!(f, "DAI"),
            Symbol::FCR => write!(f, "FCR"),
            Symbol::ADA => write!(f, "ADA"),
            Symbol::LINK => write!(f, "LINK"),
            Symbol::DOGE => write!(f, "DOGE"),
            Symbol::SOL => write!(f, "SOL"),
            Symbol::ASTR => write!(f, "ASTR"),
            Symbol::NAC => write!(f, "NAC"),
            Symbol::SUI => write!(f, "SUI"),
            Symbol::BTC_JPY => write!(f, "BTC_JPY"),
            Symbol::ETH_JPY => write!(f, "ETH_JPY"),
            Symbol::BCH_JPY => write!(f, "BCH_JPY"),
            Symbol::LTC_JPY => write!(f, "LTC_JPY"),
            Symbol::XRP_JPY => write!(f, "XRP_JPY"),
            Symbol::DOT_JPY => write!(f, "DOT_JPY"),
            Symbol::ATOM_JPY => write!(f, "ATOM_JPY"),
            Symbol::ADA_JPY => write!(f, "ADA_JPY"),
            Symbol::LINK_JPY => write!(f, "LINK_JPY"),
            Symbol::DOGE_JPY => write!(f, "DOGE_JPY"),
            Symbol::SOL_JPY => write!(f, "SOL_JPY"),
            Symbol::SUI_JPY => write!(f, "SUI_JPY"),
        }
    }
}

#[derive(Debug)]
pub enum SymbolError {
    SymbolParseError,
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
            "ETH" | "eth" => Ok(Symbol::ETH),
            "BCH" | "bch" => Ok(Symbol::BCH),
            "LTC" | "ltc" => Ok(Symbol::LTC),
            "XRP" | "xrp" => Ok(Symbol::XRP),
            "XLM" | "xlm" => Ok(Symbol::XLM),
            "XTZ" | "xtz" => Ok(Symbol::XTZ),
            "DOT" | "dot" => Ok(Symbol::DOT),
            "ATOM" | "atom" => Ok(Symbol::ATOM),
            "DAI" | "dai" => Ok(Symbol::DAI),
            "FCR" | "fcr" => Ok(Symbol::FCR),
            "ADA" | "ada" => Ok(Symbol::ADA),
            "LINK" | "link" => Ok(Symbol::LINK),
            "DOGE" | "doge" => Ok(Symbol::DOGE),
            "SOL" | "sol" => Ok(Symbol::SOL),
            "ASTR" | "astr" => Ok(Symbol::ASTR),
            "NAC" | "nac" => Ok(Symbol::NAC),
            "SUI" | "sui" => Ok(Symbol::SUI),
            "BTC_JPY" | "btc_jpy" => Ok(Symbol::BTC_JPY),
            "ETH_JPY" | "eth_jpy" => Ok(Symbol::ETH_JPY),
            "BCH_JPY" | "bch_jpy" => Ok(Symbol::BCH_JPY),
            "LTC_JPY" | "ltc_jpy" => Ok(Symbol::LTC_JPY),
            "XRP_JPY" | "xrp_jpy" => Ok(Symbol::XRP_JPY),
            "DOT_JPY" | "dot_jpy" => Ok(Symbol::DOT_JPY),
            "ATOM_JPY" | "atom_jpy" => Ok(Symbol::ATOM_JPY),
            "ADA_JPY" | "ada_jpy" => Ok(Symbol::ADA_JPY),
            "LINK_JPY" | "link_jpy" => Ok(Symbol::LINK_JPY),
            "DOGE_JPY" | "doge_jpy" => Ok(Symbol::DOGE_JPY),
            "SOL_JPY" | "sol_jpy" => Ok(Symbol::SOL_JPY),
            "SUI_JPY" | "sui_jpy" => Ok(Symbol::SUI_JPY),
            _ => Err(SymbolError::SymbolParseError),
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
    DOT_JPY,
    ATOM_JPY,
    ADA_JPY,
    LINK_JPY,
    DOGE_JPY,
    SOL_JPY,
    SUI_JPY,
}

impl fmt::Display for self::LeverageSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LeverageSymbol::BTC_JPY => write!(f, "BTC_JPY"),
            LeverageSymbol::ETH_JPY => write!(f, "ETH_JPY"),
            LeverageSymbol::BCH_JPY => write!(f, "BCH_JPY"),
            LeverageSymbol::LTC_JPY => write!(f, "LTC_JPY"),
            LeverageSymbol::XRP_JPY => write!(f, "XRP_JPY"),
            LeverageSymbol::DOT_JPY => write!(f, "DOT_JPY"),
            LeverageSymbol::ATOM_JPY => write!(f, "ATOM_JPY"),
            LeverageSymbol::ADA_JPY => write!(f, "ADA_JPY"),
            LeverageSymbol::LINK_JPY => write!(f, "LINK_JPY"),
            LeverageSymbol::DOGE_JPY => write!(f, "DOGE_JPY"),
            LeverageSymbol::SOL_JPY => write!(f, "SOL_JPY"),
            LeverageSymbol::SUI_JPY => write!(f, "SUI_JPY"),
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
            "DOT_JPY" | "dot_jpy" => Ok(LeverageSymbol::DOT_JPY),
            "ATOM_JPY" | "atom_jpy" => Ok(LeverageSymbol::ATOM_JPY),
            "ADA_JPY" | "ada_jpy" => Ok(LeverageSymbol::ADA_JPY),
            "LINK_JPY" | "link_jpy" => Ok(LeverageSymbol::LINK_JPY),
            "DOGE_JPY" | "doge_jpy" => Ok(LeverageSymbol::DOGE_JPY),
            "SOL_JPY" | "sol_jpy" => Ok(LeverageSymbol::SOL_JPY),
            "SUI_JPY" | "sui_jpy" => Ok(LeverageSymbol::SUI_JPY),
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
    SideParseError,
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
            "SELL" | "sell" => Ok(Side::SELL),
            _ => Err(SideError::SideParseError),
        }
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
    pub responsetime: String,
}

/// ## ResponsePage
///
/// Response
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponsePage<T> {
    pub status: usize,
    pub data: DataPage<T>,
    pub responsetime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPage<T> {
    pub pagination: Pagenation,
    pub list: Vec<T>,
}

/// ## ResponseList
///
/// Response
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseList<T> {
    pub status: usize,
    pub data: List<T>,
    pub responsetime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List<T> {
    pub list: Vec<T>,
}
