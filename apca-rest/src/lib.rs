use vila::Client;

pub mod account;
pub mod account_activities;
pub mod account_configurations;
pub mod assets;
pub mod calendar;
pub mod clock;
pub mod orders;
pub mod portfolio_history;
pub mod positions;
mod utils;
pub mod watchlists;

pub fn paper_client<T: AsRef<str>>(key: T, secret: T) -> Client {
    Client::new("https://paper-api.alpaca.markets").header_auth(vec![
        ("apca-api-key-id", key.as_ref()),
        ("apca-api-secret-key", secret.as_ref()),
    ])
}

pub fn live_client<T: AsRef<str>>(key: T, secret: T) -> Client {
    Client::new("https://api.alpaca.markets").header_auth(vec![
        ("apca-api-key-id", key.as_ref()),
        ("apca-api-secret-key", secret.as_ref()),
    ])
}

pub fn client_with_url(url: &str, key: &str, secret: &str) -> Client {
    Client::new(url).header_auth(vec![
        ("apca-api-key-id", key),
        ("apca-api-secret-key", secret),
    ])
}
