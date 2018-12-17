//! # cp_api
//!
//! The `cp_api` crate provides a simple way to use the [Check Point Management API][ref].
//!
//! This handles the Web Services communcation by creating a Client to send
//! a Request and then receive a Response.
//!
//! ## Login
//!
//! An example logging in and logging out of the API.
//!
//! ```
//! use cp_api::{Client, Error};
//! use serde_json::json;
//!
//! fn example() -> Result<(), Error> {
//!     let mut client = Client::new("192.168.1.10", 443);
//!     client.certificate("/home/admin/cert.cer");
//!
//!     let login_response = client.login("user", "pass")?;
//!     if login_response.is_not_success() {
//!     let msg = format!("Failed to login: {}", login_response.data["message"]);
//!         return Err(Error::Custom(msg));
//!     }
//!
//!     println!("api-server-version: {}, sid: {}", client.api_server_version(), client.sid());
//!
//!     let logout_response = client.logout()?;
//!     if logout_response.is_not_success() {
//!         let msg = format!("Failed to logout: {}", logout_response.data["message"]);
//!         return Err(Error::Custom(msg));
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Install Policy
//!
//! An example installing policy to a Security Gateway.
//!
//! ```
//! use cp_api::{Client, Error};
//! use serde_json::json;
//!
//! fn install_policy(mut client: Client) -> Result<(), Error> {
//!     let payload = json!({
//!         "policy-package": "Standard",
//!         "access": true,
//!         "targets": "Gateway1"
//!     });
//!
//!     let install_response = client.call("install-policy", payload)?;
//!
//!     if install_response.is_not_success() {
//!         let msg = format!("Failed to install policy: {}", install_response.data["message"]);
//!         client.logout()?;
//!         return Err(Error::Custom(msg));
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Show hosts
//!
//! An example retrieving all host objects then print their names and IP addresses.
//!
//! ```
//! let hosts = client.query("show-hosts", "standard")?;
//!
//! if hosts_res.is_not_success() {
//!     let msg = format!("Failed to run show-hosts: {}", hosts_res.data["message"]);
//!     return Err(Error::Custom(msg));
//! }
//!
//! for host in hosts.objects {
//!     println!("{} - {}", host["name"], host["ipv4-address"]);
//! }
//! ```
//! [ref]: https://sc1.checkpoint.com/documents/latest/APIs/index.html

pub use crate::client::Client;
pub use crate::response::Response;
pub use crate::error::{Error, Result};

mod client;
mod response;
mod error;
