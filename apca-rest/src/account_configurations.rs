use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use vila::{Method, Request, RequestData};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
/// Controls Day Trading Margin Call (DTMC) checks.
pub enum DtbpCheck {
    /// Check on both entry and exit
    Both,
    /// Only check on entry
    Entry,
    /// Only check on exit
    Exit,
}

impl Default for DtbpCheck {
    fn default() -> Self {
        Self::Entry
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
/// If `Zero`, emails for order fills are not sent.
pub enum TradeConfirmEmail {
    /// Send emails for all trades
    All,
    #[serde(rename = "none")]
    /// Don't send emails
    Zero,
}

impl Default for TradeConfirmEmail {
    fn default() -> Self {
        Self::All
    }
}

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
/// AccountConfigurations object
pub struct AccountConfigurations {
    /// Controls Day Trading Margin Call (DTMC) checks.
    pub dtbp_check: DtbpCheck,
    /// If `Zero`, emails for order fills are not sent.
    pub trade_confirm_email: TradeConfirmEmail,
    /// If true, new orders are blocked.
    pub suspend_trade: bool,
    /// If true, account becomes long-only mode.
    pub no_shorting: bool,
}

#[derive(Clone, Debug)]
/// Returns the current account configuration values
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     account_configurations::{AccountConfigurations, GetAccountConfigurations},
///     paper_client,
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///    let client = paper_client("KEY", "SECRET");
///    let config: AccountConfigurations = client.send(&GetAccountConfigurations).await?;
///     Ok(())
/// }
/// ```
pub struct GetAccountConfigurations;

impl Request for GetAccountConfigurations {
    type Data = ();
    type Response = AccountConfigurations;

    fn endpoint(&self) -> Cow<str> {
        "/v2/account/configurations".into()
    }
}

#[derive(Clone, Debug, Default, Serialize)]
/// Updates the current account configuration values
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     account_configurations::{
///         AccountConfigurations, DtbpCheck, PatchAccountConfigurations, TradeConfirmEmail,
///     },
///     paper_client,
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let config: AccountConfigurations = client
///         .send(
///             &PatchAccountConfigurations::new()
///                 .dtbp_check(DtbpCheck::Entry)
///                 .trade_confirm_email(TradeConfirmEmail::All)
///                 .suspend_trade(false)
///                 .no_shorting(true),
///         )
///         .await?;
///     Ok(())
/// }
// TODO: The Alpaca docs here are wrong
pub struct PatchAccountConfigurations {
    dtbp_check: Option<DtbpCheck>,
    trade_confirm_email: Option<TradeConfirmEmail>,
    suspend_trade: Option<bool>,
    no_shorting: Option<bool>,
}

impl PatchAccountConfigurations {
    /// Create a new request
    pub fn new() -> Self {
        Self::default()
    }

    /// Controls Day Trading Margin Call (DTMC) checks.
    pub fn dtbp_check(mut self, dtbp_check: DtbpCheck) -> Self {
        self.dtbp_check = Some(dtbp_check);
        self
    }

    /// If `Zero`, emails for order fills are not sent.
    pub fn trade_confirm_email(mut self, trade_confirm_email: TradeConfirmEmail) -> Self {
        self.trade_confirm_email = Some(trade_confirm_email);
        self
    }

    /// If true, new orders are blocked.
    pub fn suspend_trade(mut self, suspend_trade: bool) -> Self {
        self.suspend_trade = Some(suspend_trade);
        self
    }

    /// If true, account becomes long-only mode.
    pub fn no_shorting(mut self, no_shorting: bool) -> Self {
        self.no_shorting = Some(no_shorting);
        self
    }
}

impl Request for PatchAccountConfigurations {
    type Data = Self;
    type Response = AccountConfigurations;
    const METHOD: Method = Method::PATCH;

    fn endpoint(&self) -> Cow<str> {
        "/v2/account/configurations".into()
    }

    fn data(&self) -> RequestData<&Self::Data> {
        RequestData::Json(&self)
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
            .with_body(r#"{
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
            .send(
                &PatchAccountConfigurations::new()
                    .dtbp_check(DtbpCheck::Entry)
                    .trade_confirm_email(TradeConfirmEmail::All)
                    .no_shorting(false)
                    .suspend_trade(false),
            )
            .await
            .unwrap();
    }
}
