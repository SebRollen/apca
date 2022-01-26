use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;
use std::ops::Neg;
use uuid::Uuid;
use vila::{Method, Request, RequestData};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrderType {
    Market,
    Limit {
        limit_price: Decimal,
    },
    Stop {
        stop_price: Decimal,
    },
    StopLimit {
        limit_price: Decimal,
        stop_price: Decimal,
    },
    TrailingStop {
        trail_price: Option<Decimal>,
        trail_percent: Option<Decimal>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TimeInForce {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "gtc")]
    GoodTilCancelled,
    #[serde(rename = "opg")]
    Open,
    #[serde(rename = "cls")]
    Close,
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    #[serde(rename = "fok")]
    FillOrKill,
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::Day
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TakeProfitSpec {
    pub limit_price: Decimal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StopLossSpec {
    pub stop_price: Decimal,
    pub limit_price: Decimal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OrderClass {
    Simple,
    Bracket {
        take_profit: TakeProfitSpec,
        stop_loss: StopLossSpec,
    },
    OneCancelsOther {
        take_ptofit: TakeProfitSpec,
        stop_loss: StopLossSpec,
    },
    OneTriggersOther {
        stop_loss: StopLossSpec,
    },
}

impl Default for OrderClass {
    fn default() -> Self {
        OrderClass::Simple
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Accepted,
    AcceptedForBidding,
    Calculated,
    Canceled,
    DoneForDay,
    Expired,
    Filled,
    New,
    PartiallyFilled,
    PendingCancel,
    PendingNew,
    PendingReplace,
    Rejected,
    Replaced,
    Stopped,
    Suspended,
}

impl Default for OrderStatus {
    fn default() -> OrderStatus {
        OrderStatus::New
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

impl Default for Side {
    fn default() -> Side {
        Side::Buy
    }
}

impl Neg for Side {
    type Output = Side;

    fn neg(self) -> Self::Output {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Order {
    pub id: Uuid,
    pub client_order_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub filled_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub replaced_at: Option<DateTime<Utc>>,
    pub replaced_by: Option<Uuid>,
    pub replaces: Option<Uuid>,
    pub asset_id: Uuid,
    pub symbol: String,
    pub asset_class: String,
    #[serde(
        deserialize_with = "crate::utils::from_str",
        serialize_with = "crate::utils::to_string"
    )]
    pub qty: usize,
    #[serde(
        deserialize_with = "crate::utils::from_str",
        serialize_with = "crate::utils::to_string"
    )]
    pub filled_qty: usize,
    pub filled_avg_price: Option<Decimal>,
    #[serde(flatten, rename(serialize = "type"))]
    pub order_type: OrderType,
    pub side: Side,
    pub time_in_force: TimeInForce,
    pub status: OrderStatus,
    pub extended_hours: bool,
    pub legs: Option<Vec<Order>>,
    pub hwm: Option<Decimal>,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Market
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OrderIntent {
    pub symbol: String,
    #[serde(
        deserialize_with = "crate::utils::from_str",
        serialize_with = "crate::utils::to_string"
    )]
    pub qty: usize,
    pub side: Side,
    #[serde(flatten, rename(serialize = "type"))]
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub extended_hours: bool,
    pub client_order_id: Option<String>,
    pub order_class: OrderClass,
}

impl OrderIntent {
    pub fn new(symbol: &str) -> Self {
        OrderIntent {
            symbol: symbol.to_string(),
            qty: 1,
            side: Side::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GoodTilCancelled,
            extended_hours: false,
            client_order_id: None,
            order_class: OrderClass::Simple,
        }
    }

    pub fn qty(mut self, qty: usize) -> Self {
        self.qty = qty;
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = order_type;
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = time_in_force;
        self
    }

    pub fn extended_hours(mut self, extended_hours: bool) -> Self {
        self.extended_hours = extended_hours;
        self
    }

    pub fn client_order_id(mut self, client_order_id: String) -> Self {
        self.client_order_id = Some(client_order_id);
        self
    }

    pub fn order_class(mut self, order_class: OrderClass) -> Self {
        self.order_class = order_class;
        self
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum QueryOrderStatus {
    Open,
    Closed,
    All,
}
impl Default for QueryOrderStatus {
    fn default() -> Self {
        QueryOrderStatus::Open
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum Sort {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}
impl Default for Sort {
    fn default() -> Self {
        Sort::Descending
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct GetOrders {
    pub status: QueryOrderStatus,
    pub limit: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    pub direction: Sort,
    pub nested: bool,
    pub symbols: Option<Vec<String>>,
}

impl GetOrders {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for GetOrders {
    fn default() -> Self {
        Self {
            status: Default::default(),
            limit: 50,
            after: None,
            until: None,
            direction: Default::default(),
            nested: false,
            symbols: None,
        }
    }
}

impl Request for GetOrders {
    type Data = Self;
    type Response = Vec<Order>;

    fn endpoint(&self) -> Cow<str> {
        "/v2/orders".into()
    }

    fn data(&self) -> RequestData<&Self> {
        RequestData::Query(self)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct GetOrder<'a> {
    #[serde(skip)]
    pub order_id: &'a str,
    pub nested: bool,
}

impl<'a> GetOrder<'a> {
    pub fn new(order_id: &'a str) -> Self {
        Self {
            order_id,
            nested: false,
        }
    }
}
impl Request for GetOrder<'_> {
    type Data = Self;
    type Response = Order;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/orders/{}", self.order_id).into()
    }

    fn data(&self) -> RequestData<&Self> {
        RequestData::Query(self)
    }
}

#[derive(Clone, Debug)]
pub struct SubmitOrder(pub OrderIntent);
impl Request for SubmitOrder {
    type Data = OrderIntent;
    type Response = Order;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<str> {
        "/v2/orders".into()
    }

    fn data(&self) -> RequestData<&OrderIntent> {
        RequestData::Json(&self.0)
    }
}

#[derive(Clone, Debug)]
pub struct ReplaceOrder<'a>(pub &'a str, pub OrderIntent);
impl Request for ReplaceOrder<'_> {
    type Data = OrderIntent;
    type Response = Order;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/orders/{}", self.0).into()
    }

    fn data(&self) -> RequestData<&OrderIntent> {
        RequestData::Json(&self.1)
    }
}

#[derive(Clone, Debug)]
pub struct EmptyResponse;
impl<'de> Deserialize<'de> for EmptyResponse {
    fn deserialize<D>(_deserializer: D) -> Result<EmptyResponse, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(EmptyResponse {})
    }
}

#[derive(Clone, Debug)]
pub struct CancelOrder(pub Uuid);

impl Request for CancelOrder {
    type Data = ();
    type Response = EmptyResponse;
    const METHOD: Method = Method::DELETE;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/orders/{}", self.0).into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancellationAttempt {
    pub id: Uuid,
    pub status: usize,
    pub body: Order,
}

#[derive(Clone, Debug)]
pub struct CancelAllOrders();

impl Request for CancelAllOrders {
    type Data = ();
    type Response = Vec<Order>;
    const METHOD: Method = Method::DELETE;

    fn endpoint(&self) -> Cow<str> {
        "/v2/orders".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client_with_url;
    use mockito::{mock, Matcher};

    #[test]
    fn test_defaults() {
        match OrderType::default() {
            OrderType::Market => {} // Happy case
            _ => panic!(),
        };
        match TimeInForce::default() {
            TimeInForce::Day => {} // Happy case
            _ => panic!(),
        };
        match OrderClass::default() {
            OrderClass::Simple => {} // Happy case
            _ => panic!(),
        };
        match Side::default() {
            Side::Buy => {} // Happy case
            _ => panic!(),
        };
    }

    #[test]
    fn serde() {
        let json = r#"{
            "symbol":"AAPL",
            "qty":"123",
            "side":"buy",
            "type":"limit",
            "limit_price":"100",
            "time_in_force":"gtc",
            "extended_hours":false,
            "client_order_id":"TEST",
            "order_class":{
                "bracket":{
                    "take_profit":{
                        "limit_price":301.0
                    },
                    "stop_loss":{
                        "stop_price":299.0,
                        "limit_price":298.5
                    }
                }
            }
        }"#;
        let deserialized: OrderIntent = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
    }

    #[tokio::test]
    async fn test_get_order() {
        let _m = mock("GET", "/v2/orders/904837e3-3b76-47ec-b432-046db621571b")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::UrlEncoded("nested".into(), "false".into()))
            .with_body(
                r#"{
			        "id": "904837e3-3b76-47ec-b432-046db621571b",
			        "client_order_id": "904837e3-3b76-47ec-b432-046db621571b",
			        "created_at": "2018-10-05T05:48:59Z",
			        "updated_at": "2018-10-05T05:48:59Z",
			        "submitted_at": "2018-10-05T05:48:59Z",
			        "filled_at": "2018-10-05T05:48:59Z",
			        "expired_at": "2018-10-05T05:48:59Z",
			        "canceled_at": "2018-10-05T05:48:59Z",
			        "failed_at": "2018-10-05T05:48:59Z",
			        "replaced_at": "2018-10-05T05:48:59Z",
			        "replaced_by": "904837e3-3b76-47ec-b432-046db621571b",
			        "replaces": null,
			        "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
			        "symbol": "AAPL",
			        "asset_class": "us_equity",
			        "qty": "15",
			        "filled_qty": "0",
			        "type": "market",
			        "side": "buy",
			        "time_in_force": "day",
			        "limit_price": "107.00",
			        "stop_price": "106.00",
			        "filled_avg_price": "106.00",
			        "status": "accepted",
			        "extended_hours": false,
			        "legs": null,
                    "trail_price": "1.05",
                    "trail_percent": null,
                    "hwm": "108.05"
			    }"#,
            )
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client
            .send(&GetOrder::new("904837e3-3b76-47ec-b432-046db621571b"))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_orders() {
        let _m = mock("GET", "/v2/orders")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("status".into(), "open".into()),
                Matcher::UrlEncoded("limit".into(), "50".into()),
                Matcher::UrlEncoded("direction".into(), "desc".into()),
                Matcher::UrlEncoded("nested".into(), "false".into()),
            ]))
            .with_body(
                r#"[{
			        "id": "904837e3-3b76-47ec-b432-046db621571b",
			        "client_order_id": "904837e3-3b76-47ec-b432-046db621571b",
			        "created_at": "2018-10-05T05:48:59Z",
			        "updated_at": "2018-10-05T05:48:59Z",
			        "submitted_at": "2018-10-05T05:48:59Z",
			        "filled_at": "2018-10-05T05:48:59Z",
			        "expired_at": "2018-10-05T05:48:59Z",
			        "canceled_at": "2018-10-05T05:48:59Z",
			        "failed_at": "2018-10-05T05:48:59Z",
			        "replaced_at": "2018-10-05T05:48:59Z",
			        "replaced_by": "904837e3-3b76-47ec-b432-046db621571b",
			        "replaces": null,
			        "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
			        "symbol": "AAPL",
			        "asset_class": "us_equity",
			        "qty": "15",
			        "filled_qty": "0",
			        "type": "market",
			        "side": "buy",
			        "time_in_force": "day",
			        "limit_price": "107.00",
			        "stop_price": "106.00",
			        "filled_avg_price": "106.00",
			        "status": "accepted",
			        "extended_hours": false,
			        "legs": null,
                    "trail_price": "1.05",
                    "trail_percent": null,
                    "hwm": "108.05"
                }]"#,
            )
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&GetOrders::new()).await.unwrap();
    }

    #[tokio::test]
    async fn missing_order() {
        let _m = mock("GET", "/v2/orders/904837e3-3b76-47ec-b432-046db621571b")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::UrlEncoded("nested".into(), "false".into()))
            .with_status(404)
            .create();

        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let res = client
            .send(&GetOrder::new("904837e3-3b76-47ec-b432-046db621571b"))
            .await;

        assert!(res.is_err())
    }

    #[tokio::test]
    async fn test_cancel_order() {
        let _m = mock("DELETE", "/v2/orders/904837e3-3b76-47ec-b432-046db621571b")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_status(204)
            .with_body(
                r#"{
                    "id": "904837e3-3b76-47ec-b432-046db621571b",
		    "client_order_id": "904837e3-3b76-47ec-b432-046db621571b",
	            "created_at": "2018-10-05T05:48:59Z",
		    "updated_at": "2018-10-05T05:48:59Z",
		    "submitted_at": "2018-10-05T05:48:59Z",
		    "filled_at": "2018-10-05T05:48:59Z",
		    "expired_at": "2018-10-05T05:48:59Z",
		    "canceled_at": "2018-10-05T05:48:59Z",
		    "failed_at": "2018-10-05T05:48:59Z",
		    "replaced_at": "2018-10-05T05:48:59Z",
		    "replaced_by": "904837e3-3b76-47ec-b432-046db621571b",
		    "replaces": null,
		    "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
		    "symbol": "AAPL",
		    "asset_class": "us_equity",
		    "qty": "15",
		    "filled_qty": "0",
		    "type": "market",
		    "side": "buy",
		    "time_in_force": "day",
		    "limit_price": "107.00",
		    "stop_price": "106.00",
		    "filled_avg_price": "106.00",
		    "status": "accepted",
		    "extended_hours": false,
		    "legs": null,
                    "trail_price": "1.05",
                    "trail_percent": null,
                    "hwm": "108.05"
		}"#,
            )
            .create();

        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client
            .send(&CancelOrder(
                Uuid::parse_str("904837e3-3b76-47ec-b432-046db621571b").unwrap(),
            ))
            .await
            .unwrap();
    }
}
