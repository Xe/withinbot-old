use super::error::{Error, Result};
use reqwest::header::{HeaderMap, HeaderValue};

/// Forced cool down duration performed at every request. E621 allows at most 2 requests per second,
/// so the lowest safe value we can have here is 500 ms.
const REQ_COOLDOWN_DURATION: ::std::time::Duration = ::std::time::Duration::from_millis(600);

fn create_header_map<T: AsRef<[u8]>>(user_agent: T) -> Result<HeaderMap> {
    if user_agent.as_ref() == b"" {
        Err(Error::CannotCreateClient {
            desc: "User Agent mustn't be empty".into(),
        })
    } else {
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            HeaderValue::from_bytes(user_agent.as_ref())?,
        );

        Ok(headers)
    }
}

/// Client struct.
#[derive(Debug)]
pub struct Client {
    pub(crate) client: reqwest::blocking::Client,
    headers: HeaderMap,
}

impl Client {
    /// Create a new client with the specified value for the User-Agent header. The API requires a
    /// non-empty User-Agent header for all requests, preferably including your E621 username and
    /// the name of your project.
    pub fn new(user_agent: impl AsRef<[u8]>) -> Result<Self> {
        Ok(Client {
            client: reqwest::blocking::Client::new(),
            headers: create_header_map(user_agent)?,
        })
    }

    pub fn get_json_endpoint(&self, endpoint: &str) -> Result<serde_json::Value> {
        #[cfg(not(test))]
        let url = "https://e621.net";

        #[cfg(test)]
        let url = &mockito::server_url();

        // Wait first to make sure we're not exceeding the limit
        ::std::thread::sleep(REQ_COOLDOWN_DURATION);

        match self
            .client
            .get(&format!("{}{}", url, endpoint))
            .headers(self.headers.clone())
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json() {
                        Ok(v) => Ok(v),
                        Err(e) => Err(Error::Serial {
                            desc: format!("{}", e),
                        }),
                    }
                } else {
                    Err(Error::Http {
                        code: res.status().as_u16(),
                        reason: match res.json::<serde_json::Value>() {
                            Ok(v) => v["reason"].as_str().map(ToString::to_string),
                            Err(_) => None,
                        },
                    })
                }
            }

            Err(e) => Err(Error::CannotSendRequest {
                desc: format!("{}", e),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    fn get_json_endpoint_http_error() {
        let client = Client::new(b"xe621/unit_test").unwrap();

        let _m = mock("GET", "/post/show.json?id=8595")
            .with_status(500)
            .with_body(r#"{"success":false,"reason":"foo"}"#)
            .create();

        assert_eq!(
            client.get_json_endpoint("/post/show.json?id=8595"),
            Err(crate::error::Error::Http {
                code: 500,
                reason: Some(String::from("foo"))
            })
        );
    }

    #[test]
    fn get_json_endpoint_success() {
        let client = Client::new(b"xe621/unit_test").unwrap();

        let _m = mock("GET", "/post/show.json?id=8595")
            .with_body(r#"{"dummy":"json"}"#)
            .create();

        assert_eq!(
            client.get_json_endpoint("/post/show.json?id=8595"),
            Ok({
                let mut m = serde_json::Map::new();
                m.insert(String::from("dummy"), "json".into());
                m.into()
            })
        );
    }

    #[test]
    fn create_header_map_works() {
        assert!(create_header_map(b"xe621/unit_test").is_ok());
    }

    #[test]
    fn create_header_map_requires_valid_user_agent() {
        assert!(create_header_map(b"\n").is_err());
    }

    #[test]
    fn create_header_map_requires_non_empty_user_agent() {
        assert!(create_header_map(b"").is_err());
    }
}
