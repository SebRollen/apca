use crate::{AssetClass, Sort};
use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Neg;
use uuid::Uuid;
use vila::{EmptyResponse, Method, Request, RequestData};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
/// Specifies the type of an order
pub enum OrderType {
    /// Market order. This order type will be matched immediately, but there's no guarantee of fill
    /// price.
    Market,
    /// Limit order. This order is guaranteed to be filled at a specific price or better, but may
    /// not be executed immediately (or at all).
    Limit {
        /// The maximum (minimum) price at which a buy (sell) order will execute.
        limit_price: Decimal,
    },
    /// Stop order. This order type is normally used to limit losses as it is only executed when an
    /// instrument reaches the stop price or worse.
    Stop {
        /// The price at which an order will stop out
        stop_price: Decimal,
    },
    /// Stop-limit order. This order type is a combination of a stop order and a limit order, and
    /// will execute when either the limit price or the stop price is reached.
    StopLimit {
        /// The maximum (minimum) price at which a buy (sell) order will execute.
        limit_price: Decimal,
        /// The price at which an order will stop out
        stop_price: Decimal,
    },
    /// A stop order that automatically updates the stop price based on a threshold compared to the
    /// current price. Can be specified as either a price offset or percent offset from the current
    /// price.
    TrailingStop {
        /// The price offset from the high-water-mark at which the stop will execute.
        trail_price: Option<Decimal>,
        /// The percent offset from the high-water-mark at which the stop will execute.
        trail_percent: Option<Decimal>,
    },
}

impl OrderType {
    /// Create a Market order-type
    pub fn market() -> OrderType {
        OrderType::Market
    }

    /// Create a Limit order-type
    pub fn limit(limit_price: Decimal) -> OrderType {
        OrderType::Limit { limit_price }
    }

    /// Create a Stop order-type
    pub fn stop(stop_price: Decimal) -> OrderType {
        OrderType::Stop { stop_price }
    }

    /// Create a Stop order-type
    pub fn stop_limit(stop_price: Decimal, limit_price: Decimal) -> OrderType {
        OrderType::StopLimit {
            stop_price,
            limit_price,
        }
    }

    /// Create a Trailing price order-type
    pub fn trail_price(trail_price: Decimal) -> OrderType {
        OrderType::TrailingStop {
            trail_price: Some(trail_price),
            trail_percent: None,
        }
    }

    /// Create a Trailing percent order-type
    pub fn trail_percent(trail_percent: Decimal) -> OrderType {
        OrderType::TrailingStop {
            trail_price: None,
            trail_percent: Some(trail_percent),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Specifies how long an order is valid.
pub enum TimeInForce {
    #[serde(rename = "day")]
    /// A day order is eligible for execution only on the day it is live. By default, the order is
    /// only valid during Regular Trading Hours (9:30am - 4:00pm ET). If unfilled after the closing
    /// auction, it is automatically canceled. If submitted after the close, it is queued and
    /// submitted the following trading day. However, if marked as eligible for extended hours, the
    /// order can also execute during supported extended hours.
    Day,
    #[serde(rename = "gtc")]
    /// The order is good until canceled. Non-marketable GTC limit orders are subject to price
    /// adjustments to offset corporate actions affecting the issue. We do not currently support Do
    /// Not Reduce(DNR) orders to opt out of such price adjustments.
    GoodTilCancelled,
    #[serde(rename = "opg")]
    /// Use this TIF with a market/limit order type to submit “market on open” (MOO) and “limit on
    /// open” (LOO) orders. This order is eligible to execute only in the market opening auction.
    /// Any unfilled orders after the open will be cancelled. OPG orders submitted after 9:28am but
    /// before 7:00pm ET will be rejected. OPG orders submitted after 7:00pm will be queued and
    /// routed to the following day’s opening auction. On open/on close orders are routed to the
    /// primary exchange. Such orders do not necessarily execute exactly at 9:30am / 4:00pm ET but
    /// execute per the exchange’s auction rules.
    Open,
    #[serde(rename = "cls")]
    /// Use this TIF with a market/limit order type to submit “market on close” (MOC) and “limit on
    /// close” (LOC) orders. This order is eligible to execute only in the market closing auction.
    /// Any unfilled orders after the close will be cancelled. CLS orders submitted after 3:50pm
    /// but before 7:00pm ET will be rejected. CLS orders submitted after 7:00pm will be queued and
    /// routed to the following day’s closing auction.
    Close,
    #[serde(rename = "ioc")]
    /// An Immediate Or Cancel (IOC) order requires all or part of the order to be executed
    /// immediately. Any unfilled portion of the order is canceled.
    /// Most market makers who receive IOC orders will attempt to fill the order on a principal
    /// basis only, and cancel any unfilled balance. On occasion, this can result in the entire
    /// order being cancelled if the market maker does not have any existing inventory of the
    /// security in question.
    ImmediateOrCancel,
    #[serde(rename = "fok")]
    /// A Fill or Kill (FOK) order is only executed if the entire order quantity can be filled,
    /// otherwise the order is canceled.
    FillOrKill,
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::Day
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Specification for the take-profit leg of a complex order.
pub struct TakeProfitSpec {
    /// The price at which the take-profit order will execute
    pub limit_price: Decimal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Specification for the stop-loss leg of a complex order.
pub struct StopLossSpec {
    /// The price at which the stop-loss order will execute
    pub stop_price: Decimal,
    /// If set, the order will be sent as a stop-limit - otherwise it will be sent as a stop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Specification for OTO orders
pub enum OtoSpec {
    /// OTO order with a take-profit leg
    TakeProfit(TakeProfitSpec),
    /// OTO order with a stop-loss leg
    StopLoss(StopLossSpec),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase", tag = "order_class")]
/// Class of order
pub enum OrderClass {
    /// Standard order
    Simple,
    /// A bracket order is a chain of three orders that can be used to manage your position entry
    /// and exit. It is a common use case of an OTOCO (One Triggers OCO {One Cancels Other}) order.
    ///
    /// The first order is used to enter a new long or short position, and once it is completely
    /// filled, two conditional exit orders are activated. One of the two closing orders is called
    /// a take-profit order, which is a limit order, and the other is called a stop-loss order,
    /// which is either a stop or stop-limit order. Importantly, only one of the two exit orders
    /// can be executed. Once one of the exit orders is filled, the other is canceled. Please note,
    /// however, that in extremely volatile and fast market conditions, both orders may fill before
    /// the cancellation occurs.
    Bracket {
        /// The specification for the take-profit leg of the order
        take_profit: TakeProfitSpec,
        /// The specification for the stop-loss leg of the order
        stop_loss: StopLossSpec,
    },
    /// OCO (One-Cancels-Other) is another type of advanced order type. This is a set of two orders
    /// with the same side (buy/buy or sell/sell) and currently only exit order is supported. In
    /// other words, this is the second part of the bracket orders where the entry order is already
    /// filled, and you can submit the take-profit and stop-loss in one order submission.
    OneCancelsOther {
        /// The specification for the take-profit leg of the order
        take_profit: TakeProfitSpec,
        /// The specification for the stop-loss leg of the order
        stop_loss: StopLossSpec,
    },
    /// OTO (One-Triggers-Other) is a variant of bracket order. It takes one of the take-profit or
    /// stop-loss order in addition to the entry order. For example, if you want to set only a
    /// stop-loss order attached to the position, without a take-profit, you may want to consider
    /// OTO orders.
    OneTriggersOther {
        /// The specification for the stop-loss leg of the order
        spec: OtoSpec,
    },
}

impl Default for OrderClass {
    fn default() -> Self {
        OrderClass::Simple
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
/// Specifies the current status of an order in its lifecycle
pub enum OrderStatus {
    /// The order has been received by Alpaca, but hasn’t yet been routed to the execution venue.
    /// This could be seen often out side of trading session hours.
    Accepted,
    /// The order has been received by exchanges, and is evaluated for pricing. This state only
    /// occurs on rare occasions.
    AcceptedForBidding,
    /// The order has been completed for the day (either filled or done for day), but remaining
    /// settlement calculations are still pending. This state only occurs on rare occasions.
    Calculated,
    /// The order has been canceled, and no further updates will occur for the order. This can be
    /// either due to a cancel request by the user, or the order has been canceled by the exchanges
    /// due to its time-in-force.
    Canceled,
    /// The order is done executing for the day, and will not receive further updates until the
    /// next trading day.
    DoneForDay,
    /// The order has expired, and no further updates will occur for the order.
    Expired,
    /// The order has been filled, and no further updates will occur for the order.
    Filled,
    /// The order has been received by Alpaca, and routed to exchanges for execution. This is the
    /// usual initial state of an order.
    New,
    /// The order has been partially filled.
    PartiallyFilled,
    /// The order is waiting to be canceled.
    PendingCancel,
    /// The order has been received by Alpaca, and routed to the exchanges, but has not yet been
    /// accepted for execution. This state only occurs on rare occasions.
    PendingNew,
    /// The order is waiting to be replaced by another order. The order will reject cancel request
    /// while in this state.
    PendingReplace,
    /// The order has been rejected, and no further updates will occur for the order. This state
    /// occurs on rare occasions and may occur based on various conditions decided by the
    /// exchanges.
    Rejected,
    /// The order was replaced by another order, or was updated due to a market event such as
    /// corporate action.
    Replaced,
    /// The order has been stopped, and a trade is guaranteed for the order, usually at a stated
    /// price or better, but has not yet occurred. This state only occurs on rare occasions.
    Stopped,
    /// The order has been suspended, and is not eligible for trading. This state only occurs on
    /// rare occasions.
    Suspended,
}

impl Default for OrderStatus {
    fn default() -> OrderStatus {
        OrderStatus::New
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
/// The side of an order
pub enum Side {
    /// Buy
    Buy,
    /// Sell
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
/// Order object
pub struct Order {
    /// Order ID
    pub id: Uuid,
    /// Client unique order id
    pub client_order_id: String,
    /// When order was created.
    pub created_at: DateTime<Utc>,
    /// When order was last updated.
    pub updated_at: Option<DateTime<Utc>>,
    /// When order was submitted to Alpaca.
    pub submitted_at: Option<DateTime<Utc>>,
    /// When order was filled.
    pub filled_at: Option<DateTime<Utc>>,
    /// When order expired.
    pub expired_at: Option<DateTime<Utc>>,
    /// When order was cancelled.
    pub canceled_at: Option<DateTime<Utc>>,
    /// When order failed.
    pub failed_at: Option<DateTime<Utc>>,
    /// When order was replaced.
    pub replaced_at: Option<DateTime<Utc>>,
    /// Order ID
    pub replaced_by: Option<Uuid>,
    /// The order ID that this order replaces
    pub replaces: Option<Uuid>,
    /// Asset ID
    pub asset_id: Uuid,
    /// Asset symbol
    pub symbol: String,
    /// Asset class
    pub asset_class: AssetClass,
    #[serde(
        deserialize_with = "crate::utils::from_str",
        serialize_with = "crate::utils::to_string"
    )]
    /// Ordered quantity
    pub qty: usize,
    #[serde(
        deserialize_with = "crate::utils::from_str",
        serialize_with = "crate::utils::to_string"
    )]
    /// Filled quantity
    pub filled_qty: usize,
    /// Filled average price
    pub filled_avg_price: Option<Decimal>,
    #[serde(flatten, rename(serialize = "type"))]
    /// Type of order
    pub order_type: OrderType,
    /// Whether order is buy or sell
    pub side: Side,
    /// The time-in-force of the order
    pub time_in_force: TimeInForce,
    /// The current status of the order
    pub status: OrderStatus,
    /// If true, eligible for execution outside regular trading hours.
    pub extended_hours: bool,
    /// When querying non-simple order_class orders in a nested style, an array of Order entities
    /// associated with this order.
    pub legs: Option<Vec<Order>>,
    /// The percent value away from the high water mark for trailing stop orders.
    pub trail_percent: Option<Decimal>,
    /// The dollar value away from the high water mark for trailing stop orders.
    pub trail_price: Option<Decimal>,
    /// The highest (lowest) market price seen since the trailing stop order was submitted.
    pub hwm: Option<Decimal>,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Market
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
/// The status of orders to return/
pub enum QueryOrderStatus {
    /// Only active orders.
    Open,
    /// Only inactive orders.
    Closed,
    /// Both active and inactive orders.
    All,
}

#[derive(Serialize, Clone, Debug, Default)]
/// Retrieves a list of orders for the account, filtered by the supplied query parameters.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     orders::{GetOrders, Order},
///     paper_client, Sort
/// };
/// use chrono::Utc;
///
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let orders: Vec<Order> = client
///         .send(
///             &GetOrders::new()
///                 .limit(50)
///                 .after(Utc::now())
///                 .until(Utc::now())
///                 .sort(Sort::Ascending)
///                 .nested(false)
///                 .symbols(["AAPL"]),
///         )
///         .await?;
///     Ok(())
/// }
/// ```
pub struct GetOrders {
    status: Option<QueryOrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "direction")]
    sort: Option<Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
}

impl GetOrders {
    /// Create a new request.
    pub fn new() -> Self {
        Default::default()
    }

    /// Order status to be queried. Open, Closed or All. Defaults to Open.
    pub fn status(mut self, status: QueryOrderStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// The maximum number of orders in response. Defaults to 50 and max is 500.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// The response will include only ones submitted after this timestamp (exclusive.)
    pub fn after(mut self, after: DateTime<Utc>) -> Self {
        self.after = Some(after);
        self
    }

    /// The response will include only ones submitted until this timestamp (exclusive.)
    pub fn until(mut self, until: DateTime<Utc>) -> Self {
        self.until = Some(until);
        self
    }

    /// The chronological order of response based on the submission time. Ascending or Descending. Defaults to
    /// Descending.
    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    /// If true, the result will roll up multi-leg orders under the legs field of primary order.
    pub fn nested(mut self, nested: bool) -> Self {
        self.nested = Some(nested);
        self
    }

    /// A list of the symbols to filter by.
    pub fn symbols<T1: IntoIterator<Item = T2>, T2: ToString>(mut self, symbols: T1) -> Self {
        self.symbols = Some(symbols.into_iter().map(|s| s.to_string()).collect());
        self
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
/// Retrieves a single order for the given order_id.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     orders::{GetOrder, Order},
///     paper_client,
/// };
/// use uuid::Uuid;
///
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let id = Uuid::nil();
///     let order: Order = client.send(&GetOrder::new(id).nested(false)).await?;
///     Ok(())
/// }
/// ```
pub struct GetOrder {
    #[serde(skip_serializing)]
    order_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    nested: Option<bool>,
}

impl GetOrder {
    /// Create a new request.
    pub fn new(order_id: Uuid) -> Self {
        Self {
            order_id,
            nested: None,
        }
    }

    /// If true, the result will roll up multi-leg orders under the legs field of primary order.
    pub fn nested(mut self, nested: bool) -> Self {
        self.nested = Some(nested);
        self
    }
}
impl Request for GetOrder {
    type Data = Self;
    type Response = Order;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/orders/{}", self.order_id).into()
    }

    fn data(&self) -> RequestData<&Self> {
        RequestData::Query(self)
    }
}

#[derive(Clone, Debug, Serialize)]
/// Places a new order for the given account. An order request may be rejected if the account is
/// not authorized for trading, or if the tradable balance is insufficient to fill the order.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     orders::{Order, OrderClass, OrderType, Side, SubmitOrder, TimeInForce},
///     paper_client,
/// };
/// use rust_decimal::Decimal;
///
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let order: Order = client
///         .send(
///             &SubmitOrder::new("AAPL")
///                 .qty(2)
///                 .side(Side::Sell)
///                 .time_in_force(TimeInForce::Day)
///                 .order_type(OrderType::limit(Decimal::new(100, 0)))
///                 .order_class(OrderClass::Simple)
///                 .extended_hours(false)
///                 .client_order_id("A"),
///         )
///         .await?;
///     Ok(())
/// }
/// ```
pub struct SubmitOrder {
    symbol: String,
    #[serde(
        deserialize_with = "crate::utils::from_str",
        serialize_with = "crate::utils::to_string"
    )]
    qty: usize,
    side: Side,
    #[serde(flatten, rename(serialize = "type"))]
    order_type: OrderType,
    time_in_force: TimeInForce,
    extended_hours: bool,
    client_order_id: Option<String>,
    #[serde(flatten)]
    order_class: OrderClass,
}

impl SubmitOrder {
    /// Create a new request.
    pub fn new<T: ToString>(symbol: T) -> Self {
        Self {
            symbol: symbol.to_string(),
            qty: 1,
            side: Side::Buy,
            order_type: OrderType::market(),
            time_in_force: TimeInForce::GoodTilCancelled,
            extended_hours: false,
            client_order_id: None,
            order_class: OrderClass::Simple,
        }
    }

    /// Update the quantity of the order.
    pub fn qty(mut self, qty: usize) -> Self {
        self.qty = qty;
        self
    }

    /// Update the side of the order.
    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    /// Update the order type of the order.
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = order_type;
        self
    }

    /// Update the time-in-force of the order.
    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = time_in_force;
        self
    }

    /// Set whether the order is in force during extended hours or not.
    pub fn extended_hours(mut self, extended_hours: bool) -> Self {
        self.extended_hours = extended_hours;
        self
    }

    /// Set the client order id of the order.
    pub fn client_order_id<T: ToString>(mut self, client_order_id: T) -> Self {
        self.client_order_id = Some(client_order_id.to_string());
        self
    }

    /// Set the order class of the order.
    pub fn order_class(mut self, order_class: OrderClass) -> Self {
        self.order_class = order_class;
        self
    }
}

impl Request for SubmitOrder {
    type Data = Self;
    type Response = Order;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<str> {
        "/v2/orders".into()
    }

    fn data(&self) -> RequestData<&Self::Data> {
        RequestData::Json(&self)
    }
}

#[derive(Clone, Debug, Serialize)]
/// Replaces a single order with updated parameters. Each parameter overrides the corresponding
/// attribute of the existing order. The other attributes remain the same as the existing order.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     orders::{Order, ReplaceOrder, TimeInForce},
///     paper_client,
/// };
/// use rust_decimal::Decimal;
/// use uuid::Uuid;
/// use vila::EmptyResponse;
///
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let id = Uuid::nil();
///     let order: Order = client
///         .send(
///             &ReplaceOrder::new(id)
///                 .qty(2)
///                 .time_in_force(TimeInForce::Day)
///                 .limit_price(Decimal::new(100, 0))
///                 .client_order_id("A"),
///         )
///         .await?;
///     Ok(())
/// }
/// ```
pub struct ReplaceOrder {
    #[serde(skip_serializing)]
    id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    qty: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trail: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_order_id: Option<String>,
}

impl ReplaceOrder {
    /// Create a new request
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            qty: None,
            time_in_force: None,
            limit_price: None,
            stop_price: None,
            trail: None,
            client_order_id: None,
        }
    }

    /// Update the quantity of the order
    pub fn qty(mut self, qty: usize) -> Self {
        self.qty = Some(qty);
        self
    }

    /// Update the time-in-force of the order
    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    /// Update the limit price of the order
    pub fn limit_price(mut self, limit_price: Decimal) -> Self {
        self.limit_price = Some(limit_price);
        self
    }

    /// Update the stop price of the order
    pub fn stop_price(mut self, stop_price: Decimal) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    /// Update the trail configuration of the order. If the order was originally sent with a
    /// trail_price configured, this updates the price. Otherwise, this updates the trail_percent.
    pub fn trail(mut self, trail: Decimal) -> Self {
        self.trail = Some(trail);
        self
    }

    /// Update the client order id of the order
    pub fn client_order_id<T: ToString>(mut self, client_order_id: T) -> Self {
        self.client_order_id = Some(client_order_id.to_string());
        self
    }
}

impl Request for ReplaceOrder {
    type Data = Self;
    type Response = Order;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/orders/{}", self.id).into()
    }

    fn data(&self) -> RequestData<&Self::Data> {
        RequestData::Json(&self)
    }
}

#[derive(Clone, Debug)]
/// Attempts to cancel an order.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     orders::{CancelOrder, Order},
///     paper_client,
/// };
/// use uuid::Uuid;
/// use vila::EmptyResponse;
///
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let id = Uuid::nil();
///     let _: EmptyResponse  = client.send(&CancelOrder::new(id)).await?;
///     Ok(())
/// }
/// ```
pub struct CancelOrder {
    id: Uuid,
}

impl CancelOrder {
    /// Create a new request
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

impl Request for CancelOrder {
    type Data = ();
    type Response = EmptyResponse;
    const METHOD: Method = Method::DELETE;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/orders/{}", self.id).into()
    }
}

#[derive(Clone, Debug)]
/// Attempts to cancel all open orders
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     orders::{CancelAllOrders, Order},
///     paper_client,
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let orders: Vec<Order> = client.send(&CancelAllOrders).await?;
///     Ok(())
/// }
/// ```
pub struct CancelAllOrders;

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
    fn defaults() {
        assert!(matches!(OrderType::default(), OrderType::Market));
        assert!(matches!(TimeInForce::default(), TimeInForce::Day));
        assert!(matches!(OrderClass::default(), OrderClass::Simple));
        assert!(matches!(Side::default(), Side::Buy));
    }

    #[tokio::test]
    async fn get_order() {
        let _m = mock("GET", "/v2/orders/904837e3-3b76-47ec-b432-046db621571b")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::UrlEncoded("nested".into(), "false".into()))
            .with_body(ORDER)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client
            .send(
                &GetOrder::new(Uuid::parse_str("904837e3-3b76-47ec-b432-046db621571b").unwrap())
                    .nested(false),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_orders() {
        let orders = format!("[{}]", ORDER);
        let _m = mock("GET", "/v2/orders")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("status".into(), "open".into()),
                Matcher::UrlEncoded("limit".into(), "50".into()),
                Matcher::UrlEncoded("direction".into(), "desc".into()),
                Matcher::UrlEncoded("nested".into(), "false".into()),
            ]))
            .with_body(orders)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client
            .send(
                &GetOrders::new()
                    .status(QueryOrderStatus::Open)
                    .limit(50)
                    .sort(Sort::Descending)
                    .nested(false),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn submit_simple_order() {
        let _m = mock("POST", "/v2/orders")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_body(ORDER_INTENT)
            .with_status(200)
            .with_body(ORDER)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = SubmitOrder::new("AAPL")
            .qty(15)
            .time_in_force(TimeInForce::Day)
            .client_order_id("904837e3-3b76-47ec-b432-046db621571b");
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn submit_complex_order() {
        let _m = mock("POST", "/v2/orders")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_body(COMPLEX_ORDER_INTENT)
            .with_status(200)
            .with_body(COMPLEX_ORDER)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = SubmitOrder::new("SPY")
            .client_order_id("904837e3-3b76-47ec-b432-046db621571b")
            .qty(100)
            .time_in_force(TimeInForce::GoodTilCancelled)
            .order_class(OrderClass::Bracket {
                take_profit: TakeProfitSpec {
                    limit_price: Decimal::new(301, 0),
                },
                stop_loss: StopLossSpec {
                    stop_price: Decimal::new(299, 0),
                    limit_price: Some(Decimal::new(2985, 1)),
                },
            });

        client.send(&req).await.unwrap();
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
            .send(&GetOrder::new(
                Uuid::parse_str("904837e3-3b76-47ec-b432-046db621571b").unwrap(),
            ))
            .await;

        assert!(res.is_err())
    }

    #[tokio::test]
    async fn cancel_order() {
        let _m = mock("DELETE", "/v2/orders/904837e3-3b76-47ec-b432-046db621571b")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_status(204)
            .create();

        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client
            .send(&CancelOrder::new(
                Uuid::parse_str("904837e3-3b76-47ec-b432-046db621571b").unwrap(),
            ))
            .await
            .unwrap();
    }

    const ORDER: &'static str = r#"{
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
    }"#;

    const COMPLEX_ORDER: &'static str = r#"{
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
	    "symbol": "SPY",
	    "asset_class": "us_equity",
	    "qty": "100",
	    "filled_qty": "0",
	    "type": "market",
	    "side": "buy",
	    "time_in_force": "gtc",
	    "status": "accepted",
	    "extended_hours": false,
	    "legs": [
            {
                "id": "904837e3-3b76-47ec-b432-046db621571c",
	            "client_order_id": "904837e3-3b76-47ec-b432-046db621571c",
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
	            "symbol": "SPY",
	            "asset_class": "us_equity",
	            "qty": "100",
	            "filled_qty": "0",
	            "type": "limit",
                "limit_price": "301",
	            "side": "sell",
	            "time_in_force": "gtc",
	            "status": "accepted",
	            "extended_hours": false,
	            "legs": null,
                "trail_price": null,
                "trail_percent": null,
                "hwm": null
            },
            {
                "id": "904837e3-3b76-47ec-b432-046db621571d",
	            "client_order_id": "904837e3-3b76-47ec-b432-046db621571d",
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
	            "symbol": "SPY",
	            "asset_class": "us_equity",
	            "qty": "100",
	            "filled_qty": "0",
	            "type": "stop_limit",
                "limit_price": "298.5",
                "stop_price": "299",
	            "side": "buy",
	            "time_in_force": "gtc",
	            "status": "accepted",
	            "extended_hours": false,
	            "legs": null,
                "trail_price": null,
                "trail_percent": null,
                "hwm": null
            }
        ],
        "trail_price": null,
        "trail_percent": null,
        "hwm": null
    }"#;
    const ORDER_INTENT: &'static str = r#"{"symbol":"AAPL","qty":"15","side":"buy","type":"market","time_in_force":"day","extended_hours":false,"client_order_id":"904837e3-3b76-47ec-b432-046db621571b","order_class":"simple"}"#;

    const COMPLEX_ORDER_INTENT: &'static str = r#"{"symbol":"SPY","qty":"100","side":"buy","type":"market","time_in_force":"gtc","extended_hours":false,"client_order_id":"904837e3-3b76-47ec-b432-046db621571b","order_class":"bracket","take_profit":{"limit_price":"301"},"stop_loss":{"stop_price":"299","limit_price":"298.5"}}"#;
}
