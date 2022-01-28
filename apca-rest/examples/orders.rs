use apca_rest::{
    orders::{CancelOrder, OrderType, SubmitOrder},
    paper_client,
};
use rust_decimal::prelude::*;
use std::env::var;

#[tokio::main]
async fn main() {
    let key = var("APCA_API_KEY_ID").unwrap();
    let secret = var("APCA_API_SECRET_KEY").unwrap();
    let client = paper_client(key, secret);
    // WARNING: This example WILL issue an order to your paper account if run. The order sent is
    // deliberately sent with a very high limit price in the hopes that it will not be executed and
    // so can be cancelled right afterwards.
    let req = SubmitOrder::new("AAPL").order_type(OrderType::limit(Decimal::new(10000, 0)));
    let res = client.send(&req).await.unwrap();
    println!("{:#?}", res);

    let req = CancelOrder::new(res.id);
    let res = client.send(&req).await.unwrap();
    println!("{:#?}", res);
}
