use serde::{Deserialize, Serialize};
use std::convert::From;
use uuid::Uuid;

#[derive(Serialize, Clone, Debug)]
/// Sort settings
pub enum Sort {
    #[serde(rename = "asc")]
    /// Ascending sort
    Ascending,
    #[serde(rename = "desc")]
    /// Descending sort
    Descending,
}

impl Default for Sort {
    fn default() -> Self {
        Sort::Descending
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
/// Types of asset classes supported
pub enum AssetClass {
    /// US equities
    UsEquity,
    /// Crypto
    Crypto,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
/// Listing of exchanges
pub enum Exchange {
    /// American Stock Exchange
    Amex,
    /// Arca
    Arca,
    /// BATS
    Bats,
    /// New York Stock Exchange
    Nyse,
    /// NASDAQ
    Nasdaq,
    /// NYSE Arca
    NyseArca,
    /// Over the counter
    Otc,
    /// Crypto only
    ErisX,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Identifier that can be used to find an asset at Alpaca
pub enum Identifier {
    /// Symbol, optionally with exchange and asset class
    Symbol(String, Option<(String, Option<String>)>),
    /// Unique asset id
    AssetId(Uuid),
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identifier::AssetId(id) => write!(f, "{}", id),
            Identifier::Symbol(symbol, None) => write!(f, "{}", symbol),
            Identifier::Symbol(symbol, Some((exchange, None))) => {
                write!(f, "{}:{}", symbol, exchange)
            }
            Identifier::Symbol(symbol, Some((exchange, Some(asset_class)))) => {
                write!(f, "{}:{}:{}", symbol, exchange, asset_class)
            }
        }
    }
}

impl From<Uuid> for Identifier {
    fn from(u: Uuid) -> Identifier {
        Identifier::AssetId(u)
    }
}

impl<'a> From<&'a str> for Identifier {
    fn from(s: &'a str) -> Identifier {
        if let Ok(u) = Uuid::parse_str(s) {
            Identifier::AssetId(u)
        } else if let Some((symbol, rest)) = s.split_once(':') {
            if let Some((exchange, asset_class)) = rest.split_once(':') {
                Identifier::Symbol(
                    symbol.to_string(),
                    Some((exchange.to_string(), Some(asset_class.to_string()))),
                )
            } else {
                Identifier::Symbol(symbol.to_string(), Some((rest.to_string(), None)))
            }
        } else {
            Identifier::Symbol(s.to_string(), None)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn formatting() {
        assert_eq!(
            Identifier::Symbol("AAPL".to_string(), None).to_string(),
            "AAPL".to_string()
        );
        assert_eq!(
            Identifier::Symbol("AAPL".to_string(), Some(("NYSE".to_string(), None))).to_string(),
            "AAPL:NYSE".to_string()
        );
        assert_eq!(
            Identifier::Symbol(
                "AAPL".to_string(),
                Some(("NYSE".to_string(), Some("us_equity".to_string())))
            )
            .to_string(),
            "AAPL:NYSE:us_equity".to_string()
        );
    }

    #[test]
    fn construction() {
        let i: Identifier = "AAPL".into();
        assert_eq!(i, Identifier::Symbol("AAPL".to_string(), None));
        let i: Identifier = "AAPL:NYSE".into();
        assert_eq!(
            i,
            Identifier::Symbol("AAPL".to_string(), Some(("NYSE".to_string(), None)))
        );
        let i: Identifier = "AAPL:NYSE:us_equity".into();
        assert_eq!(
            i,
            Identifier::Symbol(
                "AAPL".to_string(),
                Some(("NYSE".to_string(), Some("us_equity".to_string())))
            )
        );
        let i: Identifier = "00000000-0000-0000-0000-000000000000".into();
        assert_eq!(i, Identifier::AssetId(Uuid::nil()))
    }
}
