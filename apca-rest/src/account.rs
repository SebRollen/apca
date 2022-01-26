use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use vila::Request;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
    Onboarding,
    SubmissionFailed,
    Submitted,
    AccountUpdate,
    ApprovalPending,
    Active,
    Rejected,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    pub id: Uuid,
    pub account_number: String,
    pub status: AccountStatus,
    pub currency: String,
    pub cash: Decimal,
    pub portfolio_value: Decimal,
    pub pattern_day_trader: bool,
    pub trade_suspended_by_user: bool,
    pub trading_blocked: bool,
    pub transfers_blocked: bool,
    pub account_blocked: bool,
    pub created_at: DateTime<Utc>,
    pub shorting_enabled: bool,
    pub long_market_value: Decimal,
    pub short_market_value: Decimal,
    pub equity: Decimal,
    pub last_equity: Decimal,
    pub multiplier: Decimal,
    pub buying_power: Decimal,
    pub initial_margin: Decimal,
    pub maintenance_margin: Decimal,
    pub sma: Decimal,
    pub daytrade_count: u32,
    pub last_maintenance_margin: Decimal,
    pub daytrading_buying_power: Decimal,
    pub regt_buying_power: Decimal,
}

#[derive(Clone, Debug)]
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
    async fn test_get_account() {
        let _m = mock("GET", "/v2/account")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(
                r#"{
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
		            "portfolio_value": "103820.56",
		            "regt_buying_power": "80680.36",
		            "short_market_value": "0",
		            "shorting_enabled": true,
		            "sma": "0",
		            "status": "ACTIVE",
		            "trade_suspended_by_user": false,
		            "trading_blocked": false,
		            "transfers_blocked": false
		        }"#,
            )
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        client.send(&GetAccount).await.unwrap();
    }
}
