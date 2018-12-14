// cargo run --example show_hosts

use cp_api::{Client, Error};
use std::process;
use std::io::{self, Write};

fn main() {
	println!("Rust Management API Show Hosts Example\n");

	let mut client = build_client();

	if let Err(e) = login(&mut client) {
		eprintln!("Failed to login: {}", e);
		process::exit(1);
	}

	if let Err(e) = show_hosts(&mut client) {
		eprintln!("Failed to show-hosts: {}", e);
		logout(&mut client).expect("Failed to logout");
		process::exit(1);
	}

	if let Err(e) = logout(&mut client) {
		eprintln!("Failed to logout: {}", e);
		process::exit(1);
	}

    client.save_log().expect("Failed to save log file");
}

fn build_client() -> Client {
	let server = get_input("Enter server IP or name: ");

	let port = get_input("Enter server port: ");
	let port: u16 = port.parse().expect("Failed to convert port from String to u16");

	let mut client = Client::new(server.as_str(), port);

	// NOT RECOMMENDED
	// but setting this to true as this is an example
	client.accept_invalid_certs(true);

    client.log_file("show_hosts.log");

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
		eprintln!("Failed to login: {}", login_res.data["message"]);
		process::exit(1);
	}

	Ok(())
}

fn logout(client: &mut Client) -> Result<(), Error> {
	println!("Logging out...");

	let logout_res = client.logout()?;

	if logout_res.is_not_success() {
		eprintln!("Failed to logout: {}", logout_res.data["message"]);
		process::exit(1);
	}

	Ok(())
}

fn show_hosts(client: &mut Client) -> Result<(), Error> {
	println!("Querying all hosts...");

	let hosts_res = client.query("show-hosts", "standard")?;

	if hosts_res.is_not_success() {
		eprintln!("Failed to run show-hosts: {}", hosts_res.data["message"]);
		process::exit(1);
	}

    for host in hosts_res.objects {
        println!("{} - {}", host["name"], host["ipv4-address"]);
    }

	Ok(())
}
