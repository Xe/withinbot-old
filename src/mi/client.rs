use reqwest::blocking::Client;
use reqwest::header;
use serenity::prelude::*;
use std::{env, sync::{Arc, Mutex}};

// Name your user agent after your app?
pub static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " +https://github.com/Xe/withinbot",
);

pub fn make() -> Arc<Mutex<Client>> {
  let mut headers = header::HeaderMap::new();
  headers.insert(
    header::AUTHORIZATION,
    header::HeaderValue::from_str(
      env::var("MI_TOKEN")
        .expect("wanted MI_TOKEN in the environment")
        .as_str(),
    )
    .unwrap(),
  );

  let client = reqwest::blocking::Client::builder()
    .user_agent(APP_USER_AGENT)
    .default_headers(headers)
    .build();

  Arc::new(Mutex::new(client.unwrap()))
}

pub struct ClientContainer;

impl TypeMapKey for ClientContainer {
    type Value = Arc<Mutex<Client>>;
}
