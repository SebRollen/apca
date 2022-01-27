use crate::utils::{hm_from_str, hm_to_string};
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use vila::{Request, RequestData};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Calendar {
    pub date: NaiveDate,
    #[serde(deserialize_with = "hm_from_str", serialize_with = "hm_to_string")]
    pub open: NaiveTime,
    #[serde(deserialize_with = "hm_from_str", serialize_with = "hm_to_string")]
    pub close: NaiveTime,
}

#[derive(Serialize, Clone, Debug)]
pub struct GetCalendar {
    pub start: NaiveDate,
    pub end: NaiveDate,
}
impl GetCalendar {
    pub fn new() -> Self {
        Default::default()
    }
}
impl Default for GetCalendar {
    fn default() -> Self {
        Self {
            start: NaiveDate::from_ymd(1970, 1, 1),
            end: NaiveDate::from_ymd(2029, 12, 31),
        }
    }
}

impl Request for GetCalendar {
    type Data = Self;
    type Response = Vec<Calendar>;

    fn endpoint(&self) -> Cow<str> {
        "/v2/calendar".into()
    }

    fn data(&self) -> RequestData<&Self> {
        RequestData::Query(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_with_url;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn get_calendar() {
        let _m = mock("GET", "/v2/calendar")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("start".into(), "1970-01-01".into()),
                Matcher::UrlEncoded("end".into(), "2029-12-31".into()),
            ]))
            .with_body(
                r#"[
		        {
			        "date": "2018-01-03",
			        "open": "09:30",
			        "close": "16:00"
		       }
            ]"#,
            )
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&GetCalendar::new()).await.unwrap();
    }
}
