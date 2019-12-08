// cargo run --example create_domain

use cp_api::{Client, Error};
use serde_json::json;
use std::process;
use std::io;

fn main() {
    println!("Rust Management API Create API Domain\n");

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        enter_to_exit();
        process::exit(1);
    }

    println!("Completed :-)");
    enter_to_exit();
}

fn run() -> Result<(), Error> {
    let mut client = Client::new("10.1.1.110", 443);
    client.accept_invalid_certs(true);
    client.log_file("create_domain.log");

    login(&mut client)?;
    create(&mut client)?;
    logout(&mut client)?;
    client.save_log()?;

    Ok(())
}

fn enter_to_exit() {
    println!("\nPress [Enter] to exit");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read enter key");
}

fn login(client: &mut Client) -> Result<(), Error> {
    println!("Logging into the API...");

    let login_res = match client.login("admin", "vpn123") {
        Ok(t) => t,
        Err(e) => {
            let msg = format!("Failed to run 'login': {}", e);
            println!("{:#?}", e);
            println!("{:#?}", client);
            return Err(Error::Custom(msg));
        }
    };

    if login_res.is_not_success() {
        let msg = format!("'login' was not successful: {}", login_res.data["message"]);
        return Err(Error::Custom(msg));
    }

    Ok(())
}

fn logout(client: &mut Client) -> Result<(), Error> {
    println!("\nLogging out...");

    let logout_res = match client.logout() {
        Ok(t) => t,
        Err(e) => {
            let msg = format!("Failed to run 'logout': {}", e);
            return Err(Error::Custom(msg));
        }
    };

    if logout_res.is_not_success() {
        let msg = format!("'logout' was not successful: {}", logout_res.data["message"]);
        return Err(Error::Custom(msg));
    }

    Ok(())
}

fn create(client: &mut Client) -> Result<(), Error> {
    client.call_and_check("set-session", json!({"description": "R80 Multi-Domain Lab"}))?;

    let add = json!({
        "name": "API_Domain",
        "servers": {
            "ip-address": "10.1.1.113",
            "name": "API_CMA-1",
            "type": "management server",
            "multi-domain-server": "MDS-1",
        }
    });

    println!("\nCreating API_Domain with IP 10.1.1.113\n");

    client.call_and_check("add-domain", add)?;

    let any = json!({
        "name": "AnyHost",
        "domains-assignment": {
            "add": "CheckPoint",
            "add": "Microsoft",
            "add": "API_Domain"
        }
    });

    client.call_and_check("set-trusted-client", any)?;
    client.call_and_check("publish", json!({}))?;

    Ok(())
}
