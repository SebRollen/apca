use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use vila::{Method, Request, RequestData};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DtbpCheck {
    Both,
    Entry,
    Exit,
}
impl Default for DtbpCheck {
    fn default() -> Self {
        Self::Entry
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TradeConfirmEmail {
    All,
    #[serde(rename = "none")]
    Zero,
}
impl Default for TradeConfirmEmail {
    fn default() -> Self {
        Self::All
    }
}

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct AccountConfigurations {
    pub dtbp_check: DtbpCheck,
    pub trade_confirm_email: TradeConfirmEmail,
    pub suspend_trade: bool,
    pub no_shorting: bool,
}

impl AccountConfigurations {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Clone, Debug)]
pub struct GetAccountConfigurations;

impl Request for GetAccountConfigurations {
    type Data = ();
    type Response = AccountConfigurations;

    fn endpoint(&self) -> Cow<str> {
        "/v2/account/configurations".into()
    }
}

#[derive(Clone, Debug)]
pub struct PatchAccountConfigurations(AccountConfigurations);

impl Request for PatchAccountConfigurations {
    type Data = AccountConfigurations;
    type Response = AccountConfigurations;
    const METHOD: Method = Method::PATCH;

    fn endpoint(&self) -> Cow<str> {
        "/v2/account/configurations".into()
    }

    fn data(&self) -> RequestData<&AccountConfigurations> {
        RequestData::Json(&self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_with_url;
    use mockito::mock;

    #[tokio::test]
    async fn get_account_configurations() {
        let _m = mock("GET", "/v2/account/configurations")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(
                r#"{
               	  "dtbp_check": "entry",
 		  "no_shorting": false,
 		  "suspend_trade": false,
 		  "trade_confirm_email": "all" 
		}"#,
            )
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&GetAccountConfigurations).await.unwrap();
    }

    #[tokio::test]
    async fn patch_account_configurations() {
        let _m = mock("PATCH", "/v2/account/configurations")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_body(r#"{"dtbp_check":"entry","trade_confirm_email":"all","suspend_trade":false,"no_shorting":false}"#)
            .with_body(
                r#"{
               	  "dtbp_check": "entry",
 		  "no_shorting": false,
 		  "suspend_trade": false,
 		  "trade_confirm_email": "all" 
		}"#,
            )
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client
            .send(&PatchAccountConfigurations(AccountConfigurations::new()))
            .await
            .unwrap();
    }
}
