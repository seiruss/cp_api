// cargo run --example discard_sessions

use cp_api::{Client, Error};
use serde_json::json;
use rpassword;
use std::process;
use std::io::{self, Write};

fn main() {
    println!("Rust Management API Discard WEB_API Sessions Example\n");

    let mut client = build_client();

    if let Err(e) = login(&mut client) {
        eprintln!("Failed to login: {}", e);
        process::exit(1);
    }

    if let Err(e) = discard_sessions(&mut client) {
        eprintln!("Failed to discard sessions: {}", e);
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

    client.log_file("discard_sessions.log");

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
        let msg = format!("{}", login_res.data["message"]);
        return Err(Error::Custom(msg));
    }

    Ok(())
}

fn logout(client: &mut Client) -> Result<(), Error> {
    println!("Logging out...");

    let logout_res = client.logout()?;

    if logout_res.is_not_success() {
        let msg = format!("{}", logout_res.data["message"]);
        return Err(Error::Custom(msg));
    }

    Ok(())
}

fn discard_sessions(client: &mut Client) -> Result<(), Error> {
    println!("Querying all sessions...");

    let sessions_res = client.query("show-sessions", "full")?;

    if sessions_res.is_not_success() {
        let msg = format!("{}", sessions_res.data["message"]);
        return Err(Error::Custom(msg));
    }

    for session in sessions_res.objects {
        // only discard web_api sessions
        if session["application"] != "WEB_API" {
            continue;
        }

        // ignore sessons with changes or locks
        if session["changes"] != 0 && session["locks"] != 0 {
            continue;
        }

        // skip discarding own session until the end
        if session["uid"] == client.uid() {
            continue;
        }

        let discard_res = client.call("discard", json!({"uid": session["uid"]}))?;
        if discard_res.is_success() {
            println!("Session {} discarded", session["uid"]);
        }
        else {
            println!("Failed to discard session {}", session["uid"]);
        }
    }

    let discard_my_sid = client.call("discard", json!({"uid": client.uid()}))?;
    if discard_my_sid.is_success() {
        println!("Discarded my own session");
    }
    else {
        println!("Failed to discard my own session");
    }

    Ok(())
}
