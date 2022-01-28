use apca_rest::{paper_client, portfolio_history::GetPortfolioHistory};
use std::env::var;

#[tokio::main]
async fn main() {
    let key = var("APCA_API_KEY_ID").unwrap();
    let secret = var("APCA_API_SECRET_KEY").unwrap();
    let client = paper_client(key, secret);
    let res = client.send(&GetPortfolioHistory::new()).await.ok();
    println!("{:#?}", res);
}
