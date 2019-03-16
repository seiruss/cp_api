use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use serde_json::json;
use serde::Serialize;
use serde_derive::Serialize;

use crate::error::{Error, Result};

/// A Response from the API.
#[derive(Debug, Serialize)]
pub struct Response {
    status: u16,
    success: bool,
    url: String,
    headers: HashMap<String, String>,

    /// Contains the JSON value from the API after running a call.
    pub data: serde_json::Value,

    /// Contains the JSON value from the API after running a query.
    pub objects: Vec<serde_json::Value>,
}

impl Response {
    // Create a new Response.
    pub(crate) fn new() -> Response {
        Response {
            status: 200,
            success: true,
            url: String::with_capacity(50),
            headers: HashMap::new(),
            data: json!({}),
            objects: Vec::new(),
        }
    }

    // Set the values for a Response.
    pub(crate) fn set(reqwest_response: &mut reqwest::Response) -> Result<Response> {
        let mut res = Response::new();

        res.status = reqwest_response.status().as_u16();
        if res.is_success() {
            res.success = true;
        }
        else {
            res.success = false;
        }

        res.url = reqwest_response.url().to_string();

        res.data = match reqwest_response.json() {
            Ok(t) => t,
            Err(e) => {
                let msg = format!("Response body received is not valid JSON. \
                                   Error code: {}, message: {}", res.status(), e);
                return Err(Error::Custom(msg));
            }
        };

        let reqwest_headers = reqwest_response.headers();
        let mut map = HashMap::new();

        for (k, v) in reqwest_headers.iter() {
            let k = k.as_str().to_string();
            let v = v.to_str()?;
            let v = v.to_string();

            map.insert(k, v);
        }

        res.headers = map;

        Ok(res)
    }

    // Check if any tasks failed and if so, set the Response success field to false.
    pub(crate) fn check_tasks_status(res: &mut Response) {
        if res.data["tasks"].is_array() {
            for task in res.data["tasks"].as_array().unwrap() {
                if task["status"] == "failed" || task["status"] == "partially succeeded" {
                    res.success = false;
                }
            }
        }
    }

    /// Get the status of this Response.
    ///
    /// Reference: [IANA HTTP Status Codes][ref]
    ///
    /// ```
    /// let res = client.call("show-host", json!({"name": "host1"}))?;
    /// if res.is_success() {
    ///     println!("host1 IP = {}", res.data["ipv4-address"]);
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

    /// Check if the status is between 200-299 and is successful.
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300 && self.success == true
    }

    /// Check if the status is not successful.
    pub fn is_not_success(&self) -> bool {
        self.status < 200 || self.status >= 300 || self.success == false
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
    /// ```
    /// let res = client.call("show-host", json!({"name": "host1"}))?;
    /// assert_eq!(res.url() "https://192.168.1.10/web_api/show-host")
    /// ```
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Get the headers of this Response.
    ///
    /// ```
    /// let login = client.login("user", "pass")?;
    /// println!("{:#?}", login.headers());
    /// ```
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    /// Save data from a call to a file.
    ///
    /// ```
    /// let host = client.call("show-host", json!({"name": "host1"}))?;
    /// host.save_data("/home/admin/host.txt")?;
    /// ```
    pub fn save_data(&self, file: &str) -> Result<()> {
        let mut f = File::create(file)?;

        // Save with an indent of 4 spaces instead of 2 (the default)
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

        self.data.serialize(&mut ser)?;
        f.write_all(&ser.into_inner())?;

        Ok(())
    }

    /// Save objects from a query to a file.
    ///
    /// ```
    /// let hosts = client.query("show-hosts", "standard")?;
    /// hosts.save_objects("/home/admin/hosts.txt")?;
    /// ```
    pub fn save_objects(&self, file: &str) -> Result<()> {
        let mut f = File::create(file)?;

        // Save with an indent of 4 spaces instead of 2 (the default)
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

        self.objects.serialize(&mut ser)?;
        f.write_all(&ser.into_inner())?;

        Ok(())
    }
}
