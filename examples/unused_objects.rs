// cargo run --example unused_objects

use cp_api::{Client, Error};
use rpassword;
use std::process;
use std::io::{self, Write};
use serde_json::json;

fn main() {
    println!("Rust Management API Unused Objects Example\n");

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
    unused_objects(&mut client)?;
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

    let domain = get_input("Enter Domain name (none for default): ")?;

    let mut client = Client::new(server.as_str(), port);

    // NOT RECOMMENDED
    // but setting this to true as this is an example
    client.accept_invalid_certs(true);

    client.read_only(true);

    if !domain.is_empty() {
        client.domain(domain.as_str());
    }

    client.log_file("unused_objects.log");

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

fn unused_objects(client: &mut Client) -> Result<(), Error> {
    let mut level = get_input("Enter details-level (none for default): ")?;
    if level.is_empty() {
        level = String::from("standard");
    }

    let mut limit = get_input("Enter limit (none for default): ")?;
    if limit.is_empty() {
        limit = String::from("50");
    }
    let limit: u16 = limit.parse()?;

    let mut offset = get_input("Enter offset (none for default): ")?;
    if offset.is_empty() {
        offset = String::from("0");
    }
    let offset: u32 = offset.parse()?;

    let unused_payload = json!({
        "details-level": level,
        "limit": limit,
        "offset": offset
    });

    if offset == 0 {
        println!("\nQuerying all unused objects...");
    }
    else {
        println!("\nQuerying unused objects starting at offset {}", offset);
    }

    let unused_res = client.query_and_check("show-unused-objects", unused_payload)?;

    // Uncomment to have objects printed
    /*for obj in &unused_res.objects {
        println!("{}", obj["name"]);
    }*/

    unused_res.save_objects("unused_objects.txt")?;

    println!("Done\n");

    Ok(())
}
