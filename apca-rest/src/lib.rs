use vila::Client;

mod account;
mod account_activities;
mod account_configurations;
mod assets;
mod calendar;
mod clock;
mod orders;
mod portfolio_history;
mod positions;
mod utils;
mod watchlists;
pub use account::*;
pub use account_activities::*;
pub use account_configurations::*;
pub use assets::*;
pub use calendar::*;
pub use clock::*;
pub use orders::*;
pub use portfolio_history::*;
pub use positions::*;
pub use watchlists::*;

pub fn paper_client(key: &str, secret: &str) -> Client {
    Client::new("https://paper-api.alpaca.markets").header_auth(vec![
        ("apca-api-key-id", key),
        ("apca-api-secret-key", secret),
    ])
}

pub fn live_client(key: &str, secret: &str) -> Client {
    Client::new("https://api.alpaca.markets").header_auth(vec![
        ("apca-api-key-id", key),
        ("apca-api-secret-key", secret),
    ])
}

pub fn client_with_url(url: &str, key: &str, secret: &str) -> Client {
    Client::new(url).header_auth(vec![
        ("apca-api-key-id", key),
        ("apca-api-secret-key", secret),
    ])
}
