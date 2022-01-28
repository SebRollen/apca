use crate::assets::Asset;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use vila::{EmptyResponse, Method, Request, RequestData};

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Wachlist object
pub struct Watchlist {
    /// Watchlist id.
    pub id: Uuid,
    /// When the watchlist was created.
    pub created_at: DateTime<Utc>,
    /// When the watchlist was last updated.
    pub updated_at: DateTime<Utc>,
    /// User-defined watchlist name (up to 64 characters).
    pub name: String,
    /// account ID.
    pub account_id: Uuid,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// The content of this watchlist, in the order as registered by the client
    pub assets: Vec<Asset>,
}

#[derive(Clone, Debug)]
/// Returns the list of watchlists registered under the account.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     watchlists::{GetWatchlists, Watchlist},
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let watchlists: Vec<Watchlist> = client.send(&GetWatchlists).await?;
///     Ok(())
/// }
/// ```
pub struct GetWatchlists;

impl Request for GetWatchlists {
    type Data = ();
    type Response = Vec<Watchlist>;

    fn endpoint(&self) -> Cow<str> {
        "/v2/watchlists".into()
    }
}

#[derive(Clone, Debug)]
/// Returns a watchlist identified by the ID.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     watchlists::{GetWatchlist, Watchlist},
/// };
/// use uuid::Uuid;
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let id = Uuid::nil();
///     let watchlist: Watchlist = client.send(&GetWatchlist::new(id)).await?;
///     Ok(())
/// }
/// ```
pub struct GetWatchlist {
    id: Uuid,
}

impl GetWatchlist {
    /// Create a new request
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

impl Request for GetWatchlist {
    type Data = ();
    type Response = Watchlist;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/watchlists/{}", self.id).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Create a new watchlist with initial set of assets.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     watchlists::{CreateWatchlist, Watchlist},
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let watchlist: Watchlist = client
///         .send(&CreateWatchlist::new("List", ["AAPL", "TSLA"]))
///         .await?;
///     Ok(())
/// }
/// ```
pub struct CreateWatchlist {
    name: String,
    symbols: Vec<String>,
}

impl CreateWatchlist {
    /// Create a new request
    pub fn new<T1: IntoIterator<Item = T2>, T2: ToString>(name: T2, symbols: T1) -> Self {
        let symbols = symbols.into_iter().map(|s| s.to_string()).collect();
        Self {
            name: name.to_string(),
            symbols,
        }
    }
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
/// Update the name and/or content of watchlist.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     watchlists::{UpdateWatchlist, Watchlist},
/// };
/// use uuid::Uuid;
///
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let id = Uuid::nil();
///     let watchlist: Watchlist = client
///         .send(&UpdateWatchlist::new(id).name("name").symbols(["AAPL"]))
///         .await?;
///     Ok(())
/// }
/// ```
pub struct UpdateWatchlist {
    #[serde(skip_serializing)]
    id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
}

impl UpdateWatchlist {
    /// Create a new request
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            name: None,
            symbols: None,
        }
    }

    /// Specify the new name of the watchlist.
    pub fn name<T: ToString>(mut self, name: T) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Specify the new symbols in the watchlist
    pub fn symbols<T1: IntoIterator<Item = T2>, T2: ToString>(mut self, symbols: T1) -> Self {
        let symbols = symbols.into_iter().map(|x| x.to_string()).collect();
        self.symbols = Some(symbols);
        self
    }
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
/// Append an asset for the symbol to the end of watchlist asset list
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     watchlists::{AddAssetToWatchlist, Watchlist},
/// };
/// use uuid::Uuid;
///
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let id = Uuid::nil();
///     let watchlist: Watchlist = client.send(&AddAssetToWatchlist::new(id, "AAPL")).await?;
///     Ok(())
/// }
/// ```
pub struct AddAssetToWatchlist {
    #[serde(skip_serializing)]
    id: Uuid,
    symbol: String,
}

impl AddAssetToWatchlist {
    /// Create a new request
    pub fn new<T: ToString>(id: Uuid, symbol: T) -> Self {
        Self {
            id,
            symbol: symbol.to_string(),
        }
    }
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
/// Delete a watchlist. This is a permanent deletion.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     watchlists::{DeleteWatchlist, Watchlist},
/// };
/// use uuid::Uuid;
/// use vila::EmptyResponse;
///
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let id = Uuid::nil();
///     let _: EmptyResponse = client.send(&DeleteWatchlist::new(id)).await?;
///     Ok(())
/// }
/// ```
pub struct DeleteWatchlist {
    id: Uuid,
}

impl DeleteWatchlist {
    /// Create a new request
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

impl Request for DeleteWatchlist {
    type Data = ();
    type Response = EmptyResponse;
    const METHOD: Method = Method::DELETE;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/watchlists/{}", self.id).into()
    }
}

#[derive(Clone, Debug)]
/// Delete one entry for an asset by symbol name
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     paper_client,
///     watchlists::{RemoveAssetFromWatchlist, Watchlist},
/// };
/// use uuid::Uuid;
/// use vila::EmptyResponse;
///
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let id = Uuid::nil();
///     let _: EmptyResponse = client
///         .send(&RemoveAssetFromWatchlist::new(id, "AAPL"))
///         .await?;
///     Ok(())
/// }
/// ```
pub struct RemoveAssetFromWatchlist {
    id: Uuid,
    symbol: String,
}

impl RemoveAssetFromWatchlist {
    /// Create a new request
    pub fn new<T: ToString>(id: Uuid, symbol: T) -> Self {
        Self {
            id,
            symbol: symbol.to_string(),
        }
    }
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
    async fn get_watchlists() {
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
    async fn get_watchlist() {
        let _m = mock("GET", "/v2/watchlists/1d5493c9-ea39-4377-aa94-340734c368ae")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(WATCHLIST)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req =
            GetWatchlist::new(Uuid::parse_str("1d5493c9-ea39-4377-aa94-340734c368ae").unwrap());
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn create_watchlist() {
        let _m = mock("POST", "/v2/watchlists")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_body(r#"{"name":"Monday list","symbols":["SPY","AMZN"]}"#)
            .with_body(WATCHLIST)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = CreateWatchlist::new("Monday list", ["SPY", "AMZN"]);
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn update_watchlist() {
        let _m = mock("PUT", "/v2/watchlists/1d5493c9-ea39-4377-aa94-340734c368ae")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_body(r#"{"name":"Monday list","symbols":["SPY","AMZN"]}"#)
            .with_body(WATCHLIST)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req =
            UpdateWatchlist::new(Uuid::parse_str("1d5493c9-ea39-4377-aa94-340734c368ae").unwrap())
                .name("Monday list")
                .symbols(["SPY", "AMZN"]);
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn add_asset_to_watchlist() {
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
    async fn remove_asset_from_watchlist() {
        let _m = mock(
            "DELETE",
            "/v2/watchlists/1d5493c9-ea39-4377-aa94-340734c368ae/FB",
        )
        .match_header("apca-api-key-id", "APCA_API_KEY_ID")
        .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
        .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = RemoveAssetFromWatchlist::new(
            Uuid::parse_str("1d5493c9-ea39-4377-aa94-340734c368ae").unwrap(),
            "FB",
        );
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn delete_watchlist() {
        let _m = mock(
            "DELETE",
            "/v2/watchlists/1d5493c9-ea39-4377-aa94-340734c368ae",
        )
        .match_header("apca-api-key-id", "APCA_API_KEY_ID")
        .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
        .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req =
            DeleteWatchlist::new(Uuid::parse_str("1d5493c9-ea39-4377-aa94-340734c368ae").unwrap());
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
