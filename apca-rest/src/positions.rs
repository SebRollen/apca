use crate::{AssetClass, Exchange, Identifier};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use vila::{Method, Request, RequestData};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
/// Denotes whether a position is long or short
pub enum Side {
    /// Long position, e.g. shares > 0
    Long,
    /// Long position, e.g. shares < 0
    Short,
}

impl Default for Side {
    fn default() -> Side {
        Side::Long
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Position object
pub struct Position {
    /// Asset ID
    pub asset_id: Uuid,
    /// Symbol name of the asset
    pub symbol: String,
    /// Exchange name of the asset
    pub exchange: Exchange,
    /// Asset class name
    pub asset_class: AssetClass,
    /// Average entry price of the position
    pub avg_entry_price: Decimal,
    #[serde(
        deserialize_with = "crate::utils::from_str",
        serialize_with = "crate::utils::to_string"
    )]
    /// The number of shares
    pub qty: i32,
    // TODO: Alpaca docs only list "long" here
    /// The side of the position
    pub side: Side,
    /// Total dollar amount of the position
    pub market_value: Decimal,
    /// Total cost basis in dollars
    pub cost_basis: Decimal,
    /// Unrealized profit/loss in dollars
    pub unrealized_pl: Decimal,
    /// Unrealized profit/loss percent (by a factor of 1)
    pub unrealized_plpc: Decimal,
    /// Unrealized profit/loss in dollars for the day
    pub unrealized_intraday_pl: Decimal,
    /// Unrealized profit/loss percent (by a factor of 1)
    pub unrealized_intraday_plpc: Decimal,
    /// Current asset price per share
    pub current_price: Decimal,
    /// Last day’s asset price per share based on the closing value of the last trading day
    pub lastday_price: Decimal,
    /// Percent change from last day price (by a factor of 1)
    pub change_today: Decimal,
}

#[derive(Clone, Debug)]
/// Retrieves a list of the account’s open positions.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     positions::{GetPositions, Position},
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let positions: Vec<Position> = client.send(&GetPositions).await?;
///     Ok(())
/// }
/// ```
pub struct GetPositions;

impl Request for GetPositions {
    type Data = ();
    type Response = Vec<Position>;

    fn endpoint(&self) -> Cow<str> {
        "/v2/positions".into()
    }
}

#[derive(Clone, Debug)]
/// Retrieves the account’s open position for the given symbol, or asset_id.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     positions::{GetPosition, Position},
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let position: Position = client.send(&GetPosition::new("AAPL")).await?;
///     Ok(())
/// }
pub struct GetPosition {
    identifier: Identifier,
}

impl GetPosition {
    /// Create a new request
    pub fn new<T: Into<Identifier>>(identifier: T) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }
}

impl Request for GetPosition {
    type Data = ();
    type Response = Position;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/positions/{}", self.identifier).into()
    }
}

#[derive(Clone, Debug, Default, Serialize)]
/// Closes (liquidates) all of the account’s open long and short positions. A response will be
/// provided for each order that is attempted to be cancelled. If an order is no longer cancelable,
/// the server will respond with status 500 and reject the request.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     positions::{CloseAllPositions, Position},
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let positions: Vec<Position> = client
///         .send(&CloseAllPositions::new().cancel_orders(true))
///         .await?;
///     Ok(())
/// }
pub struct CloseAllPositions {
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_orders: Option<bool>,
}

impl CloseAllPositions {
    /// Create a new request
    pub fn new() -> Self {
        Self::default()
    }

    /// If true is specified, cancel all open orders before liquidating all positions.
    pub fn cancel_orders(mut self, cancel_orders: bool) -> Self {
        self.cancel_orders = Some(cancel_orders);
        self
    }
}

impl Request for CloseAllPositions {
    type Data = Self;
    type Response = Vec<Position>;
    const METHOD: Method = Method::DELETE;

    fn endpoint(&self) -> Cow<str> {
        "/v2/positions".into()
    }

    fn data(&self) -> RequestData<&Self::Data> {
        RequestData::Query(&self)
    }
}

#[derive(Clone, Debug)]
/// Closes (liquidates) the account’s open position for the given symbol, or asset_id. Works for both long and short positions.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     positions::{ClosePosition, Position},
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let position: Position = client.send(&ClosePosition::new("AAPL")).await?;
///     Ok(())
/// }
pub struct ClosePosition {
    identifier: Identifier,
}

impl ClosePosition {
    /// Create a new request
    pub fn new<T: Into<Identifier>>(identifier: T) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }
}

impl Request for ClosePosition {
    type Data = ();
    type Response = Position;
    const METHOD: Method = Method::DELETE;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/positions/{}", self.identifier).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_with_url;
    use mockito::mock;

    #[tokio::test]
    async fn get_positions() {
        let positions = format!("[{}]", POSITION);
        let _m = mock("GET", "/v2/positions")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(positions)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&GetPositions).await.unwrap();
    }

    #[tokio::test]
    async fn get_position_by_ticker() {
        let _m = mock("GET", "/v2/positions/AAPL")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(POSITION)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&GetPosition::new("AAPL")).await.unwrap();
    }

    #[tokio::test]
    async fn get_position_by_id() {
        let _m = mock("GET", "/v2/positions/092efc51-b66b-4355-8132-d9c3796b9a76")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(POSITION)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client
            .send(&GetPosition::new(
                Uuid::parse_str("092efc51-b66b-4355-8132-d9c3796b9a76").unwrap(),
            ))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn close_position_by_ticker() {
        let _m = mock("DELETE", "/v2/positions/AAPL")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(POSITION)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&ClosePosition::new("AAPL")).await.unwrap();
    }

    #[tokio::test]
    async fn close_position_by_id() {
        let _m = mock(
            "DELETE",
            "/v2/positions/092efc51-b66b-4355-8132-d9c3796b9a76",
        )
        .match_header("apca-api-key-id", "APCA_API_KEY_ID")
        .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
        .with_body(POSITION)
        .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client
            .send(&ClosePosition::new(
                Uuid::parse_str("092efc51-b66b-4355-8132-d9c3796b9a76").unwrap(),
            ))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn close_all_positions() {
        let positions = format!("[{}]", POSITION);
        let _m = mock("DELETE", "/v2/positions")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query("cancel_orders=true")
            .with_status(207)
            .with_body(positions)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client
            .send(&CloseAllPositions::new().cancel_orders(true))
            .await
            .unwrap();
    }

    const POSITION: &'static str = r#"{
	  "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
	  "symbol": "AAPL",
	  "exchange": "NASDAQ",
	  "asset_class": "us_equity",
	  "avg_entry_price": "100.0",
	  "qty": "5",
	  "side": "long",
	  "market_value": "600.0",
	  "cost_basis": "500.0",
	  "unrealized_pl": "100.0",
	  "unrealized_plpc": "0.20",
	  "unrealized_intraday_pl": "10.0",
	  "unrealized_intraday_plpc": "0.0084",
	  "current_price": "120.0",
	  "lastday_price": "119.0",
	  "change_today": "0.0084"
	}"#;
}
