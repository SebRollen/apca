use crate::{AssetClass, Exchange, Identifier};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use vila::{Request, RequestData};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
/// Whether an asset is active or inactive on Alpaca
pub enum Status {
    /// Active asset
    Active,
    /// Inactive asset
    Inactive,
}

impl Default for Status {
    fn default() -> Self {
        Status::Active
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Asset object
pub struct Asset {
    /// Asset ID.
    pub id: Uuid,
    /// Asset class
    pub class: AssetClass,
    /// Primary exchange of asset.
    pub exchange: Exchange,
    /// Ticker of asset.
    pub symbol: String,
    /// Whether the asset is active or inactive.
    pub status: Status,
    /// Asset is tradable on Alpaca or not.
    pub tradable: bool,
    /// Asset is marginable or not.
    pub marginable: bool,
    /// Asset is shortable or not.
    pub shortable: bool,
    /// Asset is easy-to-borrow or not (filtering for easy_to_borrow = True is the best way to
    /// check whether the name is currently available to short at Alpaca).
    pub easy_to_borrow: bool,
    /// Asset is fractionable or not.
    pub fractionable: bool,
}

#[derive(Serialize, Clone, Debug)]
/// Get a list of assets
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     assets::{Asset, GetAssets, Status},
///     paper_client, AssetClass,
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let assets: Vec<Asset> = client
///         .send(
///             &GetAssets::new()
///                 .status(Status::Active)
///                 .asset_class(AssetClass::UsEquity),
///         )
///         .await?;
///     Ok(())
/// }
pub struct GetAssets {
    status: Status,
    asset_class: AssetClass,
}

impl GetAssets {
    /// Create a new request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the Status used to filter assets.
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    /// Set the AssetClass used to filter assets.
    pub fn asset_class(mut self, asset_class: AssetClass) -> Self {
        self.asset_class = asset_class;
        self
    }
}

impl Default for GetAssets {
    fn default() -> Self {
        Self {
            status: Status::Active,
            asset_class: AssetClass::UsEquity,
        }
    }
}

impl Request for GetAssets {
    type Data = Self;
    type Response = Vec<Asset>;

    fn endpoint(&self) -> Cow<str> {
        "/v2/assets".into()
    }

    fn data(&self) -> RequestData<&Self> {
        RequestData::Query(self)
    }
}

#[derive(Clone, Debug)]
/// Get an asset for the given symbol, or id.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     assets::{Asset, GetAsset},
///     paper_client,
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let asset: Asset = client.send(&GetAsset::new("AAPL")).await?;
///     Ok(())
/// }
pub struct GetAsset {
    identifier: Identifier,
}

impl GetAsset {
    /// Create a new request
    pub fn new<T: Into<Identifier>>(identifier: T) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }
}

impl Request for GetAsset {
    type Data = ();
    type Response = Asset;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/assets/{}", self.identifier).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_with_url;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn get_assets() {
        let assets = format!("[{}]", ASSET);
        let _m = mock("GET", "/v2/assets")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("status".into(), "active".into()),
                Matcher::UrlEncoded("asset_class".into(), "us_equity".into()),
            ]))
            .with_body(assets)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");
        let req = GetAssets {
            status: Status::Active,
            asset_class: AssetClass::UsEquity,
        };
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn get_asset_by_ticker() {
        let _m = mock("GET", "/v2/assets/AAPL")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(ASSET)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&GetAsset::new("AAPL")).await.unwrap();
    }

    #[tokio::test]
    async fn get_asset_by_id() {
        let _m = mock("GET", "/v2/assets/904837e3-3b76-47ec-b432-046db621571b")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(ASSET)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client
            .send(&GetAsset::new(
                Uuid::parse_str("904837e3-3b76-47ec-b432-046db621571b").unwrap(),
            ))
            .await
            .unwrap();
    }

    const ASSET: &'static str = r#"{
           "id": "904837e3-3b76-47ec-b432-046db621571b",
  		   "class": "us_equity",
  		   "exchange": "NASDAQ",
  		   "symbol": "AAPL",
  		   "status": "active",
  		   "tradable": true,
  		   "marginable": true,
  		   "shortable": true,
  		   "easy_to_borrow": true,
           "fractionable": true
        }"#;
}
