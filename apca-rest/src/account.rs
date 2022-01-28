use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use vila::Request;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// The following are the possible account status values. Most likely, the account status is `Active`
/// unless there is any problem. The account status may get in `AccountUpdated` when personal
/// information is being updated from the dashboard, in which case you may not be allowed trading
/// for a short period of time until the change is approved.
pub enum AccountStatus {
    /// The account is onboarding.
    Onboarding,
    /// The account application submission failed for some reason.
    SubmissionFailed,
    /// The account application has been submitted for review.
    Submitted,
    /// The account information is being updated.
    AccountUpdate,
    /// The final account approval is pending.
    ApprovalPending,
    /// The account is active for trading.
    Active,
    /// The account is active for trading.
    Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Account object
pub struct Account {
    /// Account ID.
    pub id: Uuid,
    /// Account number.
    pub account_number: String,
    /// See Account Status
    pub status: AccountStatus,
    /// "USD"
    pub currency: String,
    /// Cash balance
    pub cash: Decimal,
    /// Whether or not the account has been flagged as a pattern day trader
    pub pattern_day_trader: bool,
    /// User setting. If true, the account is not allowed to place orders.
    pub trade_suspended_by_user: bool,
    /// If true, the account is not allowed to place orders.
    pub trading_blocked: bool,
    /// If true, the account is not allowed to request money transfers.
    pub transfers_blocked: bool,
    /// If true, the account activity by user is prohibited.
    pub account_blocked: bool,
    /// Timestamp this account was created at
    pub created_at: DateTime<Utc>,
    /// Flag to denote whether or not the account is permitted to short
    pub shorting_enabled: bool,
    /// Real-time MtM value of all long positions held in the account
    pub long_market_value: Decimal,
    /// Real-time MtM value of all short positions held in the account
    pub short_market_value: Decimal,
    /// Cash + long_market_value + short_market_value
    pub equity: Decimal,
    /// Equity as of previous trading day at 16:00:00 ET
    pub last_equity: Decimal,
    /// Buying power multiplier that represents account margin classification; valid values 1
    /// (standard limited margin account with 1x buying power), 2 (reg T margin account with 2x
    /// intraday and overnight buying power; this is the default for all non-PDT accounts with
    /// $2,000 or more equity), 4 (PDT account with 4x intraday buying power and 2x reg T overnight
    /// buying power)
    pub multiplier: Decimal,
    /// Current available $ buying power; If multiplier = 4, this is your daytrade buying power
    /// which is calculated as (last_equity - (last) maintenance_margin) * 4; If multiplier = 2,
    /// buying_power = max(equity – initial_margin,0) * 2; If multiplier = 1, buying_power = cash
    pub buying_power: Decimal,
    /// Reg T initial margin requirement (continuously updated value)
    pub initial_margin: Decimal,
    /// Maintenance margin requirement (continuously updated value)
    pub maintenance_margin: Decimal,
    /// Value of special memorandum account (will be used at a later date to provide additional
    /// buying_power)
    pub sma: Decimal,
    /// The current number of daytrades that have been made in the last 5 trading days (inclusive
    /// of today)
    pub daytrade_count: u32,
    /// Your maintenance margin requirement on the previous trading day
    pub last_maintenance_margin: Decimal,
    /// Your buying power for day trades (continuously updated value)
    pub daytrading_buying_power: Decimal,
    /// Your buying power under Regulation T (your excess equity - equity minus margin value -
    /// times your margin multiplier)
    pub regt_buying_power: Decimal,
}

#[derive(Clone, Debug)]
/// Returns the account associated with the API key.
///
/// # Examples
/// ```no_run
/// use apca_rest::{
///     account::{Account, GetAccount},
///     paper_client,
/// };
/// #[tokio::main]
/// async fn main() -> Result<(), vila::Error> {
///     let client = paper_client("KEY", "SECRET");
///     let account: Account = client.send(&GetAccount).await?;
///     Ok(())
/// }
/// ```
pub struct GetAccount;

impl Request for GetAccount {
    type Data = ();
    type Response = Account;

    fn endpoint(&self) -> Cow<str> {
        "/v2/account".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_with_url;
    use mockito::mock;

    #[tokio::test]
    async fn get_account() {
        let _m = mock("GET", "/v2/account")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(ACCOUNT)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&GetAccount).await.unwrap();
    }

    const ACCOUNT: &'static str = r#"{
	    "account_blocked": false,
		"account_number": "010203ABCD",
		"buying_power": "262113.632",
		"cash": "-23140.2",
		"created_at": "2019-06-12T22:47:07.99658Z",
		"currency": "USD",
		"daytrade_count": 0,
		"daytrading_buying_power": "262113.632",
		"equity": "103820.56",
		"id": "e6fe16f3-64a4-4921-8928-cadf02f92f98",
		"initial_margin": "63480.38",
		"last_equity": "103529.24",
		"last_maintenance_margin": "38000.832",
		"long_market_value": "126960.76",
		"maintenance_margin": "38088.228",
		"multiplier": "4",
		"pattern_day_trader": false,
		"regt_buying_power": "80680.36",
		"short_market_value": "0",
		"shorting_enabled": true,
		"sma": "0",
		"status": "ACTIVE",
		"trade_suspended_by_user": false,
		"trading_blocked": false,
		"transfers_blocked": false
	}"#;
}
