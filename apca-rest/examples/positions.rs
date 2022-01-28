use apca_rest::{
    paper_client,
    positions::{GetPosition, GetPositions},
};
use std::env::var;

#[tokio::main]
async fn main() {
    let key = var("APCA_API_KEY_ID").unwrap();
    let secret = var("APCA_API_SECRET_KEY").unwrap();
    let client = paper_client(key, secret);
    let res = client.send(&GetPositions).await.ok();
    println!("{:#?}", res);
    let res = client.send(&GetPosition::new("AAPL")).await.ok();
    println!("{:#?}", res);
}
