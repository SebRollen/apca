use apca_core::{AssetClass, Exchange, Identifier, Status};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use vila::{Request, RequestData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Asset {
    pub id: Uuid,
    pub class: AssetClass,
    pub exchange: Exchange,
    pub symbol: String,
    pub status: Status,
    pub tradable: bool,
    pub marginable: bool,
    pub shortable: bool,
    pub easy_to_borrow: bool,
    pub fractionable: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct GetAssets {
    pub status: Status,
    pub asset_class: AssetClass,
}

impl Request for GetAssets {
    type Data = Self;
    type Response = Asset;

    fn endpoint(&self) -> Cow<str> {
        "/v2/assets".into()
    }

    fn data(&self) -> RequestData<&Self> {
        RequestData::Query(self)
    }
}

#[derive(Clone, Debug)]
pub struct GetAsset {
    identifier: Identifier,
}

impl GetAsset {
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
        let _m = mock("GET", "/v2/assets")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("status".into(), "active".into()),
                Matcher::UrlEncoded("asset_class".into(), "us_equity".into()),
            ]))
            .with_body(
                r#"{
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
		        }"#,
            )
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
    async fn get_asset() {
        let _m = mock("GET", "/v2/assets/AAPL")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(
                r#"{
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
		        }"#,
            )
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&GetAsset::new("AAPL")).await.unwrap();
    }
}
