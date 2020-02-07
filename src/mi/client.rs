use lazy_static::lazy_static;
use reqwest::blocking::Client;
use reqwest::header;
use std::env;

// Name your user agent after your app?
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " +https://github.com/Xe/withinbot",
);

lazy_static! {
    pub static ref CLIENT: Client = {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                env::var("MI_TOKEN")
                    .expect("wanted MI_TOKEN in the environment")
                    .as_str(),
            ).unwrap(),
        );

        let client = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build();

        client.unwrap()
    };
}
