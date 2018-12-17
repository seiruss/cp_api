// cargo run --example install_policy

use cp_api::{Client, Error};
use serde_json::json;
use rpassword;
use std::process;
use std::io::{self, Write};

fn main() {
	println!("Rust Management API Install Policy Example\n");

	let mut client = build_client();

	if let Err(e) = login(&mut client) {
		eprintln!("Failed to login: {}", e);
		process::exit(1);
	}

	if let Err(e) = install(&mut client) {
		eprintln!("Failed to install-policy: {}", e);
		logout(&mut client).expect("Failed to logout");
		process::exit(1);
	}

	if let Err(e) = logout(&mut client) {
		eprintln!("Failed to logout: {}", e);
		process::exit(1);
	}

	if let Err(e) = client.save_log() {
		eprintln!("Failed to save log file: {}", e);
		process::exit(1);
	}
}

fn build_client() -> Client {
	let server = get_input("Enter server IP or name: ");

	let port = get_input("Enter server port: ");
	let port: u16 = port.parse().expect("Failed to convert port from String to u16");

	let mut client = Client::new(server.as_str(), port);

	// NOT RECOMMENDED
	// but setting this to true as this is an example
	client.accept_invalid_certs(true);

	client.log_file("install_policy.log");

	client
}

fn get_input(msg: &str) -> String {
	print!("{}", msg);
	io::stdout().flush().expect("Failed to flush stdout buffer");

	let mut s = String::new();
	io::stdin().read_line(&mut s).expect("Failed to read from stdin");

	s.trim().to_string()
}

fn login(client: &mut Client) -> Result<(), Error> {
	let user = get_input("Enter username: ");

	print!("Enter password (will not be shown on screen): ");
	io::stdout().flush().expect("Failed to flush stdout buffer");
	let pass = rpassword::read_password().expect("Failed to read password");

	println!("\n\nLogging into the API...\n");

	let login_res = client.login(user.as_str(), pass.as_str())?;

	if login_res.is_not_success() {
		let msg = format!("Failed to login: {}", login_res.data["message"]);
		return Err(Error::Custom(msg));
	}

	Ok(())
}

fn logout(client: &mut Client) -> Result<(), Error> {
	println!("Logging out...");

	let logout_res = client.logout()?;

	if logout_res.is_not_success() {
		let msg = format!("Failed to logout: {}", logout_res.data["message"]);
		return Err(Error::Custom(msg));
	}

	Ok(())
}

fn install(client: &mut Client) -> Result<(), Error> {
	println!("Going to install policy");

	let gateway = get_input("Enter Gateway/Cluster name: ");
	let policy = get_input("Enter Policy name: ");

	let payload = json!({
		"policy-package": policy,
		"access": true,
		"targets": gateway
	});

	println!("\nInstalling {} to {}\n", policy, gateway);

	let install_res = client.call("install-policy", payload)?;

	if install_res.is_not_success() {
		let msg = format!("Failed to run install-policy: {}", install_res.data["message"]);
		return Err(Error::Custom(msg));
	}

	Ok(())
}
