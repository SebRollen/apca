use crate::utils::*;
use crate::Sort;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::CommaSeparator;
use std::borrow::Cow;
use std::collections::HashMap;
use uuid::Uuid;
use vila::pagination::{
    query::{QueryModifier, QueryPaginator},
    PaginatedRequest,
};
use vila::{Request, RequestData};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
/// Whether a TradeActivity is a partial or full fill.
pub enum FillType {
    /// Full fill
    Fill,
    /// Partial fill
    PartialFill,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Whether a TradeActivity is a buy, sell or short-sell.
pub enum Side {
    #[serde(rename = "buy")]
    /// Buy side
    Buy,
    #[serde(rename = "sell")]
    /// Sell side
    Sell,
    #[serde(rename = "sell_short")]
    /// Short sale
    SellShort,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
/// Activity
pub enum Activity {
    /// Activity type involved in trades.
    TradeActivity {
        /// Fill
        activity_type: ActivityType,
        /// The cumulative quantity of shares involved in the execution.
        cum_qty: Decimal,
        /// An id for the activity. Always in “::” format. Can be sent as page_token in requests to facilitate the paging of results.
        id: String,
        /// For PartialFill orders, the quantity of shares that are left to be filled.
        leaves_qty: Decimal,
        /// The per-share price that the trade was executed at.
        price: Decimal,
        /// The number of shares involved in the trade execution.
        qty: Decimal,
        /// The side of the trade execution.
        side: Side,
        /// The symbol of the security being traded.
        symbol: String,
        /// The time at which the execution occurred.
        transaction_time: DateTime<Utc>,
        /// The id for the order that filled.
        order_id: Uuid,
        #[serde(rename(serialize = "type", deserialize = "type"))]
        /// The type of fill being reported.
        fill_type: FillType,
    },
    /// Activity type not involved in trades
    NonTradeActivity {
        /// See ActivityType for a list of possible values.
        activity_type: ActivityType,
        /// An ID for the activity, always in “::” format. Can be sent as page_token in requests to
        /// facilitate the paging of results.
        id: String,
        /// The date on which the activity occurred or on which the transaction associated with the
        /// activity settled.
        date: NaiveDate,
        /// The net amount of money (positive or negative) associated with the activity.
        net_amount: Decimal,
        /// The symbol of the security involved with the activity. Not present for all activity
        /// types.
        symbol: Option<String>,
        #[serde(
            deserialize_with = "from_str_optional",
            serialize_with = "to_string_optional"
        )]
        /// For dividend activities, the number of shares that contributed to the payment. Not
        /// present for other activity types.
        qty: Option<i32>,
        /// For dividend activities, the average amount paid per share. Not present for other
        /// activity types.
        per_share_amount: Option<Decimal>,
    },
}

impl Activity {
    /// Get the id of the activity.
    fn id(&self) -> &str {
        match self {
            Activity::TradeActivity { id, .. } => id,
            Activity::NonTradeActivity { id, .. } => id,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// The types of activities that may be reported
pub enum ActivityType {
    #[serde(rename = "FILL")]
    /// Order fills (both partial and full fills)
    Fill,
    #[serde(rename = "TRANS")]
    /// Cash transactions (both CashDeposit and CashWithdrawal)
    CashTransactions,
    #[serde(rename = "MISC")]
    /// Miscellaneous or rarely used activity types (All types except those in CashTransactions, Dividend, or Fill)
    Miscellaneous,
    #[serde(rename = "ACATC")]
    /// ACATS IN/OUT (Cash)
    AcatsCash,
    #[serde(rename = "ACATS")]
    /// ACATS IN/OUT (Securities)
    AcatsSecurities,
    #[serde(rename = "CSD")]
    /// Cash deposit(+)
    CashDeposit,
    #[serde(rename = "CSW")]
    /// Cash withdrawal(-)
    CashWithdrawal,
    #[serde(rename = "DIV")]
    /// Dividends
    Dividend,
    #[serde(rename = "DIVCGL")]
    /// Dividend (capital gain long term)
    DividendLongTermCapitalGain,
    #[serde(rename = "DIVCGS")]
    /// Dividend (capital gain short term)
    DividendShortTermCapitalGain,
    #[serde(rename = "DIVFEE")]
    /// Dividend fee
    DividendFee,
    #[serde(rename = "DIVFT")]
    /// Dividend adjusted (Foreign Tax Withheld)
    DividendForeignTaxWithheld,
    #[serde(rename = "DIVNRA")]
    /// Dividend adjusted (NRA Withheld)
    DividendNraWithheld,
    #[serde(rename = "DIVROC")]
    /// Dividend return of capital
    DividendReturnOfCapital,
    #[serde(rename = "DIVTXEX")]
    /// Dividend adjusted (Tefra Withheld)
    DividendTefraWithheld,
    #[serde(rename = "DIVTXEX")]
    /// Dividend (tax exempt)
    DividendTaxExempt,
    #[serde(rename = "INT")]
    /// Interest (credit/margin)
    Interest,
    #[serde(rename = "INTNRA")]
    /// Interest adjusted (NRA Withheld)
    InterestNraWithheld,
    #[serde(rename = "INTTW")]
    /// Interest adjusted (Tefra Withheld)
    InterestTefraWithheld,
    #[serde(rename = "JNL")]
    /// Journal entry
    JournalEntry,
    #[serde(rename = "JNLC")]
    /// Journal entry (cash)
    JournalEntryCash,
    #[serde(rename = "JNLS")]
    /// Journal entry (stock)
    JournalEntryStock,
    #[serde(rename = "MA")]
    /// Merger/Acquisition
    MergerAcquisition,
    #[serde(rename = "NC")]
    /// Name change
    NameChange,
    #[serde(rename = "OPASN")]
    /// Option assignment
    OptionAssignment,
    #[serde(rename = "OPEXP")]
    /// Option expiration
    OptionExpiration,
    #[serde(rename = "OPXRC")]
    /// Option exercise
    OptionExercise,
    #[serde(rename = "PTC")]
    /// Pass through charge
    PassThroughCharge,
    #[serde(rename = "PTR")]
    /// Pass through rebate
    PassThroughRebate,
    #[serde(rename = "REORG")]
    /// Reorg CA
    Reorgnization,
    #[serde(rename = "SC")]
    /// Symbol change
    SymbolChange,
    #[serde(rename = "SSO")]
    /// Stock spinoff
    StockSpinoff,
    #[serde(rename = "SSP")]
    /// Stock split
    StockSplit,
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = serde_plain::to_string(self).unwrap();
        formatter.write_str(&string)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
enum DateSpec {
    Date {
        date: NaiveDate,
    },
    UntilAfter {
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        after: Option<NaiveDate>,
    },
}

#[derive(Clone, Debug, Default, Serialize)]
/// Returns account activity entries for many types of activities.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     account_activities::{Activity, GetAccountActivities},
///     paper_client, Sort
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let activities: Vec<Activity> = client
///         .send(
///             &GetAccountActivities::new()
///                 .sort(Sort::Ascending)
///                 .page_size(10),
///         )
///         .await?;
///     Ok(())
/// }
/// ```
pub struct GetAccountActivities {
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        with = "serde_with::rust::StringWithSeparator::<CommaSeparator>"
    )]
    activity_types: Vec<ActivityType>,
    #[serde(flatten)]
    date_spec: Option<DateSpec>,
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    sort: Option<Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

impl GetAccountActivities {
    /// Create a new request
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an activity to the request.
    pub fn add_activity(mut self, activity_type: ActivityType) -> Self {
        self.activity_types.push(activity_type);
        self
    }

    /// Add multiple activities to the request.
    pub fn add_activities(mut self, activity_types: &[ActivityType]) -> Self {
        self.activity_types.extend_from_slice(activity_types);
        self
    }

    /// Specify the date of the returned values
    pub fn on_date(mut self, date: NaiveDate) -> Self {
        self.date_spec = Some(DateSpec::Date { date });
        self
    }

    /// Specify the maximum date of the returned values
    pub fn before_date(mut self, date: NaiveDate) -> Self {
        match self.date_spec {
            None | Some(DateSpec::Date { .. }) => {
                self.date_spec = Some(DateSpec::UntilAfter {
                    until: Some(date),
                    after: None,
                })
            }
            Some(DateSpec::UntilAfter { after, .. }) => {
                self.date_spec = Some(DateSpec::UntilAfter {
                    until: Some(date),
                    after,
                })
            }
        }
        self
    }

    /// Specify the minimum date of the returned values
    pub fn after_date(mut self, date: NaiveDate) -> Self {
        match self.date_spec {
            None | Some(DateSpec::Date { .. }) => {
                self.date_spec = Some(DateSpec::UntilAfter {
                    after: Some(date),
                    until: None,
                })
            }
            Some(DateSpec::UntilAfter { until, .. }) => {
                self.date_spec = Some(DateSpec::UntilAfter {
                    after: Some(date),
                    until,
                })
            }
        }
        self
    }

    /// Specify the sort order of the returned values.
    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Specify the number of items returned by the request. The max is 100.
    pub fn page_size(mut self, page_size: usize) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Specify the id of the activity from where the pagination should begin.
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }
}

impl Request for GetAccountActivities {
    type Data = Self;
    type Response = Vec<Activity>;

    fn endpoint(&self) -> Cow<str> {
        "/v2/account/activities".into()
    }

    fn data(&self) -> RequestData<&Self::Data> {
        RequestData::Query(&self)
    }
}

#[derive(Clone, Debug, Serialize)]
/// Returns account activity entries for a specific type of activity.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     account_activities::{Activity, ActivityType, GetAccountActivitiesByType},
///     paper_client, Sort
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let activities: Vec<Activity> = client
///         .send(
///             &GetAccountActivitiesByType::new(ActivityType::Fill)
///                 .sort(Sort::Ascending)
///                 .page_size(10),
///         )
///         .await?;
///     Ok(())
/// }
/// ```
pub struct GetAccountActivitiesByType {
    #[serde(skip_serializing)]
    activity_type: ActivityType,
    #[serde(flatten)]
    date_spec: Option<DateSpec>,
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    sort: Option<Sort>, // TODO: Finish this struct
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

impl GetAccountActivitiesByType {
    /// Create a new request
    pub fn new(activity_type: ActivityType) -> Self {
        Self {
            activity_type,
            date_spec: None,
            sort: None,
            page_size: None,
            page_token: None,
        }
    }

    /// Specify the date of the returned values
    pub fn on_date(mut self, date: NaiveDate) -> Self {
        self.date_spec = Some(DateSpec::Date { date });
        self
    }

    /// Specify the maximum date of the returned values
    pub fn before_date(mut self, date: NaiveDate) -> Self {
        match self.date_spec {
            None | Some(DateSpec::Date { .. }) => {
                self.date_spec = Some(DateSpec::UntilAfter {
                    until: Some(date),
                    after: None,
                })
            }
            Some(DateSpec::UntilAfter { after, .. }) => {
                self.date_spec = Some(DateSpec::UntilAfter {
                    until: Some(date),
                    after,
                })
            }
        }
        self
    }

    /// Specify the minimum date of the returned values
    pub fn after_date(mut self, date: NaiveDate) -> Self {
        match self.date_spec {
            None | Some(DateSpec::Date { .. }) => {
                self.date_spec = Some(DateSpec::UntilAfter {
                    after: Some(date),
                    until: None,
                })
            }
            Some(DateSpec::UntilAfter { until, .. }) => {
                self.date_spec = Some(DateSpec::UntilAfter {
                    after: Some(date),
                    until,
                })
            }
        }
        self
    }

    /// Specify the sort order of the returned values.
    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Specify the number of items returned by the request. The max is 100.
    pub fn page_size(mut self, page_size: usize) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Specify the id of the activity from where the pagination should begin.
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }
}

impl Request for GetAccountActivitiesByType {
    type Data = Self;
    type Response = Vec<Activity>;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/account/activities/{}", self.activity_type).into()
    }

    fn data(&self) -> RequestData<&Self::Data> {
        RequestData::Query(&self)
    }
}

#[derive(Clone, Debug)]
// TODO: Find way in Vila to make this struct private
/// Struct used for pagination. Users should never interact with this struct directly, but it is
/// used under the hood when sending a paginated request.
pub struct AccountActivitiesPage {
    page_size: usize,
    page_token: String,
}

impl From<AccountActivitiesPage> for QueryModifier {
    fn from(page: AccountActivitiesPage) -> QueryModifier {
        let mut data = HashMap::new();
        data.insert("page_size".into(), page.page_size.to_string());
        data.insert("page_token".into(), page.page_token.into());
        QueryModifier { data }
    }
}

impl PaginatedRequest for GetAccountActivitiesByType {
    type Data = AccountActivitiesPage;
    type Paginator = QueryPaginator<Self::Response, AccountActivitiesPage>;
    fn paginator(&self) -> Self::Paginator {
        QueryPaginator::new(
            |prev: Option<&AccountActivitiesPage>, res: &Vec<Activity>| {
                res.last().map(|x| AccountActivitiesPage {
                    page_size: prev.map(|y| y.page_size).unwrap_or(100),
                    page_token: x.id().to_string(),
                })
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_with_url;
    use mockito::mock;

    #[tokio::test]
    async fn get_account_activities() {
        let both_activities = format!("[{},{}]", TRADE_ACTIVITY, NONTRADE_ACTIVITY);
        let _m = mock("GET", "/v2/account/activities")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query("until=2022-01-01&after=2000-01-01")
            .with_body(both_activities)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = GetAccountActivities::new()
            .after_date(NaiveDate::from_ymd(2000, 1, 1))
            .before_date(NaiveDate::from_ymd(2022, 1, 1));
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn get_only_fill_account_activities() {
        let fill_activities = format!("[{}]", TRADE_ACTIVITY);
        let _m = mock("GET", "/v2/account/activities")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query("activity_types=FILL&date=2020-01-01")
            .with_body(fill_activities)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = GetAccountActivities::new()
            .add_activity(ActivityType::Fill)
            .on_date(NaiveDate::from_ymd(2020, 1, 1));
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn get_account_activities_by_type() {
        let div_activities = format!("[{}]", NONTRADE_ACTIVITY);
        let _m = mock("GET", "/v2/account/activities/DIV")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(div_activities)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = GetAccountActivitiesByType::new(ActivityType::Dividend);
        client.send(&req).await.unwrap();
    }

    const TRADE_ACTIVITY: &'static str = r#"{
  		"activity_type": "FILL",
  		"cum_qty": "1",
  		"id": "20190524113406977::8efc7b9a-8b2b-4000-9955-d36e7db0df74",
  		"leaves_qty": "0",
  		"price": "1.63",
  		"qty": "1",
  		"side": "buy",
  		"symbol": "LPCN",
  		"transaction_time": "2019-05-24T15:34:06.977Z",
  		"order_id": "904837e3-3b76-47ec-b432-046db621571b",
  		"type": "fill"
	}"#;
    const NONTRADE_ACTIVITY: &'static str = r#"{
  		"activity_type": "DIV",
  		"id": "20190801011955195::5f596936-6f23-4cef-bdf1-3806aae57dbf",
  		"date": "2019-08-01",
  		"net_amount": "1.02",
  		"symbol": "T",
  		"qty": "2",
  		"per_share_amount": "0.51"
	}"#;
}
