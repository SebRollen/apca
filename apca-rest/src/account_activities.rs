use crate::utils::*;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::CommaSeparator;
use std::borrow::Cow;
use uuid::Uuid;
use vila::{Request, RequestData};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FillType {
    Fill,
    PartialFill,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "sell_short")]
    SellShort,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Activity {
    TradeActivity {
        activity_type: ActivityType,
        cum_qty: Decimal,
        id: String,
        leaves_qty: Decimal,
        price: Decimal,
        qty: Decimal,
        side: Side,
        symbol: String,
        transaction_time: DateTime<Utc>,
        order_id: Uuid,
        #[serde(rename(serialize = "type", deserialize = "type"))]
        fill_type: FillType,
    },
    NonTradeActivity {
        activity_type: ActivityType,
        id: String,
        date: NaiveDate,
        net_amount: Decimal,
        symbol: Option<String>,
        #[serde(
            deserialize_with = "from_str_optional",
            serialize_with = "to_string_optional"
        )]
        qty: Option<i32>,
        per_share_amount: Option<Decimal>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ActivityType {
    #[serde(rename = "FILL")]
    Fill,
    #[serde(rename = "TRANS")]
    CashTransactions,
    #[serde(rename = "MISC")]
    Miscellaneous,
    #[serde(rename = "ACATC")]
    AcatsCash,
    #[serde(rename = "ACATS")]
    AcatsSecurities,
    #[serde(rename = "CSD")]
    CashDeposit,
    #[serde(rename = "CSW")]
    CashWithdrawal,
    #[serde(rename = "DIV")]
    Dividend,
    #[serde(rename = "DIVCGL")]
    DividendLongTermCapitalGain,
    #[serde(rename = "DIVCGS")]
    DividendShortTermCapitalGain,
    #[serde(rename = "DIVFEE")]
    DividendFee,
    #[serde(rename = "DIVFT")]
    DividendForeignTaxWithheld,
    #[serde(rename = "DIVNRA")]
    DividendNraWithheld,
    #[serde(rename = "DIVROC")]
    DividendReturnOfCapital,
    #[serde(rename = "DIVTXEX")]
    DividendTefraWithheld,
    #[serde(rename = "DIVTXEX")]
    DividendTaxExempt,
    #[serde(rename = "JNL")]
    JournalEntry,
    #[serde(rename = "JNLC")]
    JournalEntryCash,
    #[serde(rename = "JNLS")]
    JournalEntrySecurities,
    #[serde(rename = "MA")]
    MergerAcquisition,
    #[serde(rename = "NC")]
    NameChange,
    #[serde(rename = "OPASN")]
    OptionAssignment,
    #[serde(rename = "OPEXP")]
    OptionExpiration,
    #[serde(rename = "OPXRC")]
    OptionExercise,
    #[serde(rename = "PTC")]
    PassThroughCharge,
    #[serde(rename = "PTR")]
    PassThroughRebate,
    #[serde(rename = "REORG")]
    Reorgnization,
    #[serde(rename = "SC")]
    SymbolChange,
    #[serde(rename = "SSO")]
    StockSpinoff,
    #[serde(rename = "SSP")]
    StockSplit,
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = serde_plain::to_string(self).unwrap();
        formatter.write_str(&string)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetAccountActivities {
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        with = "serde_with::rust::StringWithSeparator::<CommaSeparator>"
    )]
    pub activity_type: Vec<ActivityType>,
}

impl GetAccountActivities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_activity(mut self, activity_type: ActivityType) -> Self {
        self.activity_type.push(activity_type);
        self
    }
}

impl Request for GetAccountActivities {
    type Data = Self;
    type Response = Vec<Activity>;

    fn endpoint(&self) -> Cow<str> {
        "/v2/account/activities".into()
    }

    fn data(&self) -> RequestData<&Self::Data> {
        RequestData::Query(&self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_with_url;
    use mockito::mock;

    #[tokio::test]
    async fn get_account_activities() {
        let both_activities = format!("[{},{}]", TRADE_ACTIVITY, NONTRADE_ACTIVITY);
        let _m = mock("GET", "/v2/account/activities")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .with_body(both_activities)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = GetAccountActivities::new();
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn get_only_fill_account_activities() {
        let fill_activities = format!("[{}]", TRADE_ACTIVITY);
        let _m = mock("GET", "/v2/account/activities")
            .match_header("apca-api-key-id", "APCA_API_KEY_ID")
            .match_header("apca-api-secret-key", "APCA_API_SECRET_KEY")
            .match_query("activity_type=FILL")
            .with_body(fill_activities)
            .create();
        let url = mockito::server_url();
        let client = client_with_url(&url, "APCA_API_KEY_ID", "APCA_API_SECRET_KEY");

        let req = GetAccountActivities::new().add_activity(ActivityType::Fill);
        client.send(&req).await.unwrap();
    }
    const TRADE_ACTIVITY: &'static str = r#"{
  		"activity_type": "FILL",
  		"cum_qty": "1",
  		"id": "20190524113406977::8efc7b9a-8b2b-4000-9955-d36e7db0df74",
  		"leaves_qty": "0",
  		"price": "1.63",
  		"qty": "1",
  		"side": "buy",
  		"symbol": "LPCN",
  		"transaction_time": "2019-05-24T15:34:06.977Z",
  		"order_id": "904837e3-3b76-47ec-b432-046db621571b",
  		"type": "fill"
	}"#;
    const NONTRADE_ACTIVITY: &'static str = r#"{
  		"activity_type": "DIV",
  		"id": "20190801011955195::5f596936-6f23-4cef-bdf1-3806aae57dbf",
  		"date": "2019-08-01",
  		"net_amount": "1.02",
  		"symbol": "T",
  		"qty": "2",
  		"per_share_amount": "0.51"
	}"#;
}
