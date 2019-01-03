use std::{fmt, time, thread};
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::header::{ACCEPT, CONTENT_TYPE, USER_AGENT};

use serde_json::json;
use serde::Serialize;
use serde_derive::Serialize;

use crate::response::Response;
use crate::error::{Error, Result};

/// A Client to communicate with the API.
///
/// There are various configuration values to set, but the defaults
/// are the most commonly used.
///
/// The Client should be created and reused for multiple API calls.
///
/// # Example
///
/// ```
/// use cp_api::Client;
/// use serde_json::json;
///
/// let mut client = Client::new("192.168.1.10", 443);
/// client.certificate("/home/admin/cert.cer");
/// client.login("user", "pass")?;
/// client.call("show-host", json!({"name": "host1"}))?;
/// client.call("show-package", json!({"name": "Standard"}))?;
/// client.logout()?;
/// ```
#[derive(Serialize)]
pub struct Client {
    server: String,
    port: u16,
    certificate: String,
    accept_invalid_certs: bool,
    proxy: String,
    connect_timeout: time::Duration,
    session_timeout: u64,
    domain: String,
    sid: String,
    uid: String,
    api_server_version: String,
    wait_for_task: bool,
    log_file: String,
    all_calls: Vec<serde_json::Value>,
    show_password: bool,
}

impl Client {
    /// Create a new Client to make API calls.
    /// ```
    /// let mut client = Client::new("192.168.1.10", 443);
    /// ```
    pub fn new(server: &str, port: u16) -> Self {
        Client {
            server: String::from(server),
            port,
            certificate: String::new(),
            accept_invalid_certs: false,
            proxy: String::new(),
            connect_timeout: time::Duration::from_secs(30),
            session_timeout: 600,
            domain: String::new(),
            sid: String::with_capacity(50),
            uid: String::with_capacity(40),
            api_server_version: String::with_capacity(5),
            wait_for_task: true,
            log_file: String::new(),
            all_calls: Vec::new(),
            show_password: false,
        }
    }

    /// Login to the API.
    ///
    /// If the login is successful, the sid, uid, and api-server-version are stored in the Client.
    ///
    /// # Example
    ///
    /// ```
    /// let mut client = Client::new("192.168.1.10", 443);
    /// client.certificate("/home/admin/cert.cer");
    /// let login = client.login("user", "pass")?;
    /// assert!(login.is_success());
    /// assert!(!client.sid().is_empty());
    /// assert!(!client.uid().is_empty());
    /// assert!(!client.api_server_version().is_empty());
    /// ```
    pub fn login(&mut self, user: &str, pass: &str) -> Result<Response> {
        let payload = json!({
            "user": user,
            "password": pass,
            "domain": self.domain,
            "session-timeout": self.session_timeout,
        });

        let login = self.call("login", payload)?;

        if login.is_success() {
            self.sid = match login.data["sid"].as_str() {
                Some(t) => t,
                None => return Err(Error::InvalidResponse("sid", json!(login)))
            }.to_string();

            self.uid = match login.data["uid"].as_str() {
                Some(t) => t,
                None => return Err(Error::InvalidResponse("uid", json!(login)))
            }.to_string();

            self.api_server_version = match login.data["api-server-version"].as_str() {
                Some(t) => t,
                None => return Err(Error::InvalidResponse("api-server-version", json!(login)))
            }.to_string();
        }

        Ok(login)
    }

    /// Logout of the API.
    ///
    /// If the logout was successful, the sid, uid, and api-server-version are cleared
    /// from the Client.
    ///
    /// # Example
    ///
    /// ```
    /// let logout = client.logout()?;
    /// assert!(logout.is_success());
    /// assert!(client.sid().is_empty());
    /// assert!(client.uid().is_empty());
    /// assert!(client.api_server_version().is_empty());
    /// ```
    pub fn logout(&mut self) -> Result<Response> {
        let logout = self.call("logout", json!({}))?;

        if logout.is_success() {
            self.sid.clear();
            self.uid.clear();
            self.api_server_version.clear();
        }

        Ok(logout)
    }

    /// Perform an API call.
    ///
    /// # Examples
    ///
    /// ```
    /// let host_payload = json!({
    ///     "name": "host1",
    ///     "ip-address": "172.25.1.50"
    /// });
    ///
    /// let host = client.call("add-host", host_payload)?;
    /// assert!(host.is_success());
    ///
    /// let rule_payload = json!({
    ///     "name": "allow host1",
    ///     "layer": "Network",
    ///     "position": "top",
    ///     "source": "host1",
    ///     "action": "accept"
    /// });
    ///
    /// let rule = client.call("add-access-rule", rule_payload)?;
    /// assert!(rule.is_success());
    ///
    /// let publish = client.call("publish", json!({}))?;
    /// assert!(publish.is_success());
    /// ```
    pub fn call(&mut self, command: &str, payload: serde_json::Value) -> Result<Response> {
        let url = format!("https://{}:{}/web_api/{}", self.server, self.port, command);
        let headers = self.headers()?;
        let headers2 = headers.clone();

        let reqwest_client = self.build_client(headers)?;

        let mut reqwest_response = reqwest_client.post(url.as_str())
            .json(&payload)
            .send()?;

        let mut res = Response::set(&mut reqwest_response)?;

        if res.data.get("task-id").is_some() && self.wait_for_task == true {
            res = self._wait_for_task(res.data["task-id"].as_str().unwrap(), command)?;
        }

        if !self.log_file.is_empty() {
            self.update_calls(command, url.as_str(), headers2, payload, &res)?;
        }

        Ok(res)
    }

    // Generate the headers for a Request
    fn headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("cp_api"));

        if !self.sid.is_empty() {
            let n = HeaderName::from_static("x-chkp-sid");
            let v = HeaderValue::from_str(self.sid.as_str())?;

            headers.insert(n, v);
        }

        Ok(headers)
    }

    // Build the reqwest client
    fn build_client(&self, headers: HeaderMap) -> Result<reqwest::Client> {
        let mut builder = reqwest::ClientBuilder::new();
        builder = builder.default_headers(headers);
        builder = builder.timeout(self.connect_timeout);

        if !self.proxy.is_empty() {
            builder = builder.proxy(reqwest::Proxy::https(self.proxy.as_str())?);
        }

        if self.accept_invalid_certs == true && self.certificate.is_empty() {
            builder = builder.danger_accept_invalid_certs(true);
        }

        if !self.certificate.is_empty() {
            let mut buf: Vec<u8> = Vec::new();
            File::open(self.certificate.as_str())?
                .read_to_end(&mut buf)?;

            let cert = reqwest::Certificate::from_der(&buf)?;
            builder = builder.add_root_certificate(cert);
            builder = builder.danger_accept_invalid_certs(false);
        }

        let client = builder.build()?;

        Ok(client)
    }

    /// Perform an API query.
    ///
    /// All commands that return a list of objects can take a details-level parameter.
    /// The possible options are "standard", "full", and "uid".
    ///
    /// A vector of all the objects will be stored in the Response objects field.
    ///
    /// # Example
    ///
    /// ```
    /// let hosts = client.query("show-hosts", "standard")?;
    /// assert!(hosts.is_success());
    ///
    /// for host in hosts.objects {
    ///     println!("{} - {}", host["name"], host["ipv4-address"]);
    /// }
    /// ```
    pub fn query(&mut self, command: &str, details_level: &str) -> Result<Response> {
        let mut res = Response::new();
        let mut vec: Vec<serde_json::Value> = Vec::new();

        let limit = 50;
        let mut offset = 0;
        let mut to = 0;
        let mut total = 1;

        while to != total {
            let payload = json!({
                "details-level": details_level,
                "limit": limit,
                "offset": offset
            });

            res = self.call(command, payload)?;

            if res.is_not_success() {
                let msg = format!("Received an unsuccessful Response from the API \
                                   while running a query. {}, {}",
                                   res.data["code"], res.data["message"]);
                return Err(Error::Custom(msg));
            }

            to = match res.data["to"].as_u64() {
                Some(t) => t,
                None => return Err(Error::InvalidResponse("to", json!(res)))
            };

            total = match res.data["total"].as_u64() {
                Some(t) => t,
                None => return Err(Error::InvalidResponse("total", json!(res)))
            };

            let mut objects = match res.data["objects"].as_array_mut() {
                Some(t) => t,
                None => return Err(Error::InvalidResponse("objects", json!(res)))
            };

            vec.append(&mut objects);

            offset += 50;
        }

        res.objects = vec;
        res.data = json!({});

        Ok(res)
    }

    /// Set a binary DER encoded certificate.
    ///
    /// If a certificate is set, accept_invalid_certs will be ignored.
    ///
    /// ```
    /// client.certificate("/home/admin/mycert.cer");
    /// ```
    pub fn certificate(&mut self, s: &str) {
        self.certificate = s.to_string();
    }

    /// Set the certificate validation.
    ///
    /// The default is false to not accept invalid certificates.
    /// ```
    /// client.accept_invalid_cert(true);
    /// ```
    pub fn accept_invalid_certs(&mut self, b: bool) {
        self.accept_invalid_certs = b;
    }

    /// Set the proxy to use.
    ///
    /// This will proxy all HTTPS traffic to the URL.
    /// ```
    /// client.proxy("https://10.1.1.100:8080");
    /// ```
    pub fn proxy(&mut self, s: &str) {
        self.proxy = s.to_string();
    }

    /// Set the connection timeout in seconds to the Management server. Default is 30 seconds.
    /// ```
    /// client.connect_timeout(10);
    /// ```
    pub fn connect_timeout(&mut self, t: u64) {
        self.connect_timeout = time::Duration::from_secs(t);
    }

    /// Set the login session-timeout in seconds. Default is 600 seconds.
    /// ```
    /// client.session_timeout(1200);
    /// ```
    pub fn session_timeout(&mut self, t: u64) {
        self.session_timeout = t;
    }

    /// Set the Domain to login to.
    /// ```
    /// client.domain("System Data");
    /// ```
    pub fn domain(&mut self, s: &str) {
        self.domain = s.to_string();
    }

    /// Get the sid after logging in.
    /// ```
    /// client.login("user", "pass")?;
    /// println!("{}", client.sid());
    /// ```
    pub fn sid(&self) -> &str {
        self.sid.as_str()
    }

    /// Get the uid after logging in.
    /// ```
    /// client.login("user", "pass")?;
    /// println!("{}", client.uid());
    /// ```
    pub fn uid(&self) -> &str {
        self.uid.as_str()
    }

    /// Get the api-server-version after logging in.
    /// ```
    /// client.login("user", "pass")?;
    /// println!("{}", client.api_server_version());
    /// ```
    pub fn api_server_version(&self) -> &str {
        self.api_server_version.as_str()
    }

    /// Wait for an API call to complete.
    ///
    /// Some API commands return a task-id while they continue to run.
    /// The default is to wait for the task to finish.
    ///
    /// Set this to false to not wait for the task to complete.
    /// ```
    /// client.wait_for_task(false);
    ///
    /// let payload = json!({
    ///     "policy-package": "Standard",
    ///     "access": true,
    ///     "targets": "Gateway1"
    /// });
    ///
    /// let response = client.call("install-policy", payload)?;
    /// println!("task-id = {}", response.data["task-id"]);
    /// ```
    pub fn wait_for_task(&mut self, b: bool) {
        self.wait_for_task = b;
    }

    // Wait for a task to complete that returned a task-id.
    fn _wait_for_task(&mut self, taskid: &str, command: &str) -> Result<Response> {
        let mut _res = Response::new();

        loop {
            _res = self.call("show-task", json!({"task-id": taskid, "details-level": "full"}))?;

            let percent = match _res.data["tasks"][0].get("progress-percentage") {
                Some(t) => t,
                None => return Err(Error::InvalidResponse("progress-percentage", json!(_res)))
            };

            let status = match _res.data["tasks"][0].get("status") {
                Some(t) => t,
                None => return Err(Error::InvalidResponse("status", json!(_res)))
            };

            println!("{} {} - {}%", command, status, percent);

            if status != "in progress" {
                break;
            }

            thread::sleep(time::Duration::from_secs(5));
        }

        Ok(_res)
    }

    /// Set the log file name that will contain the API calls.
    ///
    /// The path to the file can be absolute or relative.
    /// Separate the path with `/` on Linux and either `/` or `\\` on Windows.
    /// ```
    /// client.log_file("C:\\Users\\admin\\Desktop\\log.txt");
    ///
    /// client.log_file("/home/admin/log.txt");
    /// ```
    pub fn log_file(&mut self, s: &str) {
        self.log_file = s.to_string();
    }

    // Update the vector of API calls
    fn update_calls(
        &mut self,
        command: &str,
        url: &str,
        headers: HeaderMap,
        mut payload: serde_json::Value,
        res: &Response,
        ) -> Result<()>
    {
        if command == "login" && self.show_password == false {
            if let Some(obj) = payload.get_mut("password") {
                *obj = json!("*****");
            }
            else {
                let msg = String::from("Failed to get the password to obfuscate from payload");
                return Err(Error::Custom(msg));
            }
        }

        let mut map = HashMap::new();

        for (k, v) in headers.iter() {
            let k = k.as_str().to_string();
            let v = v.to_str()?;
            let v = v.to_string();

            map.insert(k, v);
        }

        let j = json!({
            "Request": {
                "headers": map,
                "payload": payload,
                "url": url
            },
            "Response": res
        });

        let mut v = vec!(j);
        self.all_calls.append(&mut v);

        Ok(())
    }

    /// Save the API calls to a file.
    ///
    /// API calls made before log_file was set will not be saved.
    ///
    /// After the API calls are saved, the API calls and log_file will be cleared.
    /// ```
    /// client.log_file("/home/admin/log.txt");
    /// client.call("show-host", json!({"name": "host1"}))?;
    /// client.save_log()?;
    /// ```
    pub fn save_log(&mut self) -> Result<()> {
        if self.log_file.is_empty() {
            let msg = String::from("log_file on the Client is not set");
            return Err(Error::Custom(msg));
        }

        let mut f = File::create(self.log_file.as_str())?;

        // Save all_calls with an indent of 4 spaces instead of 2 (the default)
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);

        self.all_calls.serialize(&mut ser)?;
        f.write_all(&ser.into_inner())?;

        self.all_calls.clear();
        self.log_file.clear();

        Ok(())
    }

    /// Show the login password as clear text in the log file.
    ///
    /// This must be set before logging in or the password will be obfuscated.
    /// ```
    /// client.log_file("/home/admin/log.txt");
    /// client.show_password(true);
    /// client.login("user", "pass")?;
    /// client.save_log()?;
    /// ```
    pub fn show_password(&mut self, b: bool) {
        self.show_password = b;
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if !self.sid.is_empty() {
            if let Err(e) = self.logout() {
                // A panic isn't ideal since the Client is being dropped and can't be used anymore.
                // Printing an error message isn't ideal either.
                // Best to always call logout and handle any errors manually.
                eprintln!("Error logging out while dropping the Client: {}", e);
            }
        }
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("server", &self.server)
            .field("port", &self.port)
            .field("certificate", &self.certificate)
            .field("accept_invalid_certs", &self.accept_invalid_certs)
            .field("proxy", &self.proxy)
            .field("connect_timeout", &self.connect_timeout)
            .field("session_timeout", &self.session_timeout)
            .field("domain", &self.domain)
            .field("sid", &self.sid)
            .field("uid", &self.uid)
            .field("api_server_version", &self.api_server_version)
            .field("wait_for_task", &self.wait_for_task)
            .field("log_file", &self.log_file)
            .field("show_password", &self.show_password)
            .finish()
    }
}
