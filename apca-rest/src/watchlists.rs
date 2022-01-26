use crate::Asset;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use vila::{EmptyResponse, Method, Request, RequestData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Watchlist {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub account_id: Uuid,
    pub assets: Vec<Asset>,
}

pub struct GetWatchlists {}

impl Request for GetWatchlists {
    type Data = ();
    type Response = Vec<Watchlist>;

    fn endpoint(&self) -> Cow<str> {
        "/v2/watchlists".into()
    }
}

pub struct GetWatchlist(pub Uuid);

impl Request for GetWatchlist {
    type Data = ();
    type Response = Watchlist;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/watchlists/{}", self.0).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateWatchlist {
    pub name: String,
    pub symbols: Vec<String>,
}

impl Request for CreateWatchlist {
    type Data = Self;
    type Response = Watchlist;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<str> {
        "/v2/watchlists".into()
    }

    fn data(&self) -> RequestData<&Self::Data> {
        RequestData::Json(&self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateWatchlist {
    #[serde(skip_serializing)]
    pub id: Uuid,
    pub name: Option<String>,
    pub symbols: Option<Vec<String>>,
}

impl Request for UpdateWatchlist {
    type Data = Self;
    type Response = Watchlist;
    const METHOD: Method = Method::PUT;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/watchlists/{}", self.id).into()
    }

    fn data(&self) -> RequestData<&Self::Data> {
        RequestData::Json(&self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddAssetToWatchlist {
    #[serde(skip_serializing)]
    pub id: Uuid,
    pub symbol: String,
}

impl Request for AddAssetToWatchlist {
    type Data = Self;
    type Response = Watchlist;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/watchlists/{}", self.id).into()
    }

    fn data(&self) -> RequestData<&Self::Data> {
        RequestData::Json(&self)
    }
}

#[derive(Clone, Debug)]
pub struct DeleteWatchlist(pub Uuid);

impl Request for DeleteWatchlist {
    type Data = ();
    type Response = EmptyResponse;
    const METHOD: Method = Method::DELETE;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/watchlists/{}", self.0).into()
    }
}

#[derive(Clone, Debug)]
pub struct RemoveAssetFromWatchlist {
    pub id: Uuid,
    pub symbol: String,
}

impl Request for RemoveAssetFromWatchlist {
    type Data = ();
    type Response = EmptyResponse;
    const METHOD: Method = Method::DELETE;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/watchlists/{}/{}", self.id, self.symbol).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_with_url;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_watchlists() {
        let watchlist_list = format!("[{}]", WATCHLIST);
        let _m = mock("GET", "/v2/watchlists")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(watchlist_list)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = GetWatchlists {};
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_watchlist() {
        let _m = mock("GET", "/v2/watchlists/1d5493c9-ea39-4377-aa94-340734c368ae")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(WATCHLIST)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = GetWatchlist(Uuid::parse_str("1d5493c9-ea39-4377-aa94-340734c368ae").unwrap());
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_watchlist() {
        let _m = mock("POST", "/v2/watchlists")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_body(r#"{"name":"Monday list","symbols":["SPY","AMZN"]}"#)
            .with_body(WATCHLIST)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = CreateWatchlist {
            name: "Monday list".to_string(),
            symbols: vec!["SPY".into(), "AMZN".into()],
        };
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_watchlist() {
        let _m = mock("PUT", "/v2/watchlists/1d5493c9-ea39-4377-aa94-340734c368ae")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_body(r#"{"name":"Monday list","symbols":["SPY","AMZN"]}"#)
            .with_body(WATCHLIST)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = UpdateWatchlist {
            id: Uuid::parse_str("1d5493c9-ea39-4377-aa94-340734c368ae").unwrap(),
            name: Some("Monday list".to_string()),
            symbols: Some(vec!["SPY".into(), "AMZN".into()]),
        };
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn test_add_asset_to_watchlist() {
        let _m = mock(
            "POST",
            "/v2/watchlists/1d5493c9-ea39-4377-aa94-340734c368ae",
        )
        .match_header("apca-api-key-id", "APCA_API_KEY_ID")
        .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
        .match_body(r#"{"symbol":"SPY"}"#)
        .with_body(WATCHLIST)
        .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = AddAssetToWatchlist {
            id: Uuid::parse_str("1d5493c9-ea39-4377-aa94-340734c368ae").unwrap(),
            symbol: "SPY".into(),
        };
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn test_remove_asset_from_watchlist() {
        let _m = mock(
            "DELETE",
            "/v2/watchlists/1d5493c9-ea39-4377-aa94-340734c368ae/FB",
        )
        .match_header("apca-api-key-id", "APCA_API_KEY_ID")
        .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
        .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = RemoveAssetFromWatchlist {
            id: Uuid::parse_str("1d5493c9-ea39-4377-aa94-340734c368ae").unwrap(),
            symbol: "FB".into(),
        };
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_watchlist() {
        let _m = mock(
            "DELETE",
            "/v2/watchlists/1d5493c9-ea39-4377-aa94-340734c368ae",
        )
        .match_header("apca-api-key-id", "APCA_API_KEY_ID")
        .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
        .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = DeleteWatchlist(Uuid::parse_str("1d5493c9-ea39-4377-aa94-340734c368ae").unwrap());
        client.send(&req).await.unwrap();
    }

    const WATCHLIST: &'static str = r#"{
    	"account_id": "1d5493c9-ea39-4377-aa94-340734c368ae",
    	"assets": [
    	    {
    	        "class": "us_equity",
    	        "easy_to_borrow": true,
    	        "exchange": "ARCA",
    	        "id": "b28f4066-5c6d-479b-a2af-85dc1a8f16fb",
    	        "marginable": true,
    	        "shortable": true,
    	        "status": "active",
    	        "symbol": "SPY",
    	        "tradable": true,
                "fractionable": true
    	    },
    	    {
    	        "class": "us_equity",
    	        "easy_to_borrow": false,
    	        "exchange": "NASDAQ",
    	        "id": "f801f835-bfe6-4a9d-a6b1-ccbb84bfd75f",
    	        "marginable": true,
    	        "shortable": false,
    	        "status": "active",
    	        "symbol": "AMZN",
    	        "tradable": true,
                "fractionable": true
    	    }
    	],
    	"created_at": "2019-10-30T07:54:42.981322Z",
    	"id": "fb306e55-16d3-4118-8c3d-c1615fcd4c03",
    	"name": "Monday List",
    	"updated_at": "2019-10-30T07:54:42.981322Z"
	}"#;
}
