//! # cp_api
//!
//! The `cp_api` crate provides a simple way to use the [Check Point Management API][ref].
//!
//! This handles the Web Services communcation by creating a Client to send
//! a Request and then receive a Response.
//!
//! ## Getting Started
//!
//! ```
//! // Build a Client.
//! let mut client = Client::new("192.168.1.10", 443);
//!
//! // Set a binary DER encoded certificate.
//! client.certificate("/home/admin/cert.cer");
//!
//! // Login to the API.
//! let login = client.login("user", "pass")?;
//! if login.is_not_success() {
//!     let msg = format!("Failed to login: {}", login.data["message"]);
//!     return Err(Error::Custom(msg));
//! }
//!
//! // Perform an API call to add a new host object.
//! let payload = json!({
//!     "name": "host1",
//!     "ip-address": "172.25.1.50"
//! });
//! let add_host = client.call("add-host", payload)?;
//! if add_host.is_not_success() {
//!     let msg = format!("Failed to add-host: {}", add_host.data["message"]);
//!     return Err(Error::Custom(msg));
//! }
//!
//! // Show that added host.
//! let host1 = client.call("show-host", json!({"name": "host1"}))?;
//! if host1.is_not_success() {
//!     let msg = format!("Failed to show-host: {}", host1.data["message"]);
//!     return Err(Error::Custom(msg));
//! }
//! println!("{} - {}", host1.data["name"], host1.data["ipv4-address"]);
//!
//! // Peform an API query to show all the host objects.
//! let all_hosts = client.query("show-hosts", "standard")?;
//! if all_hosts.is_not_success() {
//!     let msg = format!("Failed to show-hosts: {}", all_hosts.data["message"]);
//!     return Err(Error::Custom(msg));
//! }
//! for host in &all_hosts.objects {
//!     println!("{} - {}", host["name"], host["ipv4-address"]);
//! }
//!
//! // Logout of the API.
//! let logout = client.logout()?;
//! if logout.is_not_success() {
//!     let msg = format!("Failed to logout: {}", logout.data["message"]);
//!     return Err(Error::Custom(msg));
//! }
//! ```
//! [ref]: https://sc1.checkpoint.com/documents/latest/APIs/index.html

pub use crate::client::Client;
pub use crate::response::Response;
pub use crate::error::{Error, Result};

mod client;
mod response;
mod error;
