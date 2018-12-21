use cp_api::Client;
use serde_json::json;

#[test]
fn login_logout() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    let login = client.login("cp_api", "vpn123").unwrap();
    assert!(login.is_success());
    assert!(!client.sid().is_empty());
    assert!(!client.uid().is_empty());
    assert!(!client.api_server_version().is_empty());

    client.logout().unwrap();
    assert!(client.sid().is_empty());
    assert!(client.uid().is_empty());
    assert!(client.api_server_version().is_empty());
}

#[test]
fn cert_and_invalid() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    client.certificate("wrong_cert.cer");

    // fail to connect since using wrong certificate
    client.login("cp_api", "vpn123").unwrap();
}

#[test]
fn proxy_test() {
    let mut client = Client::new("172.25.199.80", 443);
    client.proxy("https://192.168.1.12:8080"); // raspberry pi
    client.login("cp_api", "vpn123").unwrap();
}

#[test]
fn system_data() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    client.domain("System Data");
    client.login("cp_api", "vpn123").unwrap();

    let res = client.call("show-administrator", json!({"name": "cp_api"})).unwrap();
    println!("name = {}, type = {}", res.data["name"], res.data["type"]);

    client.call("logout", json!({})).unwrap();
}

#[test]
fn logout_then_call() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    client.login("cp_api", "vpn123").unwrap();
    client.logout().unwrap();

    assert!(client.sid().is_empty());

    let res = client.call("show-host", json!({"name": "host1"})).unwrap();
    if res.is_not_success() {
        println!("Test passed: {}, {}", res.status(), res.data["message"]);
    }
}
