// cargo run --example discard_sessions

use cp_api::{Client, Error};
use serde_json::json;
use rpassword;
use std::process;
use std::io::{self, Write};

fn main() {
    println!("Rust Management API Discard WEB_API Sessions Example\n");

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let mut client = build_client()?;

    login(&mut client)?;
    discard_sessions(&mut client)?;
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

    client.log_file("discard_sessions.log");

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

fn discard_sessions(client: &mut Client) -> Result<(), Error> {
    println!("Querying all sessions...");

    let sessions_res = client.query("show-sessions", "full")?;

    if sessions_res.is_not_success() {
        let msg = format!("{}", sessions_res.data["message"]);
        logout(client)?;
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
