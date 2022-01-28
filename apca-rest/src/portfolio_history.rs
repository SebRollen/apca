use crate::utils::datetime_from_vec_timestamp;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};
use vila::{Request, RequestData};

#[derive(Clone, Debug, Serialize, Deserialize)]
/// The resolution of the time window.
pub enum Timeframe {
    #[serde(rename = "1Min")]
    /// One minute resolution
    OneMinute,
    #[serde(rename = "5Min")]
    /// Five minutes resolution
    FiveMinutes,
    #[serde(rename = "15Min")]
    /// Fifteen minutes resolution
    FifteenMinutes,
    #[serde(rename = "1H")]
    /// One hour resolution
    OneHour,
    #[serde(rename = "1D")]
    /// One day resolution
    OneDay,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Portfolio history object
pub struct PortfolioHistory {
    /// Timestamp of each data element, left-labeled (the beginning of time window), in epoch
    /// seconds
    #[serde(deserialize_with = "datetime_from_vec_timestamp")]
    pub timestamp: Vec<DateTime<Utc>>,
    /// Equity value of the account in dollar amount as of the end of each time window
    pub equity: Vec<Decimal>,
    /// Profit/loss in dollar from the base value
    pub profit_loss: Vec<Decimal>,
    /// Profit/loss in percentage from the base value
    pub profit_loss_pct: Vec<Decimal>,
    /// Basis in dollar of the profit loss calculation
    pub base_value: Decimal,
    /// Time window size of each data element
    pub timeframe: Timeframe,
}

#[derive(Clone, Debug)]
enum PeriodUnit {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Clone, Debug)]
/// The amount of time to query for portfolio history
pub struct Period(usize, PeriodUnit);

impl Period {
    /// Creates a period of n number of days
    pub fn days(n: usize) -> Period {
        Period(n, PeriodUnit::Day)
    }

    /// Creates a period of n number of weeks
    pub fn weeks(n: usize) -> Period {
        Period(n, PeriodUnit::Week)
    }

    /// Creates a period of n number of months
    pub fn months(n: usize) -> Period {
        Period(n, PeriodUnit::Month)
    }

    /// Creates a period of n number of years
    pub fn years(n: usize) -> Period {
        Period(n, PeriodUnit::Year)
    }
}

impl Serialize for Period {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = format!("{}{}", self.0, self.1);
        serializer.serialize_str(&formatted)
    }
}

impl Display for PeriodUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PeriodUnit::Day => write!(f, "D"),
            PeriodUnit::Week => write!(f, "W"),
            PeriodUnit::Month => write!(f, "M"),
            PeriodUnit::Year => write!(f, "A"),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
/// Returns timeseries data about equity and profit/loss (P/L) of the account in requested
/// timespan.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     portfolio_history::{GetPortfolioHistory, Period, PortfolioHistory, Timeframe},
/// };
/// use chrono::NaiveDate;
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let history: PortfolioHistory = client
///         .send(
///             &GetPortfolioHistory::new()
///                 .period(Period::years(1))
///                 .timeframe(Timeframe::OneDay)
///                 .date_end(NaiveDate::from_ymd(2021, 12, 31))
///                 .extended_hours(false),
///         )
///         .await?;
///     Ok(())
/// }
/// ```
pub struct GetPortfolioHistory {
    period: Option<Period>,
    timeframe: Option<Timeframe>,
    date_end: Option<NaiveDate>,
    extended_hours: Option<bool>,
}

impl GetPortfolioHistory {
    /// Create a new request
    pub fn new() -> Self {
        Self::default()
    }

    /// The duration of the data. Defaults to one month
    pub fn period(mut self, period: Period) -> Self {
        self.period = Some(period);
        self
    }

    /// The resolution of the time-window. Defaults to 1 minute if period is less than a week -
    /// otherwise it defaults to one day.
    pub fn timeframe(mut self, timeframe: Timeframe) -> Self {
        self.timeframe = Some(timeframe);
        self
    }

    /// The date the data is returned to. Defaults to current date.
    pub fn date_end(mut self, date_end: NaiveDate) -> Self {
        self.date_end = Some(date_end);
        self
    }

    /// If true, include extended hours in the result. This is effective only for timeframe less
    /// than OneDay
    pub fn extended_hours(mut self, extended_hours: bool) -> Self {
        self.extended_hours = Some(extended_hours);
        self
    }
}

impl Request for GetPortfolioHistory {
    type Data = Self;
    type Response = PortfolioHistory;

    fn endpoint(&self) -> Cow<str> {
        "/v2/account/portfolio/history".into()
    }

    fn data(&self) -> RequestData<&Self> {
        RequestData::Query(&self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_with_url;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn get_portfolio_history() {
        let _m = mock("GET", "/v2/account/portfolio/history")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("period".into(), "1D".into()),
                Matcher::UrlEncoded("timeframe".into(), "1Min".into()),
                Matcher::UrlEncoded("date_end".into(), "2021-01-01".into()),
                Matcher::UrlEncoded("extended_hours".into(), "false".into()),
            ]))
            .with_body(PORTFOLIO_HISTORY)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = GetPortfolioHistory::new()
            .period(Period::days(1))
            .timeframe(Timeframe::OneMinute)
            .date_end(NaiveDate::from_ymd(2021, 1, 1))
            .extended_hours(false);
        client.send(&req).await.unwrap();
    }

    const PORTFOLIO_HISTORY: &'static str = r#"{
	    "timestamp": [1580826600000, 1580827500000, 1580828400000],
  		"equity": [27423.73, 27408.19, 27515.97],
  		"profit_loss": [11.8, -3.74, 104.04],
  		"profit_loss_pct": [0.000430469507254688, -0.0001364369455197062, 0.0037954277571845543],
  		"base_value": 27411.93,
  		"timeframe": "15Min"
	}"#;
}
