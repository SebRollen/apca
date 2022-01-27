use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use vila::{Method, Request, RequestData};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Side {
    Long,
    Short,
}

impl Default for Side {
    fn default() -> Side {
        Side::Long
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Position {
    pub asset_id: Uuid,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: String,
    pub avg_entry_price: Decimal,
    #[serde(
        deserialize_with = "crate::utils::from_str",
        serialize_with = "crate::utils::to_string"
    )]
    pub qty: i32,
    pub side: Side,
    pub market_value: Decimal,
    pub cost_basis: Decimal,
    pub unrealized_pl: Decimal,
    pub unrealized_plpc: Decimal,
    pub unrealized_intraday_pl: Decimal,
    pub unrealized_intraday_plpc: Decimal,
    pub current_price: Decimal,
    pub lastday_price: Decimal,
    pub change_today: Decimal,
}

#[derive(Clone, Debug)]
pub struct GetPositions;

impl Request for GetPositions {
    type Data = ();
    type Response = Vec<Position>;

    fn endpoint(&self) -> Cow<str> {
        "/v2/positions".into()
    }
}

#[derive(Clone, Debug)]
pub struct GetPosition<'a>(pub &'a str);

impl Request for GetPosition<'_> {
    type Data = ();
    type Response = Position;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/positions/{}", self.0).into()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CloseAllPositions {
    cancel_orders: Option<bool>,
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
pub struct ClosePosition<'a>(pub &'a str);

impl Request for ClosePosition<'_> {
    type Data = ();
    type Response = Position;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/positions/{}", self.0).into()
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
    async fn get_position() {
        let _m = mock("GET", "/v2/positions/AAPL")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(POSITION)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&GetPosition("AAPL")).await.unwrap();
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
