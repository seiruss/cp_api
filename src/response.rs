use std::fmt;
use std::collections::HashMap;

use serde_json::json;
use serde::Serialize;
use serde_derive::Serialize;

use crate::error::{Error, Result};

/// A Response from the API.
#[derive(Debug, Serialize)]
pub struct Response {
    status: u16,
    url: String,
    headers: HashMap<String, String>,

    /// The Payload from the API after running a call.
    pub data: serde_json::Value,

    /// The Payload from the API after running a query.
    pub objects: Vec<serde_json::Value>,
}

impl Response {
    // Not for public use.
    // Use `pub(crate)` when it stabilizes.
    #[doc(hidden)]
    pub fn new() -> Response {
        Response {
            status: 200,
            url: String::with_capacity(50),
            headers: HashMap::new(),
            data: json!({}),
            objects: Vec::new(),
        }
    }

    // Not for public use.
    // Use `pub(crate)` when it stabilizes.
    #[doc(hidden)]
    pub fn set(reqwest_response: &mut reqwest::Response) -> Result<Response> {
        let mut res = Response::new();

        res.status = reqwest_response.status().as_u16();
        res.url = reqwest_response.url().to_string();
        res.data = reqwest_response.json().map_err(Error::Reqwest)?;

        let reqwest_headers = reqwest_response.headers();
        let mut map = HashMap::new();

        for (k, v) in reqwest_headers.iter() {
            let k = k.as_str().to_string();
            let v = v.to_str().map_err(Error::HeaderToStr)?;
            let v = v.to_string();

            map.insert(k, v);
        }

        res.headers = map;

        Ok(res)
    }

    /// Get the status of this Response.
    ///
    /// Reference: [IANA HTTP Status Codes][ref]
    ///
    /// # Example
    ///
    /// ```
    /// let res = client.call("show-host", json!({"name": "host1"}))?;
    /// if res.is_success() {
    ///     println!("host1 IP = {}", res.body["ipv4-address"]);
    /// }
    /// else if res.is_client_error() {
    ///     eprintln!("Client error");
    /// }
    /// else if res.is_server_error() {
    ///     eprintln!("Server error");
    /// }
    /// ```
    /// [ref]: https://www.iana.org/assignments/http-status-codes
    pub fn status(&self) -> u16 {
        self.status
    }

    /// Check if the status is between 100-199.
    pub fn is_informational(&self) -> bool {
        self.status >= 100 && self.status < 200
    }

    /// Check if the status is between 200-299.
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    /// Check if the status is not successful.
    pub fn is_not_success(&self) -> bool {
        self.status < 200 || self.status >= 300
    }

    /// Check if the status is between 300-399.
    pub fn is_redirection(&self) -> bool {
        self.status >= 300 && self.status < 400
    }

    /// Check if the status is between 400-499.
    pub fn is_client_error(&self) -> bool {
        self.status >= 400 && self.status < 500
    }

    /// Check if the status is between 500-599.
    pub fn is_server_error(&self) -> bool {
        self.status >= 500 && self.status < 600
    }

    /// Get the URL of this Response.
    ///
    /// # Example
    ///
    /// ```
    /// let res = client.call("show-host", json!({"name": "host1"}))?;
    /// assert_eq!(res.url() "https://192.168.1.10/web_api/show-host")
    /// ```
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Get the headers of this Response.
    ///
    /// # Example
    ///
    /// ```
    /// let login = c.login("user", "pass")?;
    /// println!("{:#?}", login.headers());
    /// ```
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

        self.objects.serialize(&mut ser).unwrap();
        let s = match String::from_utf8(ser.into_inner()) {
            Ok(t) => t,
            Err(_) => String::from("Error printing Response objects due to invalid UTF-8 bytes")
        };

        write!(f, "{}", s)
    }
}
