use apca_rest::{calendar::GetCalendar, paper_client};
use std::env::var;

#[tokio::main]
async fn main() {
    let key = var("APCA_API_KEY_ID").unwrap();
    let secret = var("APCA_API_SECRET_KEY").unwrap();
    let client = paper_client(key, secret);
    let res = client.send(&GetCalendar::new()).await.unwrap();
    println!("{:#?}", res);
}
