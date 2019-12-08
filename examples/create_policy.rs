// cargo run --example create_policy

use cp_api::{Client, Error};
use serde_json::json;
use std::process;
use std::io;

fn main() {
    println!("Rust Management API Create API Domain Policy\n");

    let mut client = Client::new("10.1.1.110", 443);
    client.domain("API_Domain");
    client.accept_invalid_certs(true);
    client.log_file("create_policy.log");

    if let Err(e) = run(&mut client) {
        eprintln!("Error: {}", e);
        client.call("discard", json!({})).expect("Failed to discard changes");
        enter_to_exit();
        process::exit(1);
    }

    println!("Completed :-)");
    enter_to_exit();
}

fn run(client: &mut Client) -> Result<(), Error> {
    login(client)?;

    client.call_and_check("set-session", json!({"description": "R80 Multi-Domain Lab"}))?;

    gw(client)?;

    println!("Publishing");
    client.call_and_check("publish", json!({}))?;

    objects(client)?;
    policy(client)?;
    layers(client)?;

    println!("Publishing");
    client.call_and_check("publish", json!({}))?;

    install(client)?;

    logout(client)?;
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

fn gw(client: &mut Client) -> Result<(), Error> {
    println!("\nAdding GW-4");

    // using separate variable to hold interfaces
    // if all in 'gw' variable, compiling serde_json fails
    // error: recursion limit reached while expanding the macro `json_internal`
    // help: consider adding a `#![recursion_limit="128"]` attribute to your crate
    let settings = json!([
        {
            "name": "eth0",
            "ipv4-address": "10.1.1.80",
            "ipv4-network-mask": "255.255.255.0",
            "anti-spoofing": true,
            "anti-spoofing-settings": {
                "action": "prevent"
            },
            "topology": "external",
        },
        {
            "name": "eth1",
            "ipv4-address": "192.168.1.80",
            "ipv4-network-mask": "255.255.255.0",
            "anti-spoofing": true,
            "anti-spoofing-settings": {
                "action": "prevent"
            },
            "topology": "internal",
            "topology-settings": {
                "ip-address-behind-this-interface": "network defined by the interface ip and net mask"
            },
        }
    ]);
    let gw = json!({
        "name": "GW-4",
        "color": "pink",
        "ipv4-address": "10.1.1.80",
        "version": "R80.20",
        "one-time-password": "vpn123",
        "firewall": true,
        "application-control": true,
        "url-filtering": true,
        "anti-bot": true,
        "anti-virus": true,
        "ips": true,
        "interfaces": settings
    });

    client.call_and_check("add-simple-gateway", gw)?;

    Ok(())
}

fn objects(client: &mut Client) -> Result<(), Error> {
    println!("\nCreating objects");

    client.call_and_check("add-host", json!({"name": "Win2008", "ip-address": "10.1.1.10"}))?;
    client.call_and_check("add-network", json!({"name": "Lab net", "subnet": "10.1.1.0", "subnet-mask": "255.255.255.0"}))?;
    client.call_and_check("add-network", json!({"name": "PC net", "subnet": "192.168.1.0", "subnet-mask": "255.255.255.0"}))?;
    client.call_and_check("add-network", json!({"name": "Guest net", "subnet": "172.25.1.0", "subnet-mask": "255.255.255.0"}))?;

    Ok(())
}

fn policy(client: &mut Client) -> Result<(), Error> {
    println!("\nCreating GW-4 policy package");

    client.call_and_check("add-package", json!({"name": "GW-4_Policy", "access": true, "threat-prevention": true}))?;
    client.call_and_check("set-package", json!({"name": "GW-4_Policy", "installation-targets": "GW-4"}))?;

    let rule1 = json!({
        "layer": "GW-4_Policy Network",
        "name": "Lab network to everywhere",
        "action": "accept",
        "position": "top",
        "source": ["Lab net", "PC net"],
        "track": {"type": "Log"}
    });
    client.call_and_check("add-access-rule", rule1)?;

    let rule2 = json!({
        "layer": "GW-4_Policy Network",
        "name": "MGMT and GW access",
        "action": "accept",
        "position": "top",
        "source": ["GW-4", "API_CMA-1"],
        "destination": ["GW-4", "API_CMA-1"],
        "track": {"type": "Log"}
    });
    client.call_and_check("add-access-rule", rule2)?;

    let rule3 = json!({
        "layer": "GW-4_Policy Network",
        "name": "Win2008 to everywhere",
        "action": "accept",
        "position": "top",
        "source": "Win2008",
        "track": {"type": "Log"}
    });
    client.call_and_check("add-access-rule", rule3)?;

    let rule4 = json!({
        "layer": "GW-4_Policy Network",
        "name": "Cleanup rule",
        "action": "drop",
        "track": {"type": "Log"}
    });
    client.call_and_check("set-access-rule", rule4)?;

    Ok(())
}

fn layers(client: &mut Client) -> Result<(), Error> {
    println!("\nCreating new layers");

    let layer1 = json!({
        "name": "Web Control Layer",
        "firewall": false,
        "applications-and-url-filtering": true,
        "shared": true
    });
    client.call_and_check("add-access-layer", layer1)?;

    let layer1_set = json!({
        "layer": "Web Control Layer",
        "name": "Cleanup rule",
        "action": "accept",
        "track": {"type": "Log"}
    });
    client.call_and_check("set-access-rule", layer1_set)?;

    let layer2 = json!({
        "name": "Guest Exception Layer",
        "firewall": false,
        "applications-and-url-filtering": true,
        "shared": true
    });
    client.call_and_check("add-access-layer", layer2)?;

    let layer2_set = json!({
        "layer": "Guest Exception Layer",
        "name": "Cleanup rule",
        "action": "accept",
        "track": {"type": "Log"}
    });
    client.call_and_check("set-access-rule", layer2_set)?;

    let rule1 = json!({
        "layer": "Web Control Layer",
        "position": "top",
        "name": "Block social media",
        "action": "drop",
        "track": {"type": "Log"},
        "destination": "Internet",
        "service": "Social Networking"
    });
    client.call_and_check("add-access-rule", rule1)?;

    let rule2 = json!({
        "layer": "Web Control Layer",
        "position": "top",
        "name": "Block Child Abuse",
        "action": "drop",
        "track": {"type": "Log"},
        "destination": "Internet",
        "service": "Child Abuse"
    });
    client.call_and_check("add-access-rule", rule2)?;

    let rule3 = json!({
        "layer": "Guest Exception Layer",
        "position": "top",
        "name": "Block bandwidth apps",
        "action": "drop",
        "track": {"type": "Log"},
        "source": "Guest net",
        "destination": "Internet",
        "service": ["Streaming Media Protocols", "P2P File Sharing"]
    });
    client.call_and_check("add-access-rule", rule3)?;

    let setpkg = json!({
        "name": "GW-4_Policy",
        "access-layers": {
            "add": [
                {
                    "name": "Web Control Layer",
                    "position": 2
                }, {
                    "name": "Guest Exception Layer",
                    "position": 3
                }
            ]
        }
    });
    client.call_and_check("set-package", setpkg)?;

    Ok(())
}

fn install(client: &mut Client) -> Result<(), Error> {
    println!("\nInstalling Access Control policy to GW-4");
    client.call_and_check("install-policy", json!({"policy-package": "GW-4_Policy", "access": true, "threat-prevention": false}))?;

    println!("\nInstalling Threat Prevention policy to GW-4");
    client.call_and_check("install-policy", json!({"policy-package": "GW-4_Policy", "access": false, "threat-prevention": true}))?;

    Ok(())
}
