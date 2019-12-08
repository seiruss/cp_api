// cargo run --example show_hosts

use cp_api::{Client, Error};
use rpassword;
use std::process;
use std::io::{self, Write};
use serde_json::json;

fn main() {
    println!("Rust Management API Show Hosts Example\n");

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        enter_to_exit();
        process::exit(1);
    }

    enter_to_exit();
}

fn run() -> Result<(), Error> {
    let mut client = build_client()?;

    login(&mut client)?;
    show_hosts(&mut client)?;
    logout(&mut client)?;
    client.save_log()?;

    Ok(())
}

fn enter_to_exit() {
    println!("\nPress [Enter] to exit");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read enter key");
}

fn build_client() -> Result<Client, Error> {
    let server = get_input("Enter server IP or name: ")?;

    let port = get_input("Enter server port: ")?;
    let port: u16 = port.parse()?;

    let mut client = Client::new(server.as_str(), port);

    // NOT RECOMMENDED
    // but setting this to true as this is an example
    client.accept_invalid_certs(true);

    client.read_only(true);

    client.log_file("show_hosts.log");

    Ok(client)
}

fn get_input(msg: &str) -> Result<String, Error> {
    print!("{}", msg);
    io::stdout().flush()?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    let s = s.trim().to_string();

    Ok(s)
}

fn login(client: &mut Client) -> Result<(), Error> {
    let user = get_input("Enter username: ")?;

    print!("Enter password (will not be shown on screen): ");
    io::stdout().flush()?;
    let pass = rpassword::read_password()?;

    println!("\n\nLogging into the API...\n");

    let login_res = match client.login(user.as_str(), pass.as_str()) {
        Ok(t) => t,
        Err(e) => {
            let msg = format!("Failed to run login: {}", e);
            return Err(Error::Custom(msg));
        }
    };

    if login_res.is_not_success() {
        let msg = format!("Failed to login: {}", login_res.data["message"]);
        return Err(Error::Custom(msg));
    }

    Ok(())
}

fn logout(client: &mut Client) -> Result<(), Error> {
    println!("Logging out...");

    let logout_res = match client.logout() {
        Ok(t) => t,
        Err(e) => {
            let msg = format!("Failed to run logout: {}", e);
            return Err(Error::Custom(msg));
        }
    };

    if logout_res.is_not_success() {
        let msg = format!("Failed to logout: {}", logout_res.data["message"]);
        return Err(Error::Custom(msg));
    }

    Ok(())
}

fn show_hosts(client: &mut Client) -> Result<(), Error> {
    println!("Querying all hosts...");

    let hosts_payload = json!({
        "details-level": "full",
        "limit": 500
    });

    let hosts_res = client.query("show-hosts", hosts_payload)?;

    for host in &hosts_res.objects {
        println!("{} - {}", host["name"], host["ipv4-address"]);
    }

    hosts_res.save_objects("hosts.log")?;

    Ok(())
}
