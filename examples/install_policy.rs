// cargo run --example install_policy

use cp_api::{Client, Error};
use serde_json::json;
use rpassword;
use std::error::Error as StdError;
use std::process;
use std::io::{self, Write};

fn main() {
    println!("Rust Management API Install Policy Example\n");

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        eprintln!("Description: {}", e.description());
        if e.source().is_some() {
            eprintln!("Source: {}", e.source().unwrap());
        }
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let mut client = build_client()?;

    login(&mut client)?;
    install(&mut client)?;
    logout(&mut client)?;
    client.save_log()?;

    Ok(())
}

fn build_client() -> Result<Client, Error> {
    let server = get_input("Enter server IP or name: ")?;

    let port = get_input("Enter server port: ")?;
    let port: u16 = port.parse()?;

    let mut client = Client::new(server.as_str(), port);

    // NOT RECOMMENDED
    // but setting this to true as this is an example
    client.accept_invalid_certs(true);

    client.log_file("install_policy.log");

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

fn install(client: &mut Client) -> Result<(), Error> {
    println!("Going to install policy");

    let gateway = get_input("Enter Gateway/Cluster name: ")?;
    let policy = get_input("Enter Policy name: ")?;

    let payload = json!({
        "policy-package": policy,
        "access": true,
        "targets": gateway
    });

    println!("\nInstalling {} to {}\n", policy, gateway);

    let install_res = match client.call("install-policy", payload) {
        Ok(t) => t,
        Err(e) => {
            let msg = format!("Failed to run install-policy: {}", e);
            return Err(Error::Custom(msg));
        }
    };

    if install_res.is_not_success() {
        let msg = format!("Failed to run install-policy: {}", install_res.data["message"]);
        logout(client)?;
        return Err(Error::Custom(msg));
    }

    Ok(())
}
