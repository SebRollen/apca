use apca_rest::{
    account_activities::{Activity, ActivityType, GetAccountActivitiesByType},
    paper_client,
};
use futures::StreamExt;
use std::env::var;
use stream_flatten_iters::TryStreamExt;

#[tokio::main]
async fn main() {
    let key = var("APCA_API_KEY_ID").unwrap();
    let secret = var("APCA_API_SECRET_KEY").unwrap();
    let client = paper_client(key, secret);
    let res: Vec<Activity> = client
        .send(&GetAccountActivitiesByType::new(ActivityType::Fill))
        .await
        .unwrap();
    println!("Non-paginated results: {}", res.len());
    let res: Vec<Activity> = client
        .send_paginated(&GetAccountActivitiesByType::new(ActivityType::Fill))
        .try_flatten_iters()
        .filter_map(|x| async move { x.ok() })
        .collect()
        .await;
    println!("Paginated results: {:#?}", res.len());
}
