use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};
use vila::{Request, RequestData};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Timeframe {
    #[serde(rename = "1Min")]
    OneMinute,
    #[serde(rename = "5Min")]
    FiveMinutes,
    #[serde(rename = "15Min")]
    FifteenMinutes,
    #[serde(rename = "1H")]
    OneHour,
    #[serde(rename = "1D")]
    OneDay,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortfolioHistory {
    // TODO: Make this Vec<DateTime<Utc>> after writing deserializer
    pub timestamp: Vec<usize>,
    pub equity: Vec<Decimal>,
    pub profit_loss: Vec<Decimal>,
    pub profit_loss_pct: Vec<Decimal>,
    pub base_value: Decimal,
    pub timeframe: Timeframe,
}

#[derive(Clone, Debug)]
pub enum PeriodUnit {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Clone, Debug)]
pub struct Period(pub usize, pub PeriodUnit);

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

#[derive(Clone, Debug, Serialize)]
pub struct GetPortfolioHistory {
    pub period: Period,
    pub timeframe: Timeframe,
    pub date_end: NaiveDate,
    pub extended_hours: bool,
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
    async fn test_get_portfolio_history() {
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

        let req = GetPortfolioHistory {
            period: Period(1, PeriodUnit::Day),
            timeframe: Timeframe::OneMinute,
            date_end: NaiveDate::from_ymd(2021, 1, 1),
            extended_hours: false,
        };
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
