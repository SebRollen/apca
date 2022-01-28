use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use vila::Request;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// Clock object
pub struct Clock {
    /// Current timestamp
    pub timestamp: DateTime<Utc>,
    /// Whether or not the market is open
    pub is_open: bool,
    /// Next market open timestamp
    pub next_open: DateTime<Utc>,
    /// Next market close timestamp
    pub next_close: DateTime<Utc>,
}

#[derive(Clone, Debug)]
/// Returns the market clock.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     clock::{Clock, GetClock},
///     paper_client,
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let clock: Clock = client.send(&GetClock).await?;
///     Ok(())
/// }
/// ```
pub struct GetClock;

impl Request for GetClock {
    type Data = ();
    type Response = Clock;

    fn endpoint(&self) -> Cow<str> {
        "/v2/clock".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_with_url;
    use mockito::mock;

    #[tokio::test]
    async fn get_clock() {
        let _m = mock("GET", "/v2/clock")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(
                r#"{
                    "timestamp": "2018-04-01T12:00:00.000Z",
                    "is_open": true,
                    "next_open": "2018-04-01T12:00:00.000Z",
                    "next_close": "2018-04-01T12:00:00.000Z"
                }"#,
            )
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&GetClock).await.unwrap();
    }
}
