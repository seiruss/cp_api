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

#[test]
fn tasks() {
    let mut client = Client::new("10.1.1.110", 443);
    client.accept_invalid_certs(true);
    client.domain("CheckPoint");
    client.login("admin", "vpn123").unwrap();

    let payload = json!({
        "script-name": "example",
        "script": "ls -l /",
        "targets": "GW-2"
    });

    let res = client.call("run-script", payload).unwrap();

    for task in res.data["tasks"].as_array().unwrap() {
        println!("task name: {}, status: {}, statusDescription: {}",
                task["task-name"], task["status"], task["task-details"][0]["statusDescription"]);
    }

    assert_eq!(200, res.status());
    assert!(!res.is_success());
    assert!(res.is_not_success());

    client.logout().unwrap();
}
