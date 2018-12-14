use cp_api::Client;
use serde_json::json;

#[test]
fn save_calls_log() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    client.log_file("save_calls_log.txt");
    client.login("cp_api", "vpn123").unwrap();
    client.call("show-host", json!({"name": "host1"})).unwrap();
    client.logout().unwrap();
    client.save_log().unwrap();
}

#[test]
fn show_password_test() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    client.log_file("show_pass.txt");
    client.show_password(true);
    client.login("cp_api", "vpn123").unwrap();
    client.logout().unwrap();
    client.save_log().unwrap();
}
