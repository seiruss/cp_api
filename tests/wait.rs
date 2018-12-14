use cp_api::Client;
use serde_json::json;

#[test]
fn install_policy() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    client.login("cp_api", "vpn123").unwrap();

    let payload = json!({
        "policy-package": "Standard",
        "access": true,
        "targets": "test-fw"
    });

    client.call("install-policy", payload).unwrap();
    client.logout().unwrap();
}

#[test]
fn no_wait() {
    let mut client = Client::new("172.25.199.80", 443);
    client.accept_invalid_certs(true);
    client.login("cp_api", "vpn123").unwrap();
    client.wait_for_task(false);

    let payload = json!({
        "policy-package": "Standard",
        "access": true,
        "targets": "test-fw"
    });

    let taskid = client.call("install-policy", payload).unwrap();
    println!("taskid = {}", taskid.data["task-id"]);
}
