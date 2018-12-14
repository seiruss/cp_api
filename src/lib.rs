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
//! use std::process;
//!
//! fn example() -> Result<(), Error> {
//!     let mut client = Client::new("192.168.1.10", 443);
//!     client.certificate("/home/admin/cert.cer");
//!
//!     let login_response = client.login("user", "pass")?;
//!     if login_response.is_not_success() {
//!         eprintln!("Failed to login: {}", login_response.data["message"]);
//!         process::exit(1);
//!     }
//!
//!     println!("api-server-version: {}, sid: {}", client.api_server_version(), client.sid());
//!
//!     let logout_response = client.logout()?;
//!     if logout_response.is_not_success() {
//!         eprintln!("Failed to logout: {}", logout_response.data["message"]);
//!         process::exit(1);
//!     }
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
//! use std::process;
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
//!         eprintln!("Failed to install policy: {}", install_response.data["message"]);
//!         client.logout()?;
//!         process::exit(1);
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