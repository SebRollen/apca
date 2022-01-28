use apca_rest::{
    assets::{GetAsset, GetAssets},
    paper_client,
};
use std::env::var;

#[tokio::main]
async fn main() {
    let key = var("APCA_API_KEY_ID").unwrap();
    let secret = var("APCA_API_SECRET_KEY").unwrap();
    let client = paper_client(key, secret);
    let res = client.send(&GetAssets::new()).await.unwrap();
    println!("{:#?}", res);
    let res = client.send(&GetAsset::new("AAPL")).await.unwrap();
    println!("{:#?}", res);
    let res = client
        .send(&GetAsset::new("b0b6dd9d-8b9b-48a9-ba46-b9d54906e415"))
        .await
        .unwrap();
    println!("{:#?}", res);
}
